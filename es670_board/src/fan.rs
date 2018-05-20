pub use switch::*;
pub use led::*;
pub use display::*;
pub use frdm_kl25z::*;
pub use Es670Board;
pub use frdm_kl25z::Value::{High, Low};


impl Es670Board{
    /* ***************************************************** */
    /* Method name:        start_fan                         */
    /* Method description: starts the fan                    */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn start_fan(&self){
        let port_a = self.frdm_kl25z.get_port(PortLetter::PortA);
        let pin13 = port_a.set_pin_as_gpio(Pin::Pin13);
        pin13.set_value(Value::High);
    }

    /* ***************************************************** */
    /* Method name:        init_fan                          */
    /* Method description: initializes the TPM1 for the fan  */
    /* must only be called once                              */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn init_fan(&self){
        Tpm::init_tpm1_as_pwm();
    }

    /* ***************************************************** */
    /* Method name:        set_fan_speed                       */
    /* Method description: sets given duty for fan           */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn set_fan_speed(&self, duty_cyle: u8){
        Tpm::set_duty_cycle(duty_cyle, TpmNumber::ONE, TpmChannel::ONE);
    }


    /* ***************************************************** */
    /* Method name:        stop_fan                          */
    /* Method description: stops the fan                     */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn stop_fan(&self){
        let port_a = self.frdm_kl25z.get_port(PortLetter::PortA);
        let pin13 = port_a.set_pin_as_gpio(Pin::Pin13);
        pin13.set_value(Value::Low);
    }
}