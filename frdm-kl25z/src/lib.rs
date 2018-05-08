/* ************************************************************ */
/* File name:        lib.rs                                     */
/* File description: This module                                */
/*                   implements the FrdmKl25zBoard board        */
/*                   functionality                              */
/* Author name:      tiberioferreira                            */
/* Creation date:    14abr2018                                  */
/* Revision date:    23abr2018                                  */
/* ************************************************************ */
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
mod lptm_0;
mod system_integration_module;
mod uart_0;
mod nvic;
mod tpm;
mod osc;

pub use uart_0::Uart0;
pub use tpm::Tpm0;
pub use multi_purpose_clock_generator::MultiPurposeClockGenerator;
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
    /* ***************************************************** */
    /* Method name:        new                               */
    /* Method description: Creates a new FrdmKl25zBoard      */
    /*                     instance                          */
    /* Input params:                                         */
    /* Output params:      FrdmKl25zBoard instance           */
    /* ***************************************************** */
    fn new() -> FrdmKl25zBoard;

    /* ***************************************************** */
    /* Method name:        disable_watchdog_timer            */
    /* Method description: disables the watchdog timer       */
    /*                     instance                          */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    fn disable_watchdog_timer(&self);

    /* ***************************************************** */
    /* Method name:        enable_low_power_timer            */
    /* Method description: enable the low powertimer         */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    fn enable_low_power_timer(&self);


    /* ***************************************************** */
    /* Method name:        clear_lptm_interrupt              */
    /* Method description: clears the current lptm interrupt */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    fn clear_lptm_interrupt();

    /* ***************************************************** */
    /* Method name:        clear_tpm0_interrupt              */
    /* Method description: clears the current tpm0 interrupt */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    fn clear_tpm0_interrupt();


    /* ***************************************************** */
    /* Method name:        get_port                          */
    /* Method description: returns a PortWrapper so a port   */
    /*                     can be used                       */
    /* Input params:       port: which port to be initialized*/
    /*                     and wrapped                       */
    /* Output params:      the port wrapper                  */
    /* ***************************************************** */
    fn get_port(&self, port: PortLetter) -> PortWrapper;

    /* ***************************************************** */
    /* Method name:        delay_ms                          */
    /* Method description: busy waits                        */
    /* Input params:       millis: how long to wait in       */
    /*                     milliseconds                      */
    /* Output params:                                        */
    /* ***************************************************** */
    fn delay_ms(&self, millis: u32);
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
        Self::delay_ms(300);
        FrdmKl25zBoard{

        }
    }
    fn disable_watchdog_timer(&self){
        SystemIntegrationModule::disable_watchdog_timer();
    }
    fn enable_low_power_timer(&self) {
        lptm_0::Lptm0::init();
    }


    fn clear_lptm_interrupt(){
        lptm_0::Lptm0::clear_current_interrupt();
    }

    fn clear_tpm0_interrupt(){
        tpm::Tpm0::clear_current_interrupt();
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
}
