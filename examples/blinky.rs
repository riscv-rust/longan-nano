#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use gd32vf103xx_hal as hal;
use hal::pac as pac;
use gd32vf103xx_hal::gpio::GpioExt;
use longan_nano::led::Led;
use gd32vf103xx_hal::rcu::RcuExt;
use gd32vf103xx_hal::delay::McycleDelay;
use embedded_hal::blocking::delay::DelayMs;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.configure().freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);

    let mut led_red = gpioc.pc13.into_push_pull_output();
    led_red.off();
    let mut led_green = gpioa.pa1.into_push_pull_output();
    led_green.off();
    let mut led_blue = gpioa.pa2.into_push_pull_output();
    led_blue.off();

    let leds: [&mut dyn Led; 3] = [&mut led_red, &mut led_green, &mut led_blue];

    let mut delay = McycleDelay::new(&rcu.clocks);

    let mut i = 0;
    loop {
        let inext = (i + 1) % leds.len();
        leds[i].off();
        leds[inext].on();
        delay.delay_ms(500);

        i = inext;
    }
}
