//! On-board RGB led
//!
//! - Red = PC13
//! - Green = PA1
//! - Blue = PA2
use embedded_hal::digital::v2::OutputPin;
use gd32vf103xx_hal::gpio::gpioc::PC13;
use gd32vf103xx_hal::gpio::gpioa::{PA1, PA2};
use gd32vf103xx_hal::gpio::{Output, PushPull, Active};

/// Red LED
pub type RED = PC13<Output<PushPull>>;

/// Green LED
pub type GREEN = PA1<Output<PushPull>>;

/// Blue LED
pub type BLUE = PA2<Output<PushPull>>;

/// Returns RED, GREEN and BLUE LEDs.
pub fn rgb<X, Y, Z>(
    red: PC13<X>, green: PA1<Y>, blue: PA2<Z>
) -> (RED, GREEN, BLUE)
where X: Active, Y: Active, Z: Active
{
    let red: RED = red.into_push_pull_output();
    let green: GREEN = green.into_push_pull_output();
    let blue: BLUE = blue.into_push_pull_output();
    (red, green, blue)
}

/// Generic LED
pub trait Led {
    /// Turns the LED off
    fn off(&mut self);

    /// Turns the LED on
    fn on(&mut self);
}

impl Led for RED {
    fn off(&mut self) {
        self.set_high().unwrap();
    }

    fn on(&mut self) {
        self.set_low().unwrap();
    }
}

impl Led for GREEN {
    fn off(&mut self) {
        self.set_high().unwrap();
    }

    fn on(&mut self) {
        self.set_low().unwrap();
    }
}

impl Led for BLUE {
    fn off(&mut self) {
        self.set_high().unwrap();
    }

    fn on(&mut self) {
        self.set_low().unwrap();
    }
}
