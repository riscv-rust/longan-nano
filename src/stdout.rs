//! Stdout based on the UART hooked up to the debug connector

use core::fmt::{self, Write};
use nb::block;
use riscv::interrupt;
use gd32vf103xx_hal::{
    serial::{Serial, Tx},
    gpio::{Active, gpioa::{PA10, PA9}},
    time::Bps,
    rcu::Rcu,
    afio::Afio,
    pac::USART0,
    prelude::*
};
use gd32vf103xx_hal::serial::{Config, Parity, StopBits};


static mut STDOUT: Option<SerialWrapper> = None;


struct SerialWrapper(Tx<USART0>);

impl fmt::Write for SerialWrapper {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            if *byte == '\n' as u8 {
                let res = block!(self.0.write('\r' as u8));

                if res.is_err() {
                    return Err(::core::fmt::Error);
                }
            }

            let res = block!(self.0.write(*byte));

            if res.is_err() {
                return Err(::core::fmt::Error);
            }
        }
        Ok(())
    }
}

/// Configures stdout
pub fn configure<X, Y>(
    uart: USART0, tx: PA9<X>, rx: PA10<Y>,
    baud_rate: Bps, afio: &mut Afio, rcu: &mut Rcu
) where X: Active, Y: Active
{
    let tx = tx.into_alternate_push_pull();
    let rx = rx.into_floating_input();
    let config = Config {
        baudrate: baud_rate,
        parity: Parity::ParityNone,
        stopbits: StopBits::STOP1
    };
    let serial = Serial::new(uart, (tx, rx), config, afio, rcu);
    let (tx, _) = serial.split();

    interrupt::free(|_| {
        unsafe {
            STDOUT.replace(SerialWrapper(tx));
        }
    })
}

/// Writes string to stdout
pub fn write_str(s: &str) {
    interrupt::free(|_| unsafe {
        if let Some(stdout) = STDOUT.as_mut() {
            let _ = stdout.write_str(s);
        }
    })
}

/// Writes formatted string to stdout
pub fn write_fmt(args: fmt::Arguments) {
    interrupt::free(|_| unsafe {
        if let Some(stdout) = STDOUT.as_mut() {
            let _ = stdout.write_fmt(args);
        }
    })
}

/// Macro for printing to the serial standard output
#[macro_export]
macro_rules! sprint {
    ($s:expr) => {
        $crate::stdout::write_str($s)
    };
    ($($tt:tt)*) => {
        $crate::stdout::write_fmt(format_args!($($tt)*))
    };
}

/// Macro for printing to the serial standard output, with a newline.
#[macro_export]
macro_rules! sprintln {
    () => {
        $crate::stdout::write_str("\n")
    };
    ($s:expr) => {
        $crate::stdout::write_str(concat!($s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::stdout::write_fmt(format_args!(concat!($s, "\n"), $($tt)*))
    };
}
