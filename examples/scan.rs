#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use longan_nano::hal::prelude::*;
use longan_nano::hal::pac::Peripherals;
use longan_nano::{sprint, sprintln};

#[inline(never)]
pub unsafe extern "C" fn __read32(_default: usize, addr: usize) -> u32 {
    let ptr = addr as *const u32;
    ptr.read_volatile()
}

#[export_name = "trap_handler"]
fn trap_handler() {
    use riscv::register::{mcause, mepc, mtval, mcause::{Trap, Exception}};
    let ld_insn_addr = __read32 as *const () as usize;

    let mcause = mcause::read();
    let mepc = mepc::read();

    if mepc == ld_insn_addr && mcause.cause() == Trap::Exception(Exception::LoadFault) {
        mepc::write(mepc + 2);
        return;
    }

    sprintln!("trap!");
    sprintln!("mcause={:08x} mepc={:08x} mtval={:08x} addr={:08x}", mcause.bits(), mepc, mtval::read(), ld_insn_addr);

    loop {}
}

fn read32_safe(addr: usize) -> Option<u32> {
    unsafe {
        let v = __read32(0xdeadbeef, addr);
        if v != 0xdeadbeef {
            return Some(v);
        }
        let v = __read32(0x12345678, addr);
        if v != 0x12345678 {
            Some(v)
        } else {
            None
        }
    }
}

fn is_readable(addr: usize) -> bool {
    read32_safe(addr).is_some()
}

#[derive(Copy, Clone)]
enum ScanState {
    Invalid,
    Valid {
        interval_start: usize,
    },
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = p.RCU.configure().freeze();
    let mut afio = p.AFIO.constrain(&mut rcu);

    // Configure UART for stdout
    let gpioa = p.GPIOA.split(&mut rcu);
    longan_nano::stdout::configure(p.USART0, gpioa.pa9, gpioa.pa10, 115_200.bps(), &mut afio, &mut rcu);

    sprintln!("scan started");


    let mut addr: usize = 0;
    let mut state = ScanState::Invalid;
    loop {
        if (addr & 0xfffff) == 0 {
            sprint!("\r==> {:08x}", addr);
        }

        let readable = is_readable(addr);
        state = match (state, readable) {
            (ScanState::Invalid, true) => ScanState::Valid {
                interval_start: addr
            },
            (ScanState::Valid { interval_start }, false) => {
                sprintln!("\r{:08x}..{:08x}", interval_start, addr - 1);
                ScanState::Invalid
            },
            (ScanState::Valid {..}, true) | (ScanState::Invalid, false) => state,
        };

        if let Some(v) = addr.checked_add(0x100) {
            addr = v;
        } else {
            sprint!("\r==> {:08x}", addr);
            break;
        }
    }
    if let ScanState::Valid { interval_start } = state {
        sprintln!("\n{:08x}..{:08x}..", interval_start, addr - 1);
    }

    sprintln!("\nscan finished");

    loop {}
}
