#![no_std]
#![feature(used)]
#![feature(core_intrinsics)]
#![feature(asm)]
extern crate es670;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
use cortex_m::asm;
use es670::*;
use es670::{High, Low};


fn main() {
    fn int_to_char(mut int: i32) -> char{
        int = int%10;
        match int {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => ' '
        }
    }
    let board = es670::Es670::new();
    let mut number_times_switch_0_pressed = 4;
    let mut number_times_switch_1_pressed = 3;
    let mut number_times_switch_2_pressed = 2;
    let mut number_times_switch_3_pressed = 1;
    let mut switch_was_pressed_0 = false;
    let mut switch_was_pressed_1 = false;
    let mut switch_was_pressed_2 = false;
    let mut switch_was_pressed_3 = false;

        es670::Uart_0::enable_uart(9600);
    loop {
        es670::Uart_0::send_char(0b1100_0001);
//        es670::Uart_0::send_char(0b0000_1000);
//        es670::Uart_0::send_char(0b1000_0000);
//        board.turn_on_led(Led::GREEN);
//        board.delay(500);
//        board.turn_off_led(Led::GREEN);
        board.delay(500);
//        board.delay(1000);
//        board.display_show(Display::DS1, int_to_char(number_times_switch_0_pressed));
//        board.delay(1);
//        board.display_show(Display::DS2, int_to_char(number_times_switch_1_pressed));
//        board.delay(1);
//        board.display_show(Display::DS3, int_to_char(number_times_switch_2_pressed));
//        board.delay(1);
//        board.display_show(Display::DS4, int_to_char(number_times_switch_3_pressed));
//        board.delay(1);
//
//
//        if board.get_switch_state(Switch::S1) == Low && !switch_was_pressed_0{
//            board.turn_off_led(Led::L1);
//            number_times_switch_0_pressed = number_times_switch_0_pressed + 1;
//            switch_was_pressed_0 = true;
//            continue;
//        }else if board.get_switch_state(Switch::S1) == High{
//            switch_was_pressed_0 = false;
//            board.turn_on_led(Led::L1);
//        }
//
//
//        if board.get_switch_state(Switch::S2) == Low && !switch_was_pressed_1{
//            board.turn_off_led(Led::L2);
//            number_times_switch_1_pressed = number_times_switch_1_pressed + 1;
//            switch_was_pressed_1 = true;
//            continue;
//        }else if board.get_switch_state(Switch::S2) == High{
//            switch_was_pressed_1 = false;
//            board.turn_on_led(Led::L2);
//        }
//
//
//        if board.get_switch_state(Switch::S3) == Low && !switch_was_pressed_2{
//            board.turn_on_led(Led::L3);
//            number_times_switch_2_pressed = number_times_switch_2_pressed + 1;
//            switch_was_pressed_2 = true;
//            continue;
//        }else if board.get_switch_state(Switch::S3) == High{
//            board.turn_on_led(Led::L3);
//            switch_was_pressed_2 = false;
//        }
//
//
//        if board.get_switch_state(Switch::S4) == Low && !switch_was_pressed_3{
//            board.turn_off_led(Led::L4);
//            number_times_switch_3_pressed = number_times_switch_3_pressed + 1;
//            switch_was_pressed_3 = true;
//            continue;
//        }else if board.get_switch_state(Switch::S4) == High{
//            board.turn_on_led(Led::L4);
//            switch_was_pressed_3 = false;
//        }


    }
}


#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 20] = [default_handler; 20];

extern "C" fn default_handler() {
    asm::bkpt();
}

