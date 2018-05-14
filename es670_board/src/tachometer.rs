pub use switch::*;
pub use led::*;
pub use display::*;
pub use frdm_kl25z::*;
pub use Es670Board;
pub use frdm_kl25z::Value::{High, Low};


impl Es670Board{

    /* ***************************************************** */
    /* Method name:             tachometer_start_counter     */
    /* Method description:      starts tachometer counter    */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn tachometer_start_counter(&self){
        self.frdm_kl25z.init_tpm0_ch0_as_hardware_counter();
    }


    /* ***************************************************** */
    /* Method name:     tachometer_counter_get_current_value */
    /* Method description: gets the tachometer counter value */
    /*                     current value                     */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn tachometer_counter_get_current_value(&self) -> u16{
        self.frdm_kl25z.tmp0_ch0_get_current_value()/7
    }

    /* ***************************************************** */
    /* Method name:     tachometer_counter_reset             */
    /* Method description: resets the tachometer counter     */
    /*                     current value                     */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn tachometer_counter_reset(&self){
        self.frdm_kl25z.tmp0_ch0_reset_counter();
    }



}