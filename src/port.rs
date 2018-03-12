use io::VolatileRW;
use gpio::*;
#[derive(Clone)]
pub enum Ports{
    PortA = 0,
    PortB = 1,
    PortC = 2,
    PortD = 3,
    PortE = 4
}
#[derive(Clone)]
pub enum Pin{
    Pin0 = 0,
    Pin1 = 1,
    Pin2 = 2,
    Pin3 = 3,
    Pin4 = 4,
    Pin5 = 5,
    Pin6 = 6,
    Pin7 = 7,
    Pin8 = 8,
    Pin9 = 9,
    Pin10 = 10,
    Pin11 = 11,
    Pin12 = 12,
    Pin13 = 13,
    Pin14 = 14,
    Pin15 = 15,
    Pin16 = 16,
    Pin17 = 17,
    Pin18 = 18,
    Pin19 = 19,
    Pin20 = 20,
    Pin21 = 21,
    Pin22 = 22,
    Pin23 = 23,
    Pin24 = 24,
    Pin25 = 25,
    Pin26 = 26,
    Pin27 = 27,
    Pin28 = 28,
    Pin29 = 29,
    Pin30 = 30,
    Pin31 = 31
}
const BASE_PORT_A: u32 = 0x4004_9000;

pub struct PortWrapper{
    raw_port_mem: &'static Port,
    port_number: Ports
}

impl PortWrapper{
    pub fn new(port: Ports) -> PortWrapper{
        PortWrapper{
            raw_port_mem: Port::get(port.clone()),
            port_number: port
        }
    }
    pub fn set_pin_as_gpio(&self, pin: Pin) -> Gpio {
        self.raw_port_mem.pin_control_register[pin.clone() as usize].set(1 << 8);
        Gpio::new(self.port_number.clone(), pin)
    }
}
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

