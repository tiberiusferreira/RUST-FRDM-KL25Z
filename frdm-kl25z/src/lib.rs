#![feature(used)]
#![no_std]
#![feature(core_intrinsics)]
#![feature(asm)]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
pub mod io;
pub use io::*;
mod multi_purpose_clock_generator;
mod system_integration_module;
mod uart_0;
mod nvic;

pub use uart_0::Uart_0;
pub use system_integration_module::PortLetter;
pub use system_integration_module::PortWrapper;
pub use system_integration_module::Gpio;
pub use system_integration_module::Pin;
pub use system_integration_module::Direction;
pub use system_integration_module::Value;

use system_integration_module::SystemIntegrationModule;


pub struct FrdmKl25zBoard{

}

pub trait FrdmKl25z{
    fn new() -> FrdmKl25zBoard;
    fn disable_watchdog_timer(&self);
    fn get_port(&self, port: PortLetter) -> PortWrapper;
    fn delay_ms(&self, millis: u32);
    fn enable_uart0(&self, baud_rate: u32);
    fn enable_rx_interrupts(&self);
    fn send_char(&self, c: char);
    fn send_string(&self, s: &str);
}

impl FrdmKl25zBoard {
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


    fn delay_ms(&self, mut millis: u32) {
        while millis > 0 {
            self.delay_1ms();
            millis = millis - 1;
        }
    }
    fn enable_uart0(&self, baud_rate: u32) {
        Uart_0::enable_uart(baud_rate);
    }

    fn enable_rx_interrupts(&self) {
        Uart_0::enable_rx_interrupts();
    }

    fn send_char(&self, c: char) {
        Uart_0::send_char(c);
    }

    fn send_string(&self, s: &str) {
        Uart_0::send_string(s);
    }
}
