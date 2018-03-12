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
use gpio::*;
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

    let port_b = SystemIntegrationModule::enable_port_for_use(Ports::PortB);
    let port_d = SystemIntegrationModule::enable_port_for_use(Ports::PortD);
    let port_b_gpio_18 = port_b.set_pin_as_gpio(Pin::Pin18);
    let port_b_gpio_19 = port_b.set_pin_as_gpio(Pin::Pin19);
    let port_d_gpio_01 = port_d.set_pin_as_gpio(Pin::Pin1);
    let port_d_gpio_00 = port_d.set_pin_as_gpio(Pin::Pin0);

    // LEDs
    // RED
    port_b_gpio_18.set_value(Value::Low);
    delay(5_000_000);
    port_b_gpio_18.set_value(Value::High);

    // GREEN
    port_b_gpio_19.set_value(Value::Low);
    delay(5_000_000);
    port_b_gpio_19.set_value(Value::High);

    // BLUE
    port_d_gpio_01.set_value(Value::Low);
    delay(5_000_000);
    port_d_gpio_01.set_value(Value::High);
    delay(5_000_000);

    loop{

        port_d_gpio_00.set_value(Value::Low);
        delay(9_000);
        port_d_gpio_00.set_value(Value::High);
        delay(1_000);
    }

}

// As we are not using interrupts, we just register a dummy catch all handler

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
