#![no_std]
#![no_main]

use panic_halt as _;

use core::sync::atomic::{AtomicUsize, Ordering};

use gd32vf103xx_hal::{pac::Interrupt, timer::{self, Timer}, usbfs::device::UsbBus};
use longan_nano::hal::{eclic::*, pac::{self, *}, prelude::*};

use riscv_rt::entry;

use usb_device::{
    self,
    class_prelude::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_hid::descriptor::{KeyboardReport, KeyboardUsage, SerializedDescriptor};
use usbd_hid::hid_class::HIDClass;

const HELLO: [(u8, u8); 5] = [
    (
        KeyboardUsage::KeyboardLeftShift as u8,
        KeyboardUsage::KeyboardHh as u8,
    ),
    (0x00, KeyboardUsage::KeyboardEe as u8),
    (0x00, KeyboardUsage::KeyboardLl as u8),
    (0x00, KeyboardUsage::KeyboardLl as u8),
    (0x00, KeyboardUsage::KeyboardOo as u8),
];

static INDEX: AtomicUsize = AtomicUsize::new(0);

fn index() -> usize {
    INDEX.load(Ordering::Relaxed)
}

fn increment_index() -> usize {
    let index = (INDEX.load(Ordering::Relaxed) + 1) % HELLO.len();
    INDEX.store(index, Ordering::SeqCst);
    index
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let mut timer = Timer::timer2(dp.TIMER2, 48.mhz(), &mut rcu);
    timer.listen(timer::Event::Update);

    let usb_dev = UsbBus::new(
        dp.USBFS_GLOBAL,
        dp.USBFS_DEVICE,
        dp.USBFS_PWRCLK,
        timer,
    );

    unsafe {
        let usb_bus = &*USB_BUS.insert(UsbBusAllocator::new(usb_dev));
        HID_CLASS = Some(HIDClass::new(usb_bus, KeyboardReport::desc(), 1));
        USB_DEV = Some(
            UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x0001))
                .manufacturer("Longboi")
                .product("Keeb")
                .build(),
        );
    }

    ECLIC::reset();
    ECLIC::set_threshold_level(Level::L0);
    ECLIC::set_level_priority_bits(LevelPriorityBits::L3P1);

    // setup the USBFS interrupt
    ECLIC::setup(
        Interrupt::USBFS,
        TriggerType::Level,
        Level::L1,
        Priority::P1,
    );

    // setup the USBFS_WKUP interrupt
    ECLIC::setup(
        Interrupt::USBFS_WKUP,
        TriggerType::Level,
        Level::L1,
        Priority::P2,
    );

    unsafe { 
        // unmask and enable interrupts
        ECLIC::unmask(Interrupt::USBFS);
        ECLIC::unmask(Interrupt::USBFS_WKUP);

        riscv::interrupt::enable();
    };

    loop {}
}

static mut HID_CLASS: Option<HIDClass<UsbBus>> = None;
static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_DEV: Option<UsbDevice<UsbBus>> = None;

fn send_report() {
    let idx = index();
    let key = HELLO[idx];

    let report = KeyboardReport {
        modifier: key.0,
        reserved: 0,
        leds: 0,
        keycodes: [key.1, 0x00, 0x00, 0x00, 0x00, 0x00],
    };

    unsafe {
        let hid_class = HID_CLASS.as_mut().unwrap();
        if hid_class.push_input(&report).is_ok() {
            increment_index();
        }

        if let Some(usb) = USB_DEV.as_mut() {
            usb.poll(&mut [hid_class]);
        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
fn USBFS() {
    send_report();
}

#[allow(non_snake_case)]
#[no_mangle]
fn USBFS_WKUP() {
    send_report();
}
