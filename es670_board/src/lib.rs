/* ********************************************** */
/* File name:        lib.rs                       */
/* File description: This file implements the     */
/*                   es670 board  specifics       */
/* Author name:      tiberioferreira              */
/* Creation date:    05mar2018                    */
/* Revision date:    23abr2015                    */
/* ********************************************** */
#![feature(used)]
#![no_std]
#![feature(core_intrinsics)]
#![feature(asm)]
extern crate frdm_kl25z;
mod led;
mod display;
mod switch;
mod lcd;
pub use switch::*;
pub use led::*;
pub use display::*;
pub use frdm_kl25z::*;

pub use frdm_kl25z::Value::{High, Low};
pub struct Es670Board {
    frdm_kl25z:  frdm_kl25z::FrdmKl25zBoard
}


impl Es670Board {

    /* ***************************************************** */
    /* Method name:        new                               */
    /* Method description: Creates a new Es670Board          */
    /*                     instance disabling watchdog in    */
    /*                     in the process                    */
    /* Input params:                                         */
    /* Output params:      Es670Board instance               */
    /* ***************************************************** */
    pub fn new() -> Es670Board {
        let es670_board = Es670Board {
            frdm_kl25z: FrdmKl25zBoard::new()
        };
        es670_board.frdm_kl25z.disable_watchdog_timer();
        es670_board
    }

    pub fn enable_low_power_timer(&self){
        self.frdm_kl25z.enable_low_power_timer();
    }


    pub fn clear_lptm_interrupt(){
        FrdmKl25zBoard::clear_lptm_interrupt();
    }

    pub fn clear_tmp0_interrupt(){
        FrdmKl25zBoard::clear_tpm0_interrupt();
    }


    /* ***************************************************** */
    /* Method name:        get_gpio                          */
    /* Method description: provides direct access to a GPIO  */
    /* Input params:       port => which port has the desired*/
    /*                     GPIO. pin => which pin of the port*/
    /*                     to use                            */
    /* Output params:      Gpio instance                     */
    /* ***************************************************** */
    pub fn get_gpio(&self, port: PortLetter, pin: Pin) -> Gpio{
        let port = self.frdm_kl25z.get_port(port);
        let gpio = port.set_pin_as_gpio(pin);
        return gpio
    }
    pub fn turn_on_buzzer(&self, duration: u32){
        let port = self.frdm_kl25z.get_port(frdm_kl25z::PortLetter::PortD);
        let gpio_18 = port.set_pin_as_gpio(Pin::Pin0);
        let mut time_left: i32 = duration as i32;
        while time_left > 0 {
            gpio_18.set_direction(Direction::Out);
            gpio_18.set_value(Value::Low);
            self.delay(1);
            gpio_18.set_direction(Direction::Out);
            gpio_18.set_value(Value::High);
            self.delay(1);
            time_left = time_left - 2;
        }

    }
    /* ***************************************************** */
    /* Method name:        delay                             */
    /* Method description: implements busy waiting           */
    /* Input params:       ms => how long to wait in         */
    /*                     milliseconds                      */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn delay(&self, ms: u32){
        self.frdm_kl25z.delay_ms(ms);
    }


    /* ***************************************************** */
    /* Method name:        start_fan                         */
    /* Method description: starts the fan                    */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn start_fan(&self){
        let port_a = self.frdm_kl25z.get_port(PortLetter::PortA);
        let pin13 = port_a.set_pin_as_gpio(Pin::Pin13);
        pin13.set_value(Value::High);
    }

    /* ***************************************************** */
    /* Method name:             tachometer_start_counter     */
    /* Method description:      starts tachometer counter    */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn tachometer_start_counter(&self){
        FrdmKl25zBoard::init_tpm0_ch0_as_software_counter();
    }


    /* ***************************************************** */
    /* Method name:     tachometer_counter_get_current_value */
    /* Method description: gets the tachometer counter value */
    /*                     current value                     */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn tachometer_counter_get_current_value(&self) -> u32{
        FrdmKl25zBoard::tmp0_ch0_get_current_value()/7
    }

    /* ***************************************************** */
    /* Method name:     tachometer_counter_reset             */
    /* Method description: resets the tachometer counter     */
    /*                     current value                     */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn tachometer_counter_reset(&self){
        FrdmKl25zBoard::tmp0_ch0_reset_counter();
    }

    /* ***************************************************** */
    /* Method name:        stop_fan                          */
    /* Method description: stops the fan                     */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
    pub fn stop_fan(&self){
        let port_a = self.frdm_kl25z.get_port(PortLetter::PortA);
        let pin13 = port_a.set_pin_as_gpio(Pin::Pin13);
        pin13.set_value(Value::Low);
    }


    /* ***************************************************** */
    /* Method name:        turn_on_led                       */
    /* Method description: turns on a given Led              */
    /* Input params:       led => which led to turn on       */
    /* Output params:                                        */
    /* ***************************************************** */
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

    /* ***************************************************** */
    /* Method name:        turn_off_led                      */
    /* Method description: turns off a given Led             */
    /* Input params:       led => which led to turn off       */
    /* Output params:                                        */
    /* ***************************************************** */
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


    /* ***************************************************** */
    /* Method name:        display_show                      */
    /* Method description: displays a char on the display    */
    /*                     can a number or A B C D or F      */
    /* Input params:       display => on which display to    */
    /*                     show on. input_char which char to */
    /*                     show                              */
    /* Output params:                                        */
    /* ***************************************************** */
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
        let seg_a = port.set_pin_as_gpio(Pin::Pin0);
        let seg_b = port.set_pin_as_gpio(Pin::Pin1);
        let seg_c = port.set_pin_as_gpio(Pin::Pin2);
        let seg_d = port.set_pin_as_gpio(Pin::Pin3);
        let seg_e = port.set_pin_as_gpio(Pin::Pin4);
        let seg_f = port.set_pin_as_gpio(Pin::Pin5);
        let seg_g = port.set_pin_as_gpio(Pin::Pin6);
        let seg_dp = port.set_pin_as_gpio(Pin::Pin7);
        seg_dp.set_value(Low);
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
            'b' | 'B' =>{
                seg_a.set_value(Low);
                seg_b.set_value(Low);
                seg_c.set_value(High);
                seg_d.set_value(High);
                seg_e.set_value(High);
                seg_f.set_value(High);
                seg_g.set_value(High);
            },
            'c' | 'C' =>{
                seg_a.set_value(High);
                seg_b.set_value(Low);
                seg_c.set_value(Low);
                seg_d.set_value(High);
                seg_e.set_value(High);
                seg_f.set_value(High);
                seg_g.set_value(Low);
            },
            'd' | 'D' =>{
                seg_a.set_value(Low);
                seg_b.set_value(High);
                seg_c.set_value(High);
                seg_d.set_value(High);
                seg_e.set_value(High);
                seg_f.set_value(Low);
                seg_g.set_value(High);
            },
            'e' | 'E' =>{
                seg_a.set_value(High);
                seg_b.set_value(Low);
                seg_c.set_value(Low);
                seg_d.set_value(High);
                seg_e.set_value(High);
                seg_f.set_value(High);
                seg_g.set_value(High);
            },
            'f' | 'F' =>{
                seg_a.set_value(High);
                seg_b.set_value(Low);
                seg_c.set_value(Low);
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



    /* ***************************************************** */
    /* Method name:        display_clear                     */
    /* Method description: clears the displays               */
    /* Input params:                                         */
    /* Output params:                                        */
    /* ***************************************************** */
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
