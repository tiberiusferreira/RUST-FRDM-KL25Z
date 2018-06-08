/* ************************************************************ */
/* File name:        adc.rs                                     */
/* File description: This module                                */
/*                   implements the Analog to Digital Converter */
/* Author name:      tiberioferreira                            */
/* Creation date:    28may2018                                  */
/* Revision date:    28may2018                                  */
/* ************************************************************ */
use io::VolatileRW;
use io::VolatileR;
use system_integration_module::*;
const ADC_BASE : u32 = 0x4003_B000;
const ADICLK: u32 = 0b01; // bus/2 clock selection
const MODE: u32 = 0b00 << 2; // 8-bit Conversion mode selection
const ADLSMP: u32 = 0b0 << 4; // Short sample time configuration
const ADIV: u32 = 0b00 << 5; // Clock Divide Select (Divide by 1)
const ADLPC: u32 = 0b0 << 7; // Normal power Configuration


#[repr(C)]
pub struct Adc {

    pub status_and_control_1_a: VolatileRW<u32>,
    pub status_and_control_1_b: VolatileRW<u32>,
    pub configuration_register_1: VolatileRW<u32>,
    pub configuration_register_2: VolatileRW<u32>,
    pub result_a: VolatileR<u32>,
    pub result_b: VolatileR<u32>,
    pub compare_value_1: VolatileR<u32>,
    pub compare_value_2: VolatileR<u32>,
    pub status_and_control_2: VolatileRW<u32>,
    pub status_and_control_3: VolatileRW<u32>,
    pub offset_correction: VolatileRW<u32>,
    pub plus_side_gain: VolatileRW<u32>,
    pub minus_side_gain: VolatileRW<u32>,
}

impl Adc {
    pub (crate) fn get() -> &'static Adc {
        unsafe {
            &*(ADC_BASE as *const Adc)
        }
    }

    pub fn init_adc(){
        SystemIntegrationModule::enable_adc_clock();
        let port_e = SystemIntegrationModule::enable_port_for_use(PortLetter::PortE);

        /* set pin as ADC In */
        port_e.set_pin_as_disabled(Pin::Pin20); // Voltage Sensor
        port_e.set_pin_as_disabled(Pin::Pin21); // Temperature Sensor

        Self::get().configuration_register_1.set(ADICLK | MODE | ADLSMP | ADIV | ADLPC);

        Self::get().status_and_control_2.clear_bit(0);
        Self::get().status_and_control_2.clear_bit(1);
        Self::get().status_and_control_2.clear_bit(2);
        Self::get().status_and_control_2.clear_bit(3);
        Self::get().status_and_control_2.clear_bit(4);
        Self::get().status_and_control_2.clear_bit(5);
        Self::get().status_and_control_2.clear_bit(6);

        Self::get().configuration_register_2.clear_bit(0);
        Self::get().configuration_register_2.clear_bit(1);
        Self::get().configuration_register_2.clear_bit(2);
        Self::get().configuration_register_2.clear_bit(3);
        Self::get().configuration_register_2.clear_bit(4);

    }

    pub fn init_conversion(){
        Self::get().status_and_control_1_a.set(0b00_00100);
    }

    pub fn conversion_is_done() -> bool{
        Self::get().status_and_control_1_a.get_bit(7)
    }

    pub fn get_result() -> u16{
        Self::get().result_a.get() as u16
    }

}