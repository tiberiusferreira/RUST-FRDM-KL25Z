//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![feature(used)]
#![no_std]
#![feature(core_intrinsics)]
#![feature(asm)]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
mod io;
mod frdm_kl25z;
use frdm_kl25z::*;
use io::VolatileRW;
mod multi_purpose_clock_generator;
mod system_integration_module;
mod gpio;
mod port;
use gpio::Gpio;
use port::*;
use system_integration_module::SystemIntegrationModule;
use core::fmt::Write;

use cortex_m::asm;
use cortex_m_semihosting::hio;


pub fn delay(mut cycles: u32)
{
    while cycles > 0 {
        unsafe {
            asm!("nop" :::: "volatile");
        }
        cycles = cycles - 1;
    }
}





const BASE_SYSTEM_OSCILLATOR: u32 = 0x4006_5000;

#[repr(C)]
pub struct Oscillator {
    pub cr : VolatileRW<u8>,
}

impl Oscillator {
    pub fn get() -> &'static Oscillator {
        unsafe {
            &*(BASE_SYSTEM_OSCILLATOR as *const Oscillator)
        }
    }
}


fn main() {
    let frdmkl25z = FrdmKl25zBoard::init();
    frdmkl25z.disable_watchdog_timer();

//    let mut stdout = hio::hstdout().unwrap();
//
//    writeln!(stdout, "Hello, world!").unwrap();

    // Enabling clock on PORT B
    let sim = SystemIntegrationModule::get();
    sim.system_clock_gating_control_register_5.bitwise_inc_or(0x400);
    let port_b = Port::get(Ports::PortB);

    // Set RED LED control register to GPIO
    port_b.pin_control_register[18].set(1 << 8);
    // Set GREEN LED control register to GPIO
    port_b.pin_control_register[19].set(1 << 8);

    let ptb = Gpio::get(1);
    ptb.port_data_direction_register.set(0b11 << 18);
    ptb.port_set_output_register.set(0b11u32 << 18);

    loop{
//        ptb.port_clear_output_register.set(0b0u32 << 18);
        ptb.port_clear_output_register.set(0b01 << 18);
        delay(5_000_000);
        ptb.port_set_output_register.set(0b01 << 18);

        ptb.port_clear_output_register.set(0b10 << 18);
        delay(5_000_000);
        ptb.port_set_output_register.set(0b10 << 18);

    }

}

// As we are not using interrupts, we just register a dummy catch all handler

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
