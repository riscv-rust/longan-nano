#![no_std]
#![no_main]

use gd32vf103xx_hal::pac::Interrupt;
use panic_halt as _;
use longan_nano::hal::{pac, prelude::*, pac::*, eclic::*};
use gd32vf103xx_hal::timer;
use gd32vf103xx_hal::timer::Timer;
use longan_nano::led::{rgb, Led, RED};
use riscv_rt::entry;

static mut R_LED: Option<RED> = None;
static mut G_TIMER1: Option<Timer<TIMER1>> = None;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);

    let (mut red, mut green, mut blue) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);
    red.off();
    green.off();
    blue.off();
    unsafe { R_LED = Some(red); };

    ECLIC::reset();
    ECLIC::set_threshold_level(Level::L0);
    ECLIC::set_level_priority_bits(LevelPriorityBits::L3P1);

    // timer
    let mut timer =  Timer::timer1(dp.TIMER1, 1.hz(), &mut rcu);
    timer.listen(timer::Event::Update);
    unsafe {G_TIMER1 = Some(timer)};

    ECLIC::setup(
        Interrupt::TIMER1,
        TriggerType::Level,
        Level::L1,
        Priority::P1,
    );
    unsafe { 
        ECLIC::unmask(Interrupt::TIMER1);
        riscv::interrupt::enable();
    };

    loop { }
}

#[allow(non_snake_case)]
#[no_mangle]
fn TIMER1() {
    unsafe {
        if let Some(timer1) = G_TIMER1.as_mut() {
            timer1.clear_update_interrupt_flag();
        }
        if let Some(led) = R_LED.as_mut() {
            if led.is_on() {
                led.off();
            } else {
                led.on();
            }
        }
    }
}