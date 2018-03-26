#![no_std]
#![feature(used)]
#![feature(core_intrinsics)]
#![feature(asm)]
extern crate es670;
#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
use cortex_m::asm;
use core::fmt::Write;

use cortex_m_semihosting::hio;
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
    let mut number_times_switch_0_pressed = 0;
    let mut number_times_switch_1_pressed = 2;
    let mut number_times_switch_2_pressed = 3;
    let mut number_times_switch_3_pressed = 4;
    let mut switch_was_pressed_0 = false;
    let mut switch_was_pressed_1 = false;
    let mut switch_was_pressed_2 = false;
    let mut switch_was_pressed_3 = false;
    loop {
        board.display_show(Display::DS1, int_to_char(number_times_switch_0_pressed));
        board.delay(1);
        board.display_show(Display::DS2, int_to_char(number_times_switch_1_pressed));
        board.delay(1);
        board.display_show(Display::DS3, int_to_char(number_times_switch_2_pressed));
        board.delay(1);
        board.display_show(Display::DS4, int_to_char(number_times_switch_3_pressed));
        board.delay(1);
        if board.get_switch_state(Switch::S1) == Low && !switch_was_pressed_0 {
            number_times_switch_0_pressed = number_times_switch_0_pressed + 1;
            switch_was_pressed_0 = true;
            continue;
        }else if board.get_switch_state(Switch::S1) == High{
            switch_was_pressed_0 = false;
        }


        if board.get_switch_state(Switch::S2) == Low && !switch_was_pressed_1{
            board.turn_on_led(Led::RED);
            number_times_switch_1_pressed = number_times_switch_1_pressed + 1;
            switch_was_pressed_1 = true;
            continue;
        }else if board.get_switch_state(Switch::S2) == High{
            switch_was_pressed_1 = false;
            board.turn_off_led(Led::RED);
        }


        if board.get_switch_state(Switch::S3) == Low && !switch_was_pressed_2{
            board.turn_on_led(Led::GREEN);
            number_times_switch_2_pressed = number_times_switch_2_pressed + 1;
            switch_was_pressed_2 = true;
            continue;
        }else if board.get_switch_state(Switch::S3) == High{
            board.turn_off_led(Led::GREEN);
            switch_was_pressed_2 = false;
        }
        if board.get_switch_state(Switch::S4) == Low && !switch_was_pressed_3{
            board.turn_on_led(Led::BLUE);
            number_times_switch_3_pressed = number_times_switch_3_pressed + 1;
            switch_was_pressed_3 = true;
            continue;
        }else if board.get_switch_state(Switch::S4) == High{
            board.turn_off_led(Led::BLUE);
            switch_was_pressed_3 = false;
        }

    }
}
