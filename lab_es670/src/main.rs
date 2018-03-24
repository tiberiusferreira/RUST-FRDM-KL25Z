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
    let board = es670::Es670::new();
    loop {
        if board.get_switch_state(Switch::S1) == Low {
            board.display_show(Display::DS1, '0');
            continue;
        }else {
            board.display_clear();
        }
        if board.get_switch_state(Switch::S2) == Low {
            board.display_show(Display::DS2, '1');
            board.turn_on_led(Led::RED);
            continue;
        }else {
            board.display_clear();
            board.turn_off_led(Led::RED);
        }
        if board.get_switch_state(Switch::S3) == Low {
            board.display_show(Display::DS3, '2');
            board.turn_on_led(Led::GREEN);
            continue;
        }else {
            board.display_clear();
            board.turn_off_led(Led::GREEN);
        }
        if board.get_switch_state(Switch::S4) == Low {
            board.display_show(Display::DS4, '3');
            board.turn_on_led(Led::BLUE);
            continue;
        }else {
            board.display_clear();
            board.turn_off_led(Led::BLUE);
        }
    }
}
