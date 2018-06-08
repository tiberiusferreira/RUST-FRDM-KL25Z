pub use switch::*;
pub use led::*;
pub use display::*;
pub use frdm_kl25z::*;
pub use Es670Board;
pub use frdm_kl25z::Value::{High, Low};

/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
*                   TABELA PARA USO DO SENSOR DE TEMPERATURA            *
* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
const TABELA_TEMP :[u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,					//15
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,					//31
    1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 5, 5, 6, 6, 6, 6,					//47
    7, 7, 8, 8, 8, 8, 9, 9, 10, 10, 10, 10, 11, 11, 12, 12,			//63
    12, 12, 13, 13, 14, 14, 15, 15, 15, 15, 16, 16, 16, 17, 17, 17,	//79
    17, 18, 18, 19, 19, 19, 19, 20, 20, 21, 21, 21, 21, 22, 22, 23,	//95
    23, 24, 24, 24, 24, 25, 25, 26, 26, 26, 26, 27, 27, 28, 28, 28,	//111
    28, 29, 29, 30, 30, 30, 30, 31, 31, 32, 32, 32, 32, 33, 33, 34,	//127
    34, 35, 35, 35, 35, 36, 36, 37, 37, 37, 37, 38, 38, 39, 39, 39,	//143
    39, 40, 40, 41, 41, 41, 41, 42, 42, 43, 43, 44, 44, 44, 44, 45,	//159
    45, 46, 46, 46, 46, 47, 47, 48, 48, 48, 48, 49, 49, 50, 50, 50,	//175
    50, 51, 51, 52, 52, 53, 53, 53, 53, 54, 54, 55, 55, 55, 55, 56,	//191
    56, 57, 57, 57, 57, 58, 58, 59, 59, 59, 59, 60, 60, 61, 61, 62,	//207
    62, 62, 62, 63, 63, 64, 64, 64, 64, 65, 65, 66, 66, 66, 66, 67,	//223
    67, 68, 68, 68, 68, 69, 69, 70, 70, 71, 71, 71, 71, 72, 72, 72,	//239
    73, 73, 73, 73, 74, 74, 75, 75, 75, 75, 76, 76, 77, 77, 77, 77	//255
];
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

    /* ***************************************************** */
    /* Method name:        get_heater_temp                   */
    /* Method description: returns the heater ADC raw value  */
    /*                      and temp in Celsius              */
    /* Input params:                                         */
    /* Output params: the temperature in Celsius             */
    /* ***************************************************** */
    pub fn get_heater_temp(&mut self) -> (u16, u8){
        if !self.adc_inited{
            Adc::init_adc();
            self.adc_inited = true;
        }
        Adc::init_conversion();
        while !Adc::conversion_is_done() {

        }
        let adc_raw = Adc::get_result() ;
        (adc_raw, TABELA_TEMP[adc_raw as usize])
    }




}