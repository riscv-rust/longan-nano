#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use gd32vf103xx_hal as hal;
use hal::pac as pac;
use gd32vf103xx_hal::gpio::GpioExt;
use gd32vf103xx_hal::rcu::RcuExt;
use longan_nano::led::Led;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut red_led = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0);
    red_led.on();

    loop {}
}
