use io::VolatileRW;
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
    pub fn get() -> &'static MultiPurposeClockGenerator {
        unsafe {
            &*(BASE_MULTI_PURPOSE_CLOCK_GENERATOR as *const MultiPurposeClockGenerator)
        }
    }
}
