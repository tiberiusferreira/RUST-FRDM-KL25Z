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
const TPM_BASE : u32 = 0x4003_8000;


#[repr(C)]
pub struct Tpm0 {

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

impl Tpm0 {
    pub (crate) fn get() -> &'static Tpm0 {
        unsafe {
            &*(TPM_BASE as *const Tpm0)
        }
    }

    pub fn init(){
        // PortE Pin29 is used as CLKIN0
        let port_e = SystemIntegrationModule::enable_port_for_use(PortLetter::PortE);
        port_e.set_pin_as_alt4(Pin::Pin29);
        // Port A is where the external 8 MHz clock is connected
        let port_a = SystemIntegrationModule::enable_port_for_use(PortLetter::PortA);

        ::system_integration_module::SystemIntegrationModule::enable_tpm0_clock();
//        ::system_integration_module::SystemIntegrationModule::select_tpm0_clock_as_mcgfllclk();
        ::system_integration_module::SystemIntegrationModule::select_tpm0_clock_as_oscerclk();
        ::system_integration_module::SystemIntegrationModule::set_tpm0_clock_to_clkin0();


        // Disable counter
        Self::get().status_and_control.clear_bit(4);
        Self::get().status_and_control.clear_bit(3);

        // Disable Channel 0
        Self::get().channel_0_status_and_control.clear_bit(5);
        Self::get().channel_0_status_and_control.clear_bit(4);
        Self::get().channel_0_status_and_control.clear_bit(3);
        Self::get().channel_0_status_and_control.clear_bit(2);

        // Clear the Counter
//        Self::get().counter.set(0);

        // Set mod to 1
        Self::get().modulo.set(1);

        // Set counter in up mode
        Self::get().status_and_control.clear_bit(5);

        // Prescale 1:1
        Self::get().status_and_control.clear_bit(0);
        Self::get().status_and_control.clear_bit(1);
        Self::get().status_and_control.clear_bit(2);

        // Overflow interrupt
        Self::get().status_and_control.set_bit(6);

        // Clear the Counter
//        Self::get().counter.set(0);




        // Enable interrupts on channel 0
        Self::get().channel_0_status_and_control.set_bit(6);
        ::nvic::Nvic::enable_tpm0_interrupt();

        // Set LPTPM to count on every clock syncronized with external_clock
        Self::get().status_and_control.set_bit(4);
        Self::get().status_and_control.clear_bit(3);

        // Set as software compare
        Self::get().channel_0_status_and_control.set_bit(5);
        Self::get().channel_0_status_and_control.set_bit(4);
        Self::get().channel_0_status_and_control.clear_bit(3);
        Self::get().channel_0_status_and_control.clear_bit(2);


        // Clear the Counter
//        Self::get().counter.set(0);



        // Clear the Counter
//        Self::get().counter.set(0);
    }

    pub fn clear_current_interrupt(){
        while Self::get().channel_0_status_and_control.get_bit(7){
            Self::get().channel_0_status_and_control.set_bit(7);
        }
        Self::get().status_and_control.set_bit(7);
    }

    pub fn get_counter() -> u32{
        Self::get().counter.get()
    }

    pub fn get_tof() -> bool{
        Self::get().status_and_control.get_bit(7)
    }

    pub fn clear_tof(){
        Self::get().status_and_control.set_bit(7);
        Self::get().counter.set(0);
    }

    /*
    4 - Para a configuração do contador de pulsos, deverão ser utilizadas as seguintes configurações:
    - Será utilizado o TPM0
    - CLKIN0
    - Prescale 1:1 OK
    - LPTPM_EXTCLK. External Clock Source OK
    - TPM Source OSCERCLK clock

    5 - Leituras exigidas do DATASHEET:
    - Capítulo 12 SIM (System Integration Module). Verificar implementações necessárias nos registradores SCGCx e SOPTx.
    - Capítulo 31 Timer/PWM Module. Verificar implementações do módulo TPM0.
    */

}