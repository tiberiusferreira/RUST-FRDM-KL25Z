/* ************************************************************ */
/* File name:        tpm.rs                                     */
/* File description: This module                                */
/*                   implements the TPM module                  */
/* Author name:      tiberioferreira                            */
/* Creation date:    05may2018                                  */
/* Revision date:    05may2018                                  */
/* ************************************************************ */
use io::VolatileRW;
use system_integration_module::*;
const TPM0_BASE : u32 = 0x4003_8000;

#[derive(Clone, Copy)]
pub enum TpmNumber{
    ZERO = 0,
    ONE = 1,
    TWO = 2
}

#[derive(Clone, Copy)]
pub enum TpmChannel{
    ZERO = 0,
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5
}

#[repr(C)]
pub struct Tpm {
    pub status_and_control: VolatileRW<u32>,
    pub counter: VolatileRW<u32>,
    pub modulo: VolatileRW<u32>,
    pub channel_0_status_and_control: VolatileRW<u32>,
    pub channel_0_value: VolatileRW<u32>,
    pub channel_1_status_and_control: VolatileRW<u32>,
    pub channel_1_value: VolatileRW<u32>,
    pub channel_2_status_and_control: VolatileRW<u32>,
    pub channel_2_value: VolatileRW<u32>,
    pub channel_3_status_and_control: VolatileRW<u32>,
    pub channel_3_value: VolatileRW<u32>,
    pub channel_4_status_and_control: VolatileRW<u32>,
    pub channel_4_value: VolatileRW<u32>,
    pub channel_5_status_and_control: VolatileRW<u32>,
    pub channel_5_value: VolatileRW<u32>,
}

impl Tpm {
    pub fn init_tpm1_as_pwm(){
        let tpm_number = TpmNumber::ONE;
        Self::init_clock_as_oscerclk(tpm_number);

        // Select PORTA PIN13 as PWM for TPM1 CH1
        let port_a = SystemIntegrationModule::enable_port_for_use(PortLetter::PortA);
        port_a.set_pin_as_alt3(Pin::Pin13);

        // Prescale 1:1 only writtable when counter is disabled
        Self::get(tpm_number).status_and_control.clear_bit(0);
        Self::get(tpm_number).status_and_control.clear_bit(1);
        Self::get(tpm_number).status_and_control.clear_bit(2);

        // Center-aligned PWM Select Only writtable when counter is disabled
        Self::get(tpm_number).status_and_control.clear_bit(5);

        // Freq = 8MHz / MOD = 8*10^6/16385 16385=max value for 16 bits
        // Freq = 488 Hz Period = 2ms
        Self::get(tpm_number).modulo.set(0xFFFF);

        // Increase counter on every clock
        Self::get(tpm_number).status_and_control.set_bit(3);

        // Set as egde aligned high-true pulses
        Self::get(tpm_number).channel_1_status_and_control.bitwise_inc_or(0b0010_1000);

        Self::get(tpm_number).channel_1_value.set(0x0);
    }

    pub fn set_duty_cycle(duty_percentage_0_to_100: u8, which_tpm: TpmNumber, which_channel: TpmChannel){
        if duty_percentage_0_to_100 > 100 || duty_percentage_0_to_100 < 0{
            return;
        }
        let max_counter_value = Self::get(which_tpm).modulo.get();
        let match_value = ((max_counter_value as f64)*((duty_percentage_0_to_100 as f64/100.0) as f64))as u32;
        match which_channel {
            TpmChannel::ZERO => {
                Self::get(which_tpm).channel_0_value.set(match_value);
            },
            TpmChannel::ONE => {
                Self::get(which_tpm).channel_1_value.set(match_value);
            },
            TpmChannel::TWO => {
                Self::get(which_tpm).channel_2_value.set(match_value);
            },
            TpmChannel::THREE => {
                Self::get(which_tpm).channel_3_value.set(match_value);
            },
            TpmChannel::FOUR => {
                Self::get(which_tpm).channel_4_value.set(match_value);
            },
            TpmChannel::FIVE => {
                Self::get(which_tpm).channel_5_value.set(match_value);
            }
        }


    }

    pub (crate) fn get(which_tpm: TpmNumber) -> &'static Tpm {
        let tpm_address = (which_tpm as u32)*0x1000 + TPM0_BASE;
        unsafe {
            &*(tpm_address as *const Tpm)
        }
    }

    pub fn init_clock_as_oscerclk(which_tpm: TpmNumber){
        SystemIntegrationModule::enable_tpm_and_oscilator_clock(which_tpm);
        SystemIntegrationModule::select_tpm_clock_as_oscerclk();
    }

    pub fn init_tpm_0_ch_0_using_clkin0_as_hardware_counter(){
        // PortE Pin29 is used as CLKIN0
        let port_e = SystemIntegrationModule::enable_port_for_use(PortLetter::PortE);
        port_e.set_pin_as_alt4(Pin::Pin29);

        Self::init_clock_as_oscerclk(TpmNumber::ZERO);
        SystemIntegrationModule::set_tpm0_external_clock_to_clkin0();


        // Prescale 1:1
        Self::get(TpmNumber::ZERO).status_and_control.clear_bit(0);
        Self::get(TpmNumber::ZERO).status_and_control.clear_bit(1);
        Self::get(TpmNumber::ZERO).status_and_control.clear_bit(2);

        // Set LPTPM to count on every clock syncronized with external_clock
        Self::get(TpmNumber::ZERO).status_and_control.set_bit(4);
        Self::get(TpmNumber::ZERO).status_and_control.clear_bit(3);

        // Set as software compare
        Self::get(TpmNumber::ZERO).channel_0_status_and_control.set_bit(5);
        Self::get(TpmNumber::ZERO).channel_0_status_and_control.set_bit(4);
        Self::get(TpmNumber::ZERO).channel_0_status_and_control.clear_bit(3);
        Self::get(TpmNumber::ZERO).channel_0_status_and_control.clear_bit(2);
    }

    pub fn clear_current_interrupt(which_tpm: TpmNumber){
        while Self::get(which_tpm).channel_0_status_and_control.get_bit(7){
            Self::get(which_tpm).channel_0_status_and_control.set_bit(7);
        }
        Self::get(which_tpm).status_and_control.set_bit(7);
    }

    pub fn get_counter(which_tpm: TpmNumber) -> u32{
        Self::get(which_tpm).counter.get()
    }

    pub fn clear_counter(which_tpm: TpmNumber){
        Self::get(which_tpm).counter.set(0);
    }

    pub fn get_ch1_value(which_tpm: TpmNumber) -> u32{
        Self::get(which_tpm).channel_1_value.get()
    }

    pub fn get_tof(which_tpm: TpmNumber) -> bool{
        Self::get(which_tpm).status_and_control.get_bit(7)
    }

    pub fn clear_tof(which_tpm: TpmNumber){
        Self::get(which_tpm).status_and_control.set_bit(7);
        Self::get(which_tpm).counter.set(0);
    }


}