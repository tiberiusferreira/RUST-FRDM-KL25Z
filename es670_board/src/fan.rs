pub use switch::*;
pub use led::*;
pub use display::*;
pub use frdm_kl25z::*;
pub use Es670Board;
pub use frdm_kl25z::Value::{High, Low};


impl Es670Board{
    /* ***************************************************** */
    /* Method name:        start_fan_as_gpio                 */
    /* Method description: starts the fan as a GPIO          */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn start_fan_as_gpio(&self){
        let port_a = self.frdm_kl25z.get_port(PortLetter::PortA);
        let pin13 = port_a.set_pin_as_gpio(Pin::Pin13);
        pin13.set_value(Value::High);
    }

    /* ***************************************************** */
    /* Method name:        stop_fan_as_gpio                  */
    /* Method description: stops the fan when started as GPIO*/
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn stop_fan_as_gpio(&self){
        let port_a = self.frdm_kl25z.get_port(PortLetter::PortA);
        let pin13 = port_a.set_pin_as_gpio(Pin::Pin13);
        pin13.set_value(Value::Low);
    }

    /* ***************************************************** */
    /* Method name:        init_fan_n_heater_as_pwm          */
    /* Method description: initializes the TPM1 as PWM for   */
    /* controlling the fan. Must only be called once         */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn init_fan_n_heater_as_pwm(&self){
        Tpm::init_tpm1_ch0_n_ch1_as_pwm();
        // Make sure changes take effect before using it
        self.delay(10);
    }

    /* ***************************************************** */
    /* Method name:        set_fan_speed                     */
    /* Method description: sets given duty cycle for fan     */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn set_fan_speed(&self, duty_cyle: u8){
        Tpm::set_duty_cycle(duty_cyle, TpmNumber::ONE, TpmChannel::ONE);
    }



    /* ***************************************************** */
    /* Method name:        set_fan_n_heater_pwm_freq         */
    /* Method description: sets given pwm frequency for fan  */
    /*                     and heater                        */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn set_fan_n_heater_pwm_freq(&self, freq_percentage_from_0_to_100: u8){
        Tpm::change_freq_tpm1_pwm(freq_percentage_from_0_to_100);
    }


}