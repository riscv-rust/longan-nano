#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use gd32vf103xx_hal as hal;
use hal::pac as pac;
use gd32vf103xx_hal::prelude::*;
use longan_nano::sprintln;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();

    // Configure clocks
    let rcu = dp.RCU.constrain();
    let clocks = rcu.cctl
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    longan_nano::stdout::configure(dp.USART0, gpioa.pa9, gpioa.pa10, 115_200.bps(), clocks);

    sprintln!("Hello, world");

    loop { }
}
