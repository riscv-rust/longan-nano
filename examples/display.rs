#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use gd32vf103xx_hal::pac as pac;
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::spi::{Spi, MODE_0};
use gd32vf103xx_hal::delay::McycleDelay;
use embedded_hal::digital::v2::OutputPin;
use st7735_lcd::{ST7735, Orientation};
use embedded_graphics::prelude::*;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::Rectangle;


#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp.RCU.configure().ext_hf_clock(8.mhz()).sysclk(108.mhz()).freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    let sck = gpioa.pa5.into_alternate_push_pull();
    let miso = gpioa.pa6.into_floating_input();
    let mosi = gpioa.pa7.into_alternate_push_pull();
    let spi0 = Spi::spi0(dp.SPI0, (sck, miso, mosi), MODE_0, 16.mhz(), &mut rcu);

    let dc = gpiob.pb0.into_push_pull_output();
    let rst = gpiob.pb1.into_push_pull_output();
    let mut cs = gpiob.pb2.into_push_pull_output();
    cs.set_low().unwrap();

    let mut lcd = ST7735::new(spi0, dc, rst, false, true);
    let mut delay = McycleDelay::new(&rcu.clocks);
    lcd.init(&mut delay).unwrap();
    lcd.set_orientation(&Orientation::Landscape).unwrap();
    lcd.set_offset(0, 26);

    lcd.draw(Rectangle::new(Coord::new(0, 0), Coord::new(179, 79)).fill(Some(Rgb565::from(0x0u8))));
    let t = Font6x8::render_str(" Hello Rust! ").fill(Some(Rgb565::from((0,0xff,0)))).translate(Coord::new(40, 35));
    lcd.draw(t);

    loop {}
}
