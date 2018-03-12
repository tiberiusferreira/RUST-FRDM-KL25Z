use io::VolatileRW;
use port::*;
// Accessing GPIOs through the cross bar/AIPS interface
const BASE_PTA :u32 = 0x400F_F000;

pub enum Direction {
    In,
    Out
}

pub enum Value{
    High,
    Low
}
fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}


pub struct Gpio{
    port: Ports,
    pin: Pin,
    port_gpios: &'static PortGpios
}

impl Gpio{
    pub fn new(port: Ports, pin: Pin) -> Gpio{
        Gpio{
            port: port.clone(),
            pin,
            port_gpios: PortGpios::get(port)
        }
    }
    fn get_pin_bit(&self, all_pins_data: u32) -> bool{
        get_bit_at(all_pins_data, self.pin.clone() as u8)
    }
    pub fn get_pin_direction(&self) -> Direction {
        match self.get_pin_bit(self.port_gpios.port_data_direction_register.get()){
            false => {
                Direction::In
            },
            true => {
                Direction::Out
            }
        }
    }
    pub fn get_value(&self) -> Value{
        match self.get_pin_direction() {
            Direction::In => {
                match self.get_pin_bit(self.port_gpios.port_data_input_register.get()){
                    false => Value::Low,
                    true => Value::High
                }
            },
            Direction::Out => {
                match self.get_pin_bit(self.port_gpios.port_data_output_register.get()){
                    false => Value::Low,
                    true => Value::High
                }
            }
        }
    }

    pub fn set_value(&self, value: Value){
        self.set_direction(Direction::Out);
        match value {
            Value::High => {
                (self.port_gpios.port_set_output_register.bitwise_inc_or( (1 << (self.pin.clone() as u32))));
            },
            Value::Low => {
                (self.port_gpios.port_clear_output_register.bitwise_inc_or( (1 << (self.pin.clone() as u32))));
            }
        }
    }
    pub fn set_direction(&self, direction: Direction){
        match direction {
            Direction::In => {
                (self.port_gpios.port_data_direction_register.bitwise_and( !(1 << (self.pin.clone() as u32))));
            },
            Direction::Out => {
                (self.port_gpios.port_data_direction_register.bitwise_inc_or( (1 << (self.pin.clone() as u32))));
            }
        }
    }
}

#[repr(C)]
pub struct PortGpios {
    // Actual value of the GPIO when as output
    pub port_data_output_register: VolatileRW<u32>,

    // Whether to set it or not
    pub port_set_output_register: VolatileRW<u32>,

    // Whether to clear it or not
    pub port_clear_output_register: VolatileRW<u32>,

    // Whether to invert it or not
    pub port_toggle_output_register: VolatileRW<u32>,

    // Value of the GPIO when as input
    pub port_data_input_register: VolatileRW<u32>,

    // Direction of the GPIO: 0 = input // 1 = output
    pub port_data_direction_register: VolatileRW<u32>,
}

impl PortGpios {
    pub fn get(port : Ports) -> &'static PortGpios {
        unsafe {
            // From 0-A 1-B 2-C 3-D 4-E each one takes 0x40 and they are mapped in sequence
            &*((BASE_PTA + ((port as u32)*0x40)) as *const PortGpios)
        }
    }

}

