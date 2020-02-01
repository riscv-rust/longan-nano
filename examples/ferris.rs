#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::spi::{Spi, MODE_0};
use gd32vf103xx_hal::delay::McycleDelay;
use embedded_hal::digital::v2::OutputPin;
use st7735_lcd::{ST7735, Orientation};
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::image::Image16BPP;
use embedded_graphics::primitives::Rectangle;

const FERRIS: &[u8] = include_bytes!("ferris.raw");
const LCD_WIDTH: i32 = 180;
const LCD_HEIGHT: i32 = 80;

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

    let p = Coord::new(LCD_WIDTH as i32 - 1, LCD_HEIGHT as i32 - 1);
    lcd.draw(Rectangle::new(Coord::new(0, 0), p).fill(Some(Rgb565::from((0,0,0)))));

    let image: Image<Rgb565, _> = Image16BPP::new(&FERRIS, 86, 64);
    let image = image.translate(Coord::new(LCD_WIDTH/2 - 43, LCD_HEIGHT/2 - 32));
    lcd.draw(&image);

    loop {}
}
