#![feature(used)]
#![no_std]
#![feature(core_intrinsics)]
#![feature(asm)]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
mod io;
use io::VolatileRW;
mod multi_purpose_clock_generator;
mod system_integration_module;


pub use system_integration_module::PortLetter;
pub use system_integration_module::PortWrapper;
pub use system_integration_module::Gpio;
pub use system_integration_module::Pin;
pub use system_integration_module::Direction;
pub use system_integration_module::Value;

use system_integration_module::SystemIntegrationModule;
use core::fmt::Write;
use cortex_m::asm;
use cortex_m_semihosting::hio;


pub struct FrdmKl25zBoard{

}

pub trait FrdmKl25z{
    fn new() -> FrdmKl25zBoard;
    fn disable_watchdog_timer(&self);
    fn get_port(&self, port: PortLetter) -> PortWrapper;
    fn delay(&self, number_of_instructions: u32);
    fn delay_ms(&self, millis: u32);
    fn delay_1ms(&self);
}

impl FrdmKl25z for FrdmKl25zBoard{
    fn new() -> FrdmKl25zBoard{
        FrdmKl25zBoard{}
    }
    fn disable_watchdog_timer(&self){
        SystemIntegrationModule::disable_watchdog_timer();
    }
    fn get_port(&self, port: PortLetter) -> PortWrapper{
        SystemIntegrationModule::enable_port_for_use(port)
    }
    fn delay(&self, mut cycles: u32){
        while cycles > 0 {
            unsafe {
                asm!("nop" :::: "volatile");
            }
            cycles = cycles - 1;
        }
    }

    fn delay_ms(&self, mut millis: u32) {
        while millis > 0 {
            self.delay_1ms();
            millis = millis - 1;
        }
    }
    fn delay_1ms(&self){
        // Default freq = 20.48 mhz https://community.nxp.com/thread/311769
        // one compare
        // one nop
        // one subtraction
        // one underflow detection
        // 20480/4 = 5120
        let mut cycles = 20_480/4;
        while cycles > 0 {
            unsafe {
                asm!("nop" :::: "volatile");
            }
            cycles = cycles - 1;
        }
    }
}

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
