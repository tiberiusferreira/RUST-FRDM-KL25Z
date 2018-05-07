/* ************************************************************ */
/* File name:        tpm.rs                                     */
/* File description: This module                                */
/*                   implements the Low Power Timer Module      */
/* Author name:      tiberioferreira                            */
/* Creation date:    05may2018                                  */
/* Revision date:    05may2018                                  */
/* ************************************************************ */
use io::VolatileRW;
const OSC_BASE : u32 = 0x4006_5000;


#[repr(C)]
pub struct Osc {

    pub control_register: VolatileRW<u8>,
}

impl Osc {
    pub (crate) fn get() -> &'static Osc {
        unsafe {
            &*(OSC_BASE as *const Osc)
        }
    }

    pub fn init(){
        ::multi_purpose_clock_generator::MultiPurposeClockGenerator::use_osc_as_external_ref_clock();
        // Enable OSCERCLK
        Self::get().control_register.set_bit(5);
        Self::get().control_register.set_bit(7);
    }

}