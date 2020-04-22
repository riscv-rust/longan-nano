//! On-board RGB leds
//!
//! - Red = PC13
//! - Green = PA1
//! - Blue = PA2
use embedded_hal::digital::v2::OutputPin;
use gd32vf103xx_hal::gpio::gpioc::PC13;
use gd32vf103xx_hal::gpio::gpioa::{PA1, PA2};
use gd32vf103xx_hal::gpio::{Output, PushPull, Active};

/// Red LED
pub struct RED {
    port: PC13<Output<PushPull>>
}

impl RED {
    pub fn new<T: Active>(port: PC13<T>) -> Self {
        Self {
            port: port.into_push_pull_output()
        }
    }
}

/// Green LED
pub struct GREEN {
    port: PA1<Output<PushPull>>
}

impl GREEN {
    pub fn new<T: Active>(port: PA1<T>) -> Self {
        Self {
            port: port.into_push_pull_output()
        }
    }
}

/// Blue LED
pub struct BLUE {
    port: PA2<Output<PushPull>>
}

impl BLUE {
    pub fn new<T: Active>(port: PA2<T>) -> Self {
        Self {
            port: port.into_push_pull_output()
        }
    }
}

/// Returns RED, GREEN and BLUE LEDs.
pub fn rgb<X, Y, Z>(
    red: PC13<X>, green: PA1<Y>, blue: PA2<Z>
) -> (RED, GREEN, BLUE)
where X: Active, Y: Active, Z: Active
{
    let red: RED = RED::new(red);
    let green: GREEN = GREEN::new(green);
    let blue: BLUE = BLUE::new(blue);
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
        self.port.set_high().unwrap();
    }

    fn on(&mut self) {
        self.port.set_low().unwrap();
    }
}

impl Led for GREEN {
    fn off(&mut self) {
        self.port.set_high().unwrap();
    }

    fn on(&mut self) {
        self.port.set_low().unwrap();
    }
}

impl Led for BLUE {
    fn off(&mut self) {
        self.port.set_high().unwrap();
    }

    fn on(&mut self) {
        self.port.set_low().unwrap();
    }
}
