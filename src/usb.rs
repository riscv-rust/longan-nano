//! On-board USB OTG FS
//!
//! - D- = PA11
//! - D+ = PA12

use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::rcu::Rcu;
use gd32vf103xx_hal::gpio::gpioa::{PA11, PA12};
use gd32vf103xx_hal::gpio::Active;
pub use gd32vf103xx_hal::otg_fs::{UsbBus, UsbBusType, USB};

use usb_device::bus::UsbBusAllocator;

/// Initializes USB FS bus
pub fn usb<DM: Active, DP: Active>(
    global: pac::USBFS_GLOBAL,
    device: pac::USBFS_DEVICE,
    pwrclk: pac::USBFS_PWRCLK,
    pin_dm: PA11<DM>,
    pin_dp: PA12<DP>,
    rcu: &Rcu,
    ep_mem: &'static mut [u32],
) -> UsbBusAllocator<UsbBusType> {
    let usb = USB {
        usb_global: global,
        usb_device: device,
        usb_pwrclk: pwrclk,
        pin_dm: pin_dm.into_floating_input(),
        pin_dp: pin_dp.into_floating_input(),
        hclk: rcu.clocks.hclk(),
    };

    UsbBus::new(usb, ep_mem)
}
