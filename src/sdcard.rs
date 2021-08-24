//! On-board SD Card Slot

use embedded_hal::digital::v2::OutputPin;
use gd32vf103xx_hal::gpio::gpiob::{PB12, PB13, PB14, PB15};
use gd32vf103xx_hal::gpio::{Alternate, Floating, Input, Output, PushPull};
use gd32vf103xx_hal::pac::{SPI1};
use gd32vf103xx_hal::rcu::Rcu;
use gd32vf103xx_hal::spi::{Spi, MODE_0};
use gd32vf103xx_hal::time::{Hertz, U32Ext};
use embedded_sdmmc::{Controller, SdMmcSpi, TimeSource, Timestamp};

/// Sets up all the needed GPIO pins for the sdcard
///
/// ```
/// let gpiob = dp.GPIOB.split(&mut rcu);
/// let sdcard_pins = sdcard_pins!(gpiob);
/// ```
#[macro_export]
macro_rules! sdcard_pins {
    ($gpiob:ident) => {{
        $crate::sdcard::SdCardPins {
            miso: $gpiob.pb14.into_floating_input(),
            mosi: $gpiob.pb15.into_alternate_push_pull(),
            sck: $gpiob.pb13.into_alternate_push_pull(),
            cs: $gpiob.pb12.into_push_pull_output(),
        }
    }};
}

type SckPin = PB13<Alternate<PushPull>>;
type MisoPin = PB14<Input<Floating>>;
type MosiPin = PB15<Alternate<PushPull>>;
type CsPin = PB12<Output<PushPull>>;
type SPI1Pins = (SckPin, MisoPin, MosiPin);

type Spi1 = Spi<SPI1, SPI1Pins>;

/// A type based on embedded_sdmmc::SdMmcSpi that is used by SdCard.
pub type SdCardSpi = SdMmcSpi<Spi1, CsPin>;

/// A type based on embedded_sdmmc::Controller.
pub type SdCard = Controller<SdCardSpi, FakeTimeSource>;

pub struct SdCardPins {
    pub miso: MisoPin,
    pub mosi: MosiPin,
    pub sck: SckPin,
    pub cs: CsPin,
}

pub enum SdCardFreq {
    /// Should work for all cards
    Safe,
    /// May not work for some cards
    Fast,
    /// Specify SPI frequency
    Custom(Hertz)
}

impl From<SdCardFreq> for Hertz {
    fn from(value: SdCardFreq) -> Hertz {
        match value {
            SdCardFreq::Safe => 200.khz().into(), // using 300 kHz here because the sdcard init needs 100 to 400 kHz (see SdMmcSpi.init)
            SdCardFreq::Fast => 27.mhz().into(),  // this is the max SPI frequency according to datasheet
            SdCardFreq::Custom(val) => val,
        }
    }
}

/// Constructs SD Card driver from the required components.
pub fn configure(spi: SPI1, pins: SdCardPins, freq: SdCardFreq, rcu: &mut Rcu) -> SdCard {
    let spi1 = Spi::spi1(
        spi,
        (pins.sck, pins.miso, pins.mosi),
        MODE_0,
        freq,
        rcu,
    );

    let mut cs = pins.cs;
    cs.set_high().unwrap();

    let sdmmcspi = SdMmcSpi::new(spi1, cs);
    let ctime_source = FakeTimeSource {};

    Controller::new(sdmmcspi, ctime_source)
}

/// A fake time source that always returns a date of zero.
pub struct FakeTimeSource {}

impl TimeSource for FakeTimeSource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}
