use io::VolatileRW;
use system_integration_module::*;
const BASE_MULTI_PURPOSE_CLOCK_GENERATOR: u32 = 0x4006_4000;

#[repr(C)]
pub struct MultiPurposeClockGenerator {
    pub control_register_1: VolatileRW<u8>,
    pub control_register_2: VolatileRW<u8>,
    pub control_register_3: VolatileRW<u8>,
    pub control_register_4: VolatileRW<u8>,
    pub control_register_5: VolatileRW<u8>,
    pub control_register_6: VolatileRW<u8>,
    pub status_register: VolatileRW<u8>,
    pub reserved_0 : [u8; 1],
    pub status_n_control_register: VolatileRW<u8>,
    pub reserved_1 : [u8; 1],
    pub auto_trim_compare_value_high_register: VolatileRW<u8>,
    pub auto_trim_compare_value_low_register: VolatileRW<u8>,
    pub control_register_7: VolatileRW<u8>,
    pub control_register_8: VolatileRW<u8>,
    pub control_register_9: VolatileRW<u8>,
    pub control_register_10: VolatileRW<u8>,
}

impl MultiPurposeClockGenerator {
    pub (crate) fn get() -> &'static MultiPurposeClockGenerator {
        unsafe {
            &*(BASE_MULTI_PURPOSE_CLOCK_GENERATOR as *const MultiPurposeClockGenerator)
        }
    }


    pub fn mcg_clock_init() {
        let port_a = SystemIntegrationModule::enable_port_for_use(PortLetter::PortA);
        SystemIntegrationModule::enable_port_for_use(PortLetter::PortC);
        SystemIntegrationModule::enable_port_for_use(PortLetter::PortE);

        ::smc::Smc::enable_all_modes();
        port_a.set_pin_as_disabled(Pin::Pin18);
        port_a.set_pin_as_disabled(Pin::Pin19);
    }

    pub fn osc_is_ok() -> bool{
        Self::get().status_register.get_bit(1)
    }


}
