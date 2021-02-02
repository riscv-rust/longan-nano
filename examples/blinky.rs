#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use longan_nano::hal::{pac, prelude::*};
use longan_nano::led::{Led, rgb};
use longan_nano::hal::delay::McycleDelay;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.configure().freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);

    let (mut red, mut green, mut blue) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);
    let leds: [&mut dyn Led; 3] = [&mut red, &mut green, &mut blue];

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
