#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::prelude::*;
use longan_nano::lcd_pins;
use longan_nano::lcd::Lcd;
use embedded_graphics::prelude::*;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::Rectangle;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp.RCU.configure().ext_hf_clock(8.mhz()).sysclk(108.mhz()).freeze();
    let mut afio = dp.AFIO.constrain(&mut rcu);

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = Lcd::new(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.width() as i32, lcd.height() as i32);

    // Clear screen
    lcd.draw(Rectangle::new(Coord::new(0, 0), Coord::new(width - 1, height - 1))
        .fill(Some(Rgb565::from(0u16))));

    let t = Font6x8::render_str(" Hello Rust! ")
        .fill(Some(Rgb565::from((0,0xff,0))))
        .translate(Coord::new(40, 35));
    lcd.draw(t);

    loop {}
}
