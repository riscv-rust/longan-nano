//! Board support crate for the Longan Nano board

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use gd32vf103xx_hal as hal;

#[cfg(feature = "lcd")]
#[cfg_attr(docsrs, doc(cfg(feature = "lcd")))]
pub mod lcd;
pub mod led;
pub mod stdout;
