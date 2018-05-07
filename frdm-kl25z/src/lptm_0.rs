/* ************************************************************ */
/* File name:        lptm.rs                                     */
/* File description: This module                                */
/*                   implements the Low Power Timer Module      */
/* Author name:      tiberioferreira                            */
/* Creation date:    04may2018                                  */
/* Revision date:    04may2018                                  */
/* ************************************************************ */
use io::VolatileRW;
use io::VolatileR;
use ::multi_purpose_clock_generator::MultiPurposeClockGenerator;
const LPTM_0_BASE : u32 = 0x4004_0000;


#[repr(C)]
pub struct Lptm0 {

    pub low_power_timer_control_status_register: VolatileRW<u32>,
    pub low_power_timer_prescale_register: VolatileRW<u32>,
    pub low_power_timer_compare_register: VolatileRW<u32>,
    pub low_power_timer_counter_register: VolatileR<u32>,
}

impl Lptm0 {
    pub (crate) fn get() -> &'static Lptm0 {
        unsafe {
            &*(LPTM_0_BASE as *const Lptm0)
        }
    }

    pub fn init(){
        ::system_integration_module::SystemIntegrationModule::enable_software_control_of_lptm();

        // Disables the LPTM
        Self::get().low_power_timer_control_status_register.clear_bit(0);

        // Set as time counter
        Self::get().low_power_timer_control_status_register.clear_bit(1);

        // Reset whenever the Compare Flag is set
        Self::get().low_power_timer_control_status_register.clear_bit(2);


        // enable interrupt
        Self::get().low_power_timer_control_status_register.set_bit(6);
        ::nvic::Nvic::enable_lptm0_interrupt();

        // Clear Timer Compare Flag
        Self::get().low_power_timer_control_status_register.set_bit(7);


        // Set prescaler clock to LPO
        Self::get().low_power_timer_prescale_register.set_bit(0);
        Self::get().low_power_timer_prescale_register.clear_bit(1);

        // Enable prescaler
        Self::get().low_power_timer_prescale_register.clear_bit(2);

        // Enable prescaler divide by 2 -> outs 500Hz
        Self::get().low_power_timer_prescale_register.clear_bit(3);
        Self::get().low_power_timer_prescale_register.clear_bit(4);
        Self::get().low_power_timer_prescale_register.clear_bit(5);
        Self::get().low_power_timer_prescale_register.clear_bit(6);

        // Set comparator to 500 so we get 1Hz clock
        Self::get().low_power_timer_compare_register.set_bit(2);
        Self::get().low_power_timer_compare_register.set_bit(4);
        Self::get().low_power_timer_compare_register.set_bit(5);
        Self::get().low_power_timer_compare_register.set_bit(6);
        Self::get().low_power_timer_compare_register.set_bit(7);
        Self::get().low_power_timer_compare_register.set_bit(8);

        // Enable timer
        Self::get().low_power_timer_control_status_register.set_bit(0);
    }

    pub fn clear_current_interrupt(){
        Self::get().low_power_timer_control_status_register.set_bit(7);
    }

}