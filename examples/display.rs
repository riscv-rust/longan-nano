#![no_std]
#![no_main]

use panic_halt as _;

use embedded_graphics::fonts::{Font6x8, Text};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::{primitive_style, text_style};
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::prelude::*;
use longan_nano::{lcd, lcd_pins};
use riscv_rt::entry;

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
    let gpiob = dp.GPIOB.split(&mut rcu);

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    // Clear screen
    Rectangle::new(Point::new(0, 0), Point::new(width - 1, height - 1))
        .into_styled(primitive_style!(fill_color = Rgb565::BLACK))
        .draw(&mut lcd)
        .unwrap();

    let style = text_style!(
        font = Font6x8,
        text_color = Rgb565::BLACK,
        background_color = Rgb565::GREEN
    );

    // Create a text at position (20, 30) and draw it using style defined above
    Text::new(" Hello Rust! ", Point::new(40, 35))
        .into_styled(style)
        .draw(&mut lcd)
        .unwrap();

    loop {}
}
