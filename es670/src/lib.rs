#![feature(used)]
#![no_std]
#![feature(core_intrinsics)]
#![feature(asm)]
extern crate frdm_kl25z;
mod led;
mod display;
mod switch;
pub use switch::*;
pub use led::*;
pub use display::*;
pub use frdm_kl25z::*;

pub use frdm_kl25z::Value::{High, Low};
pub struct Es670{
    frdm_kl25z:  frdm_kl25z::FrdmKl25zBoard
}


impl Es670{
    pub fn new() -> Es670{
        let es670 = Es670{
            frdm_kl25z: FrdmKl25zBoard::new()
        };
        es670.frdm_kl25z.disable_watchdog_timer();
        es670
    }

    pub fn get_gpio(&self, port: PortLetter, pin: Pin) -> Gpio{
        let port = self.frdm_kl25z.get_port(port);
        let gpio = port.set_pin_as_gpio(pin);
        return gpio
    }


    pub fn delay(&self, ms: u32){
        self.frdm_kl25z.delay_ms(ms);
    }

    pub fn turn_on_led(&self, led: Led){
        match led{
            Led::RED => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortB);
                let gpio_18 = port.set_pin_as_gpio(Pin::Pin18);
                gpio_18.set_direction(Direction::Out);
                gpio_18.set_value(Value::Low);
            },
            Led::BLUE => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortD);
                let gpio_01 = port.set_pin_as_gpio(Pin::Pin1);
                gpio_01.set_direction(Direction::Out);
                gpio_01.set_value(Value::Low);
            },
            Led::GREEN => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortB);
                let gpio_19 = port.set_pin_as_gpio(Pin::Pin19);
                gpio_19.set_direction(Direction::Out);
                gpio_19.set_value(Value::Low);
            },
            Led::L1 => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortA);
                let gpio = port.set_pin_as_gpio(Pin::Pin1);
                gpio.set_direction(Direction::Out);
                gpio.set_value(Value::High);
            },
            Led::L2 => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortA);
                let gpio = port.set_pin_as_gpio(Pin::Pin2);
                gpio.set_direction(Direction::Out);
                gpio.set_value(Value::High);
            },
            Led::L3 => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortA);
                let gpio = port.set_pin_as_gpio(Pin::Pin4);
                gpio.set_direction(Direction::Out);
                gpio.set_value(Value::High);
            },
            Led::L4 => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortA);
                let gpio = port.set_pin_as_gpio(Pin::Pin5);
                gpio.set_direction(Direction::Out);
                gpio.set_value(Value::High);
            }
        }
    }

    pub fn turn_off_led(&self, led: Led){
        match led{
            Led::RED => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortB);
                let gpio_18 = port.set_pin_as_gpio(Pin::Pin18);
                gpio_18.set_direction(Direction::Out);
                gpio_18.set_value(Value::High);
            },
            Led::BLUE => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortD);
                let gpio_01 = port.set_pin_as_gpio(Pin::Pin1);
                gpio_01.set_direction(Direction::Out);
                gpio_01.set_value(Value::High);
            },
            Led::GREEN => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortB);
                let gpio_19 = port.set_pin_as_gpio(Pin::Pin19);
                gpio_19.set_direction(Direction::Out);
                gpio_19.set_value(Value::High);
            },
            Led::L1 => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortA);
                let gpio = port.set_pin_as_gpio(Pin::Pin1);
                gpio.set_direction(Direction::Out);
                gpio.set_value(Value::Low);
            },
            Led::L2 => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortA);
                let gpio = port.set_pin_as_gpio(Pin::Pin2);
                gpio.set_direction(Direction::Out);
                gpio.set_value(Value::Low);
            },
            Led::L3 => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortA);
                let gpio = port.set_pin_as_gpio(Pin::Pin4);
                gpio.set_direction(Direction::Out);
                gpio.set_value(Value::Low);
            },
            Led::L4 => {
                let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortA);
                let gpio = port.set_pin_as_gpio(Pin::Pin5);
                gpio.set_direction(Direction::Out);
                gpio.set_value(Value::Low);
            }
        }
    }

    pub fn display_show(&self, display: Display, input_char: char){
        let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortC);
        let gpio;
        // Disable all displays to clean up previous calls to display show
        port.set_pin_as_gpio(Pin::Pin13).set_value(Low);
        port.set_pin_as_gpio(Pin::Pin12).set_value(Low);
        port.set_pin_as_gpio(Pin::Pin11).set_value(Low);
        port.set_pin_as_gpio(Pin::Pin10).set_value(Low);
        // Enable display for usage
        match display {
            Display::DS1 => {
                gpio = port.set_pin_as_gpio(Pin::Pin13);
            },
            Display::DS2 => {
                gpio = port.set_pin_as_gpio(Pin::Pin12);
            },
            Display::DS3 => {
                gpio = port.set_pin_as_gpio(Pin::Pin11);
            },
            Display::DS4 => {
                gpio = port.set_pin_as_gpio(Pin::Pin10);
            }
        }

        //
        let seg_a = port.set_pin_as_gpio(Pin::Pin0);
        let seg_b = port.set_pin_as_gpio(Pin::Pin1);
        let seg_c = port.set_pin_as_gpio(Pin::Pin2);
        let seg_d = port.set_pin_as_gpio(Pin::Pin3);
        let seg_e = port.set_pin_as_gpio(Pin::Pin4);
        let seg_f = port.set_pin_as_gpio(Pin::Pin5);
        let seg_g = port.set_pin_as_gpio(Pin::Pin6);
        let seg_dp = port.set_pin_as_gpio(Pin::Pin7);
        seg_dp.set_value(Low);
        //
        match input_char {
            'a' | 'A' =>{
                seg_a.set_value(High);
                seg_b.set_value(High);
                seg_c.set_value(High);
                seg_d.set_value(Low);
                seg_e.set_value(High);
                seg_f.set_value(High);
                seg_g.set_value(High);
            },
            '0' => {
                seg_a.set_value(High);
                seg_b.set_value(High);
                seg_c.set_value(High);
                seg_d.set_value(High);
                seg_e.set_value(High);
                seg_f.set_value(High);
                seg_g.set_value(Low);
            },
            '1' => {
                seg_a.set_value(Low);
                seg_b.set_value(High);
                seg_c.set_value(High);
                seg_d.set_value(Low);
                seg_e.set_value(Low);
                seg_f.set_value(Low);
                seg_g.set_value(Low);
            },
            '2' => {
                seg_a.set_value(High);
                seg_b.set_value(High);
                seg_c.set_value(Low);
                seg_d.set_value(High);
                seg_e.set_value(High);
                seg_f.set_value(Low);
                seg_g.set_value(High);
            },
            '3' => {
                seg_a.set_value(High);
                seg_b.set_value(High);
                seg_c.set_value(High);
                seg_d.set_value(High);
                seg_e.set_value(Low);
                seg_f.set_value(Low);
                seg_g.set_value(High);
            },
            '4' => {
                seg_a.set_value(Low);
                seg_b.set_value(High);
                seg_c.set_value(High);
                seg_d.set_value(Low);
                seg_e.set_value(Low);
                seg_f.set_value(High);
                seg_g.set_value(High);
            },
            '5' => {
                seg_a.set_value(High);
                seg_b.set_value(Low);
                seg_c.set_value(High);
                seg_d.set_value(High);
                seg_e.set_value(Low);
                seg_f.set_value(High);
                seg_g.set_value(High);
            },
            '6' => {
                seg_a.set_value(High);
                seg_b.set_value(Low);
                seg_c.set_value(High);
                seg_d.set_value(High);
                seg_e.set_value(High);
                seg_f.set_value(High);
                seg_g.set_value(High);
            },
            '7' => {
                seg_a.set_value(High);
                seg_b.set_value(High);
                seg_c.set_value(High);
                seg_d.set_value(Low);
                seg_e.set_value(Low);
                seg_f.set_value(Low);
                seg_g.set_value(Low);
            },
            '8' => {
                seg_a.set_value(High);
                seg_b.set_value(High);
                seg_c.set_value(High);
                seg_d.set_value(High);
                seg_e.set_value(High);
                seg_f.set_value(High);
                seg_g.set_value(High);
            },
            '9' => {
                seg_a.set_value(High);
                seg_b.set_value(High);
                seg_c.set_value(High);
                seg_d.set_value(High);
                seg_e.set_value(Low);
                seg_f.set_value(High);
                seg_g.set_value(High);
            },
            ' ' => {
                seg_a.set_value(Low);
                seg_b.set_value(Low);
                seg_c.set_value(Low);
                seg_d.set_value(Low);
                seg_e.set_value(Low);
                seg_f.set_value(Low);
                seg_g.set_value(Low);
            }
            _ => {}
        }
        gpio.set_direction(Direction::Out);
        gpio.set_value(Value::High);

    }

    pub fn display_clear(&self){
        let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortC);
        // Disable all displays to clean up previous calls to display show
        port.set_pin_as_gpio(Pin::Pin13).set_value(Low);
        port.set_pin_as_gpio(Pin::Pin12).set_value(Low);
        port.set_pin_as_gpio(Pin::Pin11).set_value(Low);
        port.set_pin_as_gpio(Pin::Pin10).set_value(Low);
    }
    pub fn get_switch_state(&self, switch: Switch) -> Value{
        let port_a = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortA);
        match switch {
            Switch::S1 => {
                let gpio = port_a.set_pin_as_gpio(Pin::Pin1);
                gpio.set_direction(Direction::In);
                return gpio.get_value()
            },
            Switch::S2 => {
                let gpio = port_a.set_pin_as_gpio(Pin::Pin2);
                gpio.set_direction(Direction::In);
                return gpio.get_value()
            },
            Switch::S3 => {
                let gpio = port_a.set_pin_as_gpio(Pin::Pin4);
                gpio.set_direction(Direction::In);
                return gpio.get_value()
            },
            Switch::S4 => {
                let gpio = port_a.set_pin_as_gpio(Pin::Pin5);
                gpio.set_direction(Direction::In);
                return gpio.get_value()
            },
        }
    }

}
