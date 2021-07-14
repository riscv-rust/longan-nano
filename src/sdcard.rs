//! On-board SD Card Slot

use embedded_hal::digital::v2::OutputPin;
use gd32vf103xx_hal::gpio::gpiob::{PB12, PB13, PB14, PB15, Parts};
use gd32vf103xx_hal::gpio::{Alternate, Floating, Input, Output, PushPull};
use gd32vf103xx_hal::pac::{SPI1};
use gd32vf103xx_hal::rcu::Rcu;
use gd32vf103xx_hal::spi::{Spi, MODE_0};
use gd32vf103xx_hal::time::U32Ext;
use embedded_sdmmc::{Controller, SdMmcSpi, TimeSource, Timestamp};

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

/// Constructs SD Card driver from the required components.
pub fn configure(spi: SPI1, gpiob: Parts, rcu: &mut Rcu) -> SdCard {
    let miso = gpiob.pb14.into_floating_input();
    let mosi = gpiob.pb15.into_alternate_push_pull();
    let sck = gpiob.pb13.into_alternate_push_pull();
    let mut cs = gpiob.pb12.into_push_pull_output();

    let spi1 = Spi::spi1(
        spi,
        (sck, miso, mosi),
        MODE_0,
        300.khz(), // using 300 kHz here because the sdcard init needs 100 to 400 kHz (see SdMmcSpi.init)
        rcu,
    );

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
