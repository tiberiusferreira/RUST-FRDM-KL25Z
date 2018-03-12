use io::VolatileRW;
// Accessing GPIOs through the cross bar/AIPS interface
const BASE_PTA :u32 = 0x400F_F000;


#[repr(C)]
pub struct Gpio {
    // Actual value of the GPIO
    pub port_data_output_register: VolatileRW<u32>,

    // Whether to set it or not
    pub port_set_output_register: VolatileRW<u32>,

    // Whether to clear it or not
    pub port_clear_output_register: VolatileRW<u32>,

    // Whether to invert it or not
    pub port_toggle_output_register: VolatileRW<u32>,

    // Value of the GPIO when
    pub port_data_input_register: VolatileRW<u32>,

    // Direction of the GPIO: 0 = input // 1 = output
    pub port_data_direction_register: VolatileRW<u32>,
}

impl Gpio {
    pub fn get(port : u32) -> &'static Gpio {
        unsafe {
            // From 0-A 1-B 2-C 3-D 4-E each one takes 0x40 and they are mapped in sequence
            &*((BASE_PTA + (port*0x40)) as *const Gpio)
        }
    }
}

