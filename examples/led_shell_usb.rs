#![no_std]
#![no_main]

use panic_halt as _;

use core::fmt::Write;
use longan_nano::{
    hal::{pac, prelude::*},
    led::{rgb, Led, BLUE, GREEN, RED},
    usb::{usb, UsbBusType},
};
use riscv_rt::entry;
use ushell::{autocomplete::*, history::*, *};
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(96.mhz())
        .freeze();

    assert!(rcu.clocks.usbclk_valid());

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);

    let (mut red, mut green, mut blue) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);
    red.off();
    green.off();
    blue.off();

    static mut EP_MEMORY: [u32; 1024] = [0; 1024];

    let usb_bus = usb(
        dp.USBFS_GLOBAL, dp.USBFS_DEVICE, dp.USBFS_PWRCLK,
        gpioa.pa11, gpioa.pa12, &rcu,
        unsafe { &mut EP_MEMORY },
    );

    let serial = SerialPort::new(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Longan Nano")
        .product("Led Shell")
        .serial_number("1234")
        .device_class(USB_CLASS_CDC)
        .build();

    let autocomplete = StaticAutocomplete(["clear", "help", "status", "off ", "on "]);
    let mut shell = UShell::new(serial, autocomplete, LRUHistory::default());
    let mut env = Env { red, green, blue };

    loop {
        if !usb_dev.poll(&mut [shell.as_mut()]) {
            continue;
        }

        shell.spin(&mut env).ok();
    }
}

const CMD_LEN: usize = 16;
const HISTORY_SIZE: usize = 4;
const COMMANDS: usize = 5;

type Serial<'s> = SerialPort<'s, UsbBusType>;
type Autocomplete = StaticAutocomplete<{ COMMANDS }>;
type History = LRUHistory<{ CMD_LEN }, { HISTORY_SIZE }>;
type Shell<'s> = UShell<Serial<'s>, Autocomplete, History, { CMD_LEN }>;

struct Env {
    red: RED,
    green: GREEN,
    blue: BLUE,
}

type EnvResult<'s> = SpinResult<Serial<'s>, ()>;

impl<'s> Env {
    fn on_cmd(&mut self, shell: &mut Shell<'s>, args: &str) -> EnvResult<'s> {
        match args {
            "r" | "red" => self.red.on(),
            "g" | "green" => self.green.on(),
            "b" | "blue" => self.blue.on(),
            "a" | "all" => {
                self.red.on();
                self.green.on();
                self.blue.on();
            }
            _ => {
                write!(shell, "{0:}unsupported color channel", CR).ok();
            }
        }
        shell.write_str(CR)?;
        Ok(())
    }

    fn off_cmd(&mut self, shell: &mut Shell<'s>, args: &str) -> EnvResult<'s> {
        match args {
            "r" | "red" => self.red.off(),
            "g" | "green" => self.green.off(),
            "b" | "blue" => self.blue.off(),
            "a" | "all" => {
                self.red.off();
                self.green.off();
                self.blue.off();
            }
            _ => {
                write!(shell, "{0:}unsupported color channel", CR).ok();
            }
        }
        shell.write_str(CR)?;
        Ok(())
    }

    fn status_cmd(&mut self, shell: &mut Shell<'s>, _args: &str) -> EnvResult<'s> {
        let red = if self.red.is_on() { "On" } else { "Off" };
        let green = if self.green.is_on() { "On" } else { "Off" };
        let blue = if self.blue.is_on() { "On" } else { "Off" };
        write!(
            shell,
            "{0:}Red:   {1:}{0:}Green: {2:}{0:}Blue:  {3:}{0:}",
            CR, red, green, blue,
        )?;

        Ok(())
    }
}

impl<'s> Environment<Serial<'s>, Autocomplete, History, (), { CMD_LEN }> for Env {
    fn command(&mut self, shell: &mut Shell<'s>, cmd: &str, args: &str) -> EnvResult<'s> {
        match cmd {
            "clear" => shell.clear()?,
            "help" => shell.write_str(HELP)?,
            "status" => self.status_cmd(shell, args)?,
            "on" => self.on_cmd(shell, args)?,
            "off" => self.off_cmd(shell, args)?,
            "" => shell.write_str(CR)?,
            _ => write!(shell, "{0:}unsupported command: \"{1:}\"{0:}", CR, cmd)?,
        }
        shell.write_str(SHELL_PROMPT)?;
        Ok(())
    }

    fn control(&mut self, _shell: &mut Shell<'s>, _code: u8) -> EnvResult<'s> {
        Ok(())
    }
}

const SHELL_PROMPT: &str = "#> ";
const CR: &str = "\r\n";
const HELP: &str = "\r\n\
\x1b[31mL\x1b[32mE\x1b[34mD\x1b[33m Shell\x1b[0m\r\n\r\n\
USAGE:\r\n\
\x20 command [arg]\r\n\r\n\
COMMANDS:\r\n\
\x20 on  <ch>  Switch led channel on [r,g,b,a]\r\n\
\x20 off <ch>  Switch led channel off [r,g,b,a]\r\n\
\x20 status    Get leds status\r\n\
\x20 clear     Clear screen\r\n\
\x20 help      Print this message\r\n
";
