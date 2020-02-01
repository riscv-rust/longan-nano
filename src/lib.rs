//! Board support crate for the Longan Nano board

#![no_std]

pub use gd32vf103xx_hal as hal;

#[cfg(feature = "lcd")]
pub mod lcd;
pub mod led;
pub mod stdout;
