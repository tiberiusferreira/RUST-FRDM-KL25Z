/* ************************************************************ */
/* File name:        tpm.rs                                     */
/* File description: This module                                */
/*                   implements the Low Power Timer Module      */
/* Author name:      tiberioferreira                            */
/* Creation date:    05may2018                                  */
/* Revision date:    05may2018                                  */
/* ************************************************************ */
use io::VolatileRW;
use multi_purpose_clock_generator::MultiPurposeClockGenerator;
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
        ::multi_purpose_clock_generator::MultiPurposeClockGenerator::mcg_clock_init();
        // Enable OSCERCLK
        Self::get().control_register.set_bit(5);
        Self::get().control_register.set_bit(7);

        // Very high range
        MultiPurposeClockGenerator::get().control_register_2.set_bit(5);
        MultiPurposeClockGenerator::get().control_register_2.set_bit(4);

        // Hgo gain low
        MultiPurposeClockGenerator::get().control_register_2.clear_bit(3);

        // Request OSC
        MultiPurposeClockGenerator::get().control_register_2.set_bit(2);
    }

}