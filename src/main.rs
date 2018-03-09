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


const BASE_PTA :u32 = 0x400FF000;

pub fn delay(mut cycles: u32)
{
    while cycles > 0 {
        unsafe {
            asm!("nop" :::: "volatile");
        }
        cycles -= 1;
    }
}
#[repr(C)]
pub struct Gpio {
    pub pdor : VolatileRW<u32>,
    pub psor : VolatileRW<u32>,
    pub pcor : VolatileRW<u32>,
    pub ptor : VolatileRW<u32>,
    pub pdir : VolatileRW<u32>,
    pub pddr : VolatileRW<u32>,
}

impl Gpio {
    pub fn get(port : u32) -> &'static Gpio {
        unsafe {
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
pub struct Osc0 {
    pub cr : VolatileRW<u8>,
}

impl Osc0 {
    pub fn get() -> &'static Osc0 {
        unsafe {
            &*(BASE_SYSTEM_OSCILLATOR as *const Osc0)
        }
    }
}


pub fn system_init()
{
    let sim = SystemIntegrationModule::get();
    sim.system_clock_gating_control_register_5.bitwise_inc_or(0x0200);
    sim.system_clock_divider_register_1.set(0x10010000);
    let port_a = Port::get(0);
    port_a.pin_control_register[18].bitwise_and(!0x01000700u32);
    port_a.pin_control_register[19].bitwise_and(!0x01000700u32);
    let osc0 = Osc0::get();
    osc0.cr.set(0x89);
    let mcg = MultiPurposeClockGenerator::get();
    mcg.control_register_2.set(0x24);
    mcg.control_register_1.set(0x9A);
    mcg.control_register_4.bitwise_and_u8(!0xE0);
    mcg.control_register_5.set(0x1);
    mcg.control_register_6.set(0x0);
    while (mcg.status_register.get() & 0x10) != 0x0 {};
    while (mcg.status_register.get() & 0x0C) != 0x08 {};
    mcg.control_register_6.set(0x40);
    while (mcg.status_register.get() & 0x0C) != 0x08 {};
    while (mcg.status_register.get() & 0x40) == 0x00 {};
    mcg.control_register_1.set(0x1A);
    while (mcg.status_register.get() & 0x0C) != 0x0C {};
}

fn main() {

    system_init();
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world! F").unwrap();
    writeln!(stdout, "Hello, world! F2").unwrap();

    let sim = SystemIntegrationModule::get();
    sim.system_clock_gating_control_register_5.bitwise_inc_or(0x400);
    let portb = Port::get(1);
    portb.pin_control_register[18].set(1 << 8);

    let ptb = Gpio::get(1);
    ptb.pddr.set(1 << 18);

    ptb.psor.set(1 << 18);

    loop {
        delay(500000);
        // port toggle output register
        ptb.ptor.set(1 << 18);
    }
//    let mut a =2;
//    loop {
//        a = 3;
//    }
}

// As we are not using interrupts, we just register a dummy catch all handler

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
