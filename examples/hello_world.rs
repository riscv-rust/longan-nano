#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use gd32vf103xx_hal as hal;
use hal::pac as pac;
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::serial::{Serial, Config, Parity, StopBits};
use core::fmt::Write;
use gd32vf103xx_hal::clock::Clocks;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);


    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.ctl1);
    let rx = gpioa.pa10.into_floating_input(&mut gpioa.ctl1);
    let serial_config = Config {
        baudrate: 115_200.bps(),
        parity: Parity::ParityNone,
        stopbits: StopBits::STOP1
    };
    let clocks = Clocks;
    let serial = Serial::usart0(dp.USART0, (tx, rx), serial_config, clocks, &mut rcu.apb2);

    let (mut tx, _) = serial.split();

    writeln!(tx, "Hello, world").unwrap();

    loop { }
}
