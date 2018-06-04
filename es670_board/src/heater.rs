pub use switch::*;
pub use led::*;
pub use display::*;
pub use frdm_kl25z::*;
pub use Es670Board;
pub use frdm_kl25z::Value::{High, Low};


impl Es670Board{


    /* ***************************************************** */
    /* Method name:        set_heater_intensity              */
    /* Method description: sets given duty cycle for heater  */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn set_heater_intensity(&self, duty_cyle: u8){
        Tpm::set_duty_cycle(duty_cyle, TpmNumber::ONE, TpmChannel::ZERO);
    }




}