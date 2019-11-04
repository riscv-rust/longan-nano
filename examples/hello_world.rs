#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use gd32vf103xx_hal as hal;
use hal::pac as pac;
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::clock::Clocks;
use longan_nano::sprintln;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();

    let clocks = Clocks;
    longan_nano::stdout::configure(dp.USART0, gpioa.pa9, gpioa.pa10, 115_200.bps(), clocks);

    sprintln!("Hello, world");

    loop { }
}
