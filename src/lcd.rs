//! On-board LCD

use gd32vf103xx_hal::gpio::{Input, Output, Alternate, PushPull, Floating};
use gd32vf103xx_hal::gpio::gpioa::{PA5, PA6, PA7};
use gd32vf103xx_hal::gpio::gpiob::{PB0, PB1, PB2};
use gd32vf103xx_hal::rcu::Rcu;
use gd32vf103xx_hal::afio::Afio;
use gd32vf103xx_hal::spi::{Spi, MODE_0};
use gd32vf103xx_hal::pac::SPI0;
use st7735_lcd::{ST7735, Orientation};
use embedded_hal::digital::v2::OutputPin;
use gd32vf103xx_hal::time::U32Ext;
use gd32vf103xx_hal::delay::McycleDelay;
use core::ops::{Deref, DerefMut};

/// Sets up all the needed GPIO pins for the LCD
///
/// ```
/// let gpioa = dp.GPIOA.split(&mut rcu);
/// let gpiob = dp.GPIOB.split(&mut rcu);
/// let lcd_pins = lcd_pins!(gpioa, gpiob);
/// ```
#[macro_export]
macro_rules! lcd_pins {
    ($gpioa:ident, $gpiob:ident) => {{
        $crate::lcd::LcdPins {
            miso: $gpioa.pa6.into_floating_input(),
            mosi: $gpioa.pa7.into_alternate_push_pull(),
            sck: $gpioa.pa5.into_alternate_push_pull(),
            cs: $gpiob.pb2.into_push_pull_output(),
            dc: $gpiob.pb0.into_push_pull_output(),
            rst: $gpiob.pb1.into_push_pull_output(),
        }
    }}
}

type MisoPin = PA6<Input<Floating>>;
type MosiPin = PA7<Alternate<PushPull>>;
type SckPin = PA5<Alternate<PushPull>>;
type CsPin = PB2<Output<PushPull>>;
type DcPin = PB0<Output<PushPull>>;
type RstPin = PB1<Output<PushPull>>;
type SpiType = Spi<SPI0, (SckPin, MisoPin, MosiPin)>;
type LcdType = ST7735<SpiType, DcPin, RstPin>;

/// Pins consumed by LCD driver
pub struct LcdPins {
    pub miso: MisoPin,
    pub mosi: MosiPin,
    pub sck: SckPin,
    pub cs: CsPin,
    pub dc: DcPin,
    pub rst: RstPin,
}

/// LCD driver wrapper
pub struct Lcd {
    driver: LcdType,
}

impl Lcd {
    /// Constructs LCD driver from the required components
    pub fn new(spi: SPI0, pins: LcdPins, afio: &mut Afio, rcu: &mut Rcu) -> Lcd {
        let spi0 = Spi::spi0(spi, (pins.sck, pins.miso, pins.mosi), afio, MODE_0, 16.mhz(), rcu);

        let mut cs = pins.cs;
        cs.set_low().unwrap();

        let mut lcd = ST7735::new(spi0, pins.dc, pins.rst, false, true);
        let mut delay = McycleDelay::new(&rcu.clocks);
        lcd.init(&mut delay).unwrap();
        lcd.set_orientation(&Orientation::Landscape).unwrap();
        lcd.set_offset(0, 26);

        Lcd {
            driver: lcd
        }
    }

    /// Screen width
    pub const fn width(&self) -> u32 {
        160
    }

    /// Screen height
    pub const fn height(&self) -> u32 {
        80
    }
}

impl Deref for Lcd {
    type Target = LcdType;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.driver
    }
}

impl DerefMut for Lcd {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.driver
    }
}
