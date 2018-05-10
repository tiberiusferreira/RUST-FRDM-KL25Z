/* ************************************************************ */
/* File name:        smc.rs                                     */
/* File description: This module                                */
/*                   implements the System Mode Controler       */
/* Author name:      tiberioferreira                            */
/* Creation date:    05may2018                                  */
/* Revision date:    05may2018                                  */
/* ************************************************************ */
use io::VolatileRW;
const SMC_BASE : u32 = 0x4007_E000;


#[repr(C)]
pub struct Smc {
    pub power_mode_protection_register: VolatileRW<u8>,
    pub power_mode_control_register: VolatileRW<u8>,
    pub stop_control_register: VolatileRW<u8>,
    pub power_mode_status_register: VolatileRW<u8>,
}

impl Smc {
    pub (crate) fn get() -> &'static Smc {
        unsafe {
            &*(SMC_BASE as *const Smc)
        }
    }

    pub fn enable_all_modes(){
        Self::get().power_mode_protection_register.set(0b00_1_0_1_0_1_0);
    }

}