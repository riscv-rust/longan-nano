//! Board support crate for the Longan Nano board

#![no_std]

pub use gd32vf103xx_hal as hal;

pub mod led;
pub mod stdout;
