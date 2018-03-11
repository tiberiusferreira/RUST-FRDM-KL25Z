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
use io::VolatileRW;

use core::fmt::Write;

use cortex_m::asm;
use cortex_m_semihosting::hio;

// Accessing GPIOs through the cross bar/AIPS interface
const BASE_PTA :u32 = 0x400F_F000;

pub fn delay(mut cycles: u32)
{
    while cycles > 0 {
        unsafe {
            asm!("nop" :::: "volatile");
        }
        cycles = cycles - 1;
    }
}
#[repr(C)]
pub struct Gpio {
    // Actual value of the GPIO
    pub port_data_output_register: VolatileRW<u32>,

    // Whether to set it or not
    pub port_set_output_register: VolatileRW<u32>,

    // Whether to clear it or not
    pub port_clear_output_register: VolatileRW<u32>,

    // Whether to invert it or not
    pub port_toggle_output_register: VolatileRW<u32>,

    // Value of the GPIO when
    pub port_data_input_register: VolatileRW<u32>,

    // Direction of the GPIO: 0 = input // 1 = output
    pub port_data_direction_register: VolatileRW<u32>,
}

impl Gpio {
    pub fn get(port : u32) -> &'static Gpio {
        unsafe {
            // From 0-A 1-B 2-C 3-D 4-E each one takes 0x40 and they are mapped in sequence
            &*((BASE_PTA + (port*0x40)) as *const Gpio)
        }
    }
}

const BASE_SIM : u32 = 0x4004_7000;

// System integration Module
#[repr(C)]
pub struct SystemIntegrationModule {
    pub system_option_register_1: VolatileRW<u32>,
    pub system_config_register: VolatileRW<u32>,
    pub reserved_0 : [u8; 4092],
    pub system_option_register_2: VolatileRW<u32>,
    pub reserved_1 : [u8; 4],
    pub system_option_register_4: VolatileRW<u32>,
    pub system_option_register_5: VolatileRW<u32>,
    pub reserved_2 : [u8; 4],
    pub system_option_register_7: VolatileRW<u32>,
    pub reserved_3 : [u8; 8],
    pub system_device_identification_register: VolatileRW<u32>, //read only
    pub reserved_4 : [u8; 12],
    pub system_clock_gating_control_register_4: VolatileRW<u32>,
    pub system_clock_gating_control_register_5: VolatileRW<u32>,
    pub system_clock_gating_control_register_6: VolatileRW<u32>,
    pub system_clock_gating_control_register_7: VolatileRW<u32>,
    pub system_clock_divider_register_1: VolatileRW<u32>,
    pub reserved_5 : [u8; 4],
    pub system_flash_config_register_1: VolatileRW<u32>,
    pub system_flash_config_register_2: VolatileRW<u32>,
    pub reserved_6 : VolatileRW<u32>,
    pub unique_identification_register_mid_high: VolatileRW<u32>,
    pub unique_identification_register_mid_low: VolatileRW<u32>,
    pub unique_identification_register_low: VolatileRW<u32>,
    pub reserved_7 : [u8; 156],
    pub cop_control_register: VolatileRW<u32>,
    pub service_cop_register: VolatileRW<u32>,
}

impl SystemIntegrationModule {
    pub fn get() -> &'static SystemIntegrationModule {
        unsafe {
            &*(BASE_SIM as *const SystemIntegrationModule)
        }
    }
}
const BASE_PORT_A: u32 = 0x4004_9000;

#[repr(C)]
pub struct Port {
    pub pin_control_register: [VolatileRW<u32>; 32],
    pub global_pin_low_register: VolatileRW<u32>,
    pub global_pin_high_register: VolatileRW<u32>,
    pub reserved_0 : [u8; 24],
    pub interrupt_status_flag_register: VolatileRW<u32>,
}

// Ports A B C D E map to memory regions which are 0x1000 of distance from each other
impl Port {
    pub fn get(port: u32) -> &'static Port {
        unsafe {
            &*((BASE_PORT_A + (port*0x1000)) as *const Port)
        }
    }
}


const BASE_MULTI_PURPOSE_CLOCK_GENERATOR: u32 = 0x4006_4000;

#[repr(C)]
pub struct MultiPurposeClockGenerator {
    pub control_register_1: VolatileRW<u8>,
    pub control_register_2: VolatileRW<u8>,
    pub control_register_3: VolatileRW<u8>,
    pub control_register_4: VolatileRW<u8>,
    pub control_register_5: VolatileRW<u8>,
    pub control_register_6: VolatileRW<u8>,
    pub status_register: VolatileRW<u8>,
    pub reserved_0 : [u8; 1],
    pub status_n_control_register: VolatileRW<u8>,
    pub reserved_1 : [u8; 1],
    pub auto_trim_compare_value_high_register: VolatileRW<u8>,
    pub auto_trim_compare_value_low_register: VolatileRW<u8>,
    pub control_register_7: VolatileRW<u8>,
    pub control_register_8: VolatileRW<u8>,
    pub control_register_9: VolatileRW<u8>,
    pub control_register_10: VolatileRW<u8>,
}

impl MultiPurposeClockGenerator {
    pub fn get() -> &'static MultiPurposeClockGenerator {
        unsafe {
            &*(BASE_MULTI_PURPOSE_CLOCK_GENERATOR as *const MultiPurposeClockGenerator)
        }
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
    let sim = SystemIntegrationModule::get();
    // Disable Watchdog
    sim.cop_control_register.set(00 << 2);

    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "Hello, world! F2").unwrap();

    let sim = SystemIntegrationModule::get();
    sim.system_clock_gating_control_register_5.bitwise_inc_or(0x400);
    let port_b = Port::get(1);
    // set control register to GPIO
    port_b.pin_control_register[18].set(1 << 8);

    let ptb = Gpio::get(1);
    ptb.port_data_direction_register.set(1 << 18);

    loop{
        ptb.port_set_output_register.set(1 << 18);
        delay(10_000_000);
        ptb.port_clear_output_register.set(1 << 18);
        delay(10_000_000);
    }

}

// As we are not using interrupts, we just register a dummy catch all handler

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
