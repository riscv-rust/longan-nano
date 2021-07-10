#![no_std]
#![no_main]

use panic_halt as _;

use core::fmt::Write;
use gd32vf103xx_hal::{
    pac::USART0,
    serial::{self, Config, Parity, Rx, StopBits, Tx},
};
use longan_nano::{
    hal::{pac, prelude::*},
    led::{rgb, Led, BLUE, GREEN, RED},
};
use riscv_rt::entry;
use ushell::{autocomplete::*, history::*, *};

const MAX_COMMAND_LEN: usize = 16;
const HISTORY_SIZE: usize = 4;
const COMMANDS: usize = 5;

const SHELL_PROMPT: &str = "#> ";
const CR: &str = "\r\n";
const HELP: &str = "\r\n\
\x1b[31mL\x1b[32mE\x1b[34mD\x1b[33m Shell\x1b[0m\r\n\r\n\
USAGE:\r\n\
\tcommand [arg]\r\n\r\n\
COMMANDS:\r\n\
\ton  <ch>  Switch led channel on [r,g,b,a]\r\n\
\toff <ch>  Switch led channel off [r,g,b,a]\r\n\
\tstatus    Get leds status\r\n\
\tclear     Clear screen\r\n\
\thelp      Print this message\r\n
";

struct Context {
    red: RED,
    green: GREEN,
    blue: BLUE,
    shell: UShell<
        ushell::Serial<u8, Tx<USART0>, Rx<USART0>>,
        StaticAutocomplete<{ COMMANDS }>,
        LRUHistory<{ MAX_COMMAND_LEN }, { HISTORY_SIZE }>,
        { MAX_COMMAND_LEN },
    >,
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let mut afio = dp.AFIO.constrain(&mut rcu);

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);

    let tx = gpioa.pa9.into_alternate_push_pull();
    let rx = gpioa.pa10.into_floating_input();

    let config = Config {
        baudrate: 115_200.bps(),
        parity: Parity::ParityNone,
        stopbits: StopBits::STOP1,
    };
    let uart = serial::Serial::new(dp.USART0, (tx, rx), config, &mut afio, &mut rcu);
    let (tx, rx) = uart.split();

    let autocomplete = StaticAutocomplete(["clear", "help", "status", "off ", "on "]);
    let history = LRUHistory::default();
    let shell = UShell::new(ushell::Serial::from_parts(tx, rx), autocomplete, history);

    let (mut red, mut green, mut blue) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);
    red.off();
    green.off();
    blue.off();

    let mut ctx = Context {
        shell,
        red,
        green,
        blue,
    };

    loop {
        poll_serial(&mut ctx);
    }
}

fn poll_serial(ctx: &mut Context) {
    match ctx.shell.poll() {
        Ok(Some(Input::Command((cmd, args)))) => {
            match cmd {
                "help" => {
                    ctx.shell.write_str(HELP).ok();
                }
                "clear" => {
                    ctx.shell.clear().ok();
                }
                "status" => {
                    let red = if ctx.red.is_on() { "On" } else { "Off" };
                    let green = if ctx.green.is_on() { "On" } else { "Off" };
                    let blue = if ctx.blue.is_on() { "On" } else { "Off" };
                    write!(
                        ctx.shell,
                        "{0:}Red: {1:}{0:}Green: {2:}{0:}Blue: {3:}{0:}",
                        CR, red, green, blue,
                    )
                    .ok();
                }
                "on" => {
                    match args {
                        "r" | "red" => ctx.red.on(),
                        "g" | "green" => ctx.green.on(),
                        "b" | "blue" => ctx.blue.on(),
                        "a" | "all" => {
                            ctx.red.on();
                            ctx.green.on();
                            ctx.blue.on();
                        }
                        _ => {
                            write!(ctx.shell, "{0:}unsupported color channel", CR).ok();
                        }
                    }
                    ctx.shell.write_str(CR).ok();
                }
                "off" => {
                    match args {
                        "r" | "red" => ctx.red.off(),
                        "g" | "green" => ctx.green.off(),
                        "b" | "blue" => ctx.blue.off(),
                        "a" | "all" => {
                            ctx.red.off();
                            ctx.green.off();
                            ctx.blue.off();
                        }
                        _ => {
                            write!(ctx.shell, "{0:}unsupported color channel", CR).ok();
                        }
                    }
                    ctx.shell.write_str(CR).ok();
                }
                "" => {
                    ctx.shell.write_str(CR).ok();
                }
                _ => {
                    write!(ctx.shell, "{0:}unsupported command{0:}", CR).ok();
                }
            }
            ctx.shell.write_str(SHELL_PROMPT).ok();
        }
        _ => {}
    }
}
