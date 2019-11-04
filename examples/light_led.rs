#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use gd32vf103xx_hal as hal;
use hal::pac as pac;
use gd32vf103xx_hal::gpio::GpioExt;
use longan_nano::led::Led;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let mut red_led = gpioa.pa1.into_push_pull_output();
    red_led.on();

    loop {}
}
