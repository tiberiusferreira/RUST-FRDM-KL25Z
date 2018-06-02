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

// Heater =  TPM 1  CH 0
// Fan =  TPM 1  CH 1
impl Tpm {
    /* ***************************************************** */
    /* Method name:        init_tpm1_ch0_n_ch1_as_pwm        */
    /* Method description: initializes the tpm1 module       */
    /*                     channel 0 and channel 1 as PWM    */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn init_tpm1_ch0_n_ch1_as_pwm(){
        let tpm_number = TpmNumber::ONE;
        Self::init_clock_as_oscerclk(tpm_number);

        // Select PORTA PIN13 as PWM for TPM1 CH1
        let port_a = SystemIntegrationModule::enable_port_for_use(PortLetter::PortA);
        port_a.set_pin_as_alt3(Pin::Pin13);

        // Select PORTA PIN12 as PWM for TPM1 CH0
        port_a.set_pin_as_alt3(Pin::Pin12);

        // Prescale 1:1 only writable when counter is disabled
        Self::get(tpm_number).status_and_control.clear_bit(0);
        Self::get(tpm_number).status_and_control.clear_bit(1);
        Self::get(tpm_number).status_and_control.clear_bit(2);

        // Center-aligned PWM Select Only writable when counter is disabled
        Self::get(tpm_number).status_and_control.clear_bit(5);

        // Freq = 8MHz / MOD = 8*10^6/65535 65535=max value for 16 bits
        // Freq = 122 Hz Period = 8ms
        Self::get(tpm_number).modulo.set(0xFFFF);
//        Self::get(tpm_number).modulo.set(1000);

        // Increase counter on every clock
        Self::get(tpm_number).status_and_control.set_bit(3);

        // Set as egde aligned high-true pulses
        Self::get(tpm_number).channel_1_status_and_control.bitwise_inc_or(0b0010_1000);

        Self::get(tpm_number).channel_1_value.set(0x0);

        Self::get(tpm_number).channel_0_status_and_control.bitwise_inc_or(0b0010_1000);

        Self::get(tpm_number).channel_0_value.set(0x0);
    }


    /* ***************************************************** */
    /* Method name:        change_freq_tpm1_ch_1_pwm         */
    /* Method description: sets the PWM frequency            */
    /* Input params: freq_percentage_0_to_100 the PWM        */
    /*             frequency from 0 to 100 percent           */
    /*             of the maximum which is around 488Hz      */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn change_freq_tpm1_pwm(freq_percentage_0_to_100: u8){
        if freq_percentage_0_to_100 > 100{
            return;
        }
        let old_module = Self::get(TpmNumber::ONE).modulo.get();
        let old_cnv_ch1 =  Self::get(TpmNumber::ONE).channel_1_value.get();
        let old_cnv_ch0 =  Self::get(TpmNumber::ONE).channel_0_value.get();
        let old_duty_ch1 = ((((old_cnv_ch1 as f64) / (old_module as f64)) as f64)*(100.0 as f64)) as u8;
        let old_duty_ch0 = ((((old_cnv_ch0 as f64) / (old_module as f64)) as f64)*(100.0 as f64)) as u8;
        let new_module = ((((freq_percentage_0_to_100 as f32)/(100.0 as f32)) as f32) * ((0xFFFF as u32) as f32)) as u32;
        Self::get(TpmNumber::ONE).modulo.set(new_module);
        ::FrdmKl25zBoard::delay_1ms(); // wait for it to sync with the counter modules
        Self::set_duty_cycle(old_duty_ch1, TpmNumber::ONE, TpmChannel::ONE);
        Self::set_duty_cycle(old_duty_ch0, TpmNumber::ONE, TpmChannel::ZERO);
    }

    /* ***************************************************** */
    /* Method name:        set_duty_cycle                    */
    /* Method description: sets the duty cycle of the given  */
    /*                     pwm and channel                   */
    /* Input params: the duty cycle = duty_percentage_0_to_100*/
    /*             which_tpm = which tpm to act on           */
    /*            which_channel = which tpm channel to act on*/
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn set_duty_cycle(duty_percentage_0_to_100: u8, which_tpm: TpmNumber, which_channel: TpmChannel){
        if duty_percentage_0_to_100 > 100{
            return;
        }
        let max_counter_value = Self::get(which_tpm).modulo.get();
        let match_value = ((max_counter_value as f64)*(duty_percentage_0_to_100 as f64/(100.0 as f64)))as u32;
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

    fn init_clock_as_oscerclk(which_tpm: TpmNumber){
        SystemIntegrationModule::enable_tpm_and_oscilator_clock(which_tpm);
        SystemIntegrationModule::select_tpm_clock_as_oscerclk();
    }

    /* ****************************************************************************** */
    /* Method name:        init_tpm_0_ch_0_using_clkin0_as_hardware_counter           */
    /* Method description: initializes the tpm 0 channel 0 as hardware counter        */
    /* with clkin0 as external clock                                                  */
    /* Input params:                                                                  */
    /* Output params:                                                                 */
    /* ****************************************************************************** */
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

    /* ***************************************************** */
    /* Method name:        clear_current_interrupt           */
    /* Method description: clears the current interrupt      */
    /* Input params:    which_tpm = which tpm to act on      */
    /* Output params:                                        */
    /* ***************************************************** */
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