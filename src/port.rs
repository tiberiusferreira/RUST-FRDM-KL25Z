use io::VolatileRW;

pub enum Ports{
    PortA = 0,
    PortB = 1,
    PortC = 2,
    PortD = 3,
    PortE = 4
}
const BASE_PORT_A: u32 = 0x4004_9000;

#[repr(C)]
pub struct Port {
    pub pin_control_register: [VolatileRW<u32>; 32],
    pub global_pin_low_register: VolatileRW<u32>,
    pub global_pin_high_register: VolatileRW<u32>,
    pub reserved_0 : [u8; 24],
    pub interrupt_status_flag_register: VolatileRW<u32>,
}

// Ports A B C D E map to memory regions which are 0x1000 of distance from each other
impl Port {
    pub fn get(port: Ports) -> &'static Port {
        unsafe {
            &*((BASE_PORT_A + ((port as u32)*0x1000)) as *const Port)
        }
    }
}
