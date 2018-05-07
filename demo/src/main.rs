/* ********************************************** */
/* File name:        main.rs                      */
/* File description: This file implements sample  */
/*                   program used to demonstrate  */
/*                   each part of the project     */
/* Author name:      tiberioferreira              */
/* Creation date:    05mar2018                    */
/* Revision date:    23abr2015                    */
/* ********************************************** */
#![no_std]
#![feature(used)]
#![feature(core_intrinsics)]
#![feature(asm)]
extern crate es670_board;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate arraydeque;
//mod serial_state_machine;
//use serial_state_machine::*;
use cortex_m::asm;
use es670_board::*;
use arraydeque::{ArrayDeque, Saturating};
use core::cell::UnsafeCell;

static mut INTERRUPTS_DEQUE: Option<ArrayDeque<[char; 20], Saturating>> = None;

static mut PERIOD_ELAPSED: VolatileRW<bool> = es670_board::VolatileRW{
    value: UnsafeCell::new(false)
};

static mut NUMBER_COUNTS_1_SEC: VolatileRW<u32> = es670_board::VolatileRW{
    value: UnsafeCell::new(0)
};

fn digit_to_char(u_int: u32) -> char{
    let u_int = u_int%10;
    match u_int {
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

fn u32_to_str(u_int: u32) -> [char; 4]{
    let copy = u_int;
    let first_digit = digit_to_char(copy%10);
    let copy = copy/10;
    let second_digit = digit_to_char(copy%10);
    let copy = copy/10;
    let third_digit = digit_to_char(copy%10);
    let copy = copy/10;
    let forth_digit = digit_to_char(copy%10);
    let copy = copy/10;
    [forth_digit, third_digit, second_digit, first_digit]
//    let string = format!("{}{}{}{}",forth_digit, third_digit, second_digit, first_digit);
//    string.as_str()
}

fn main() {

    let board  = Es670Board::new();
    Tpm0::init();

    let mut was_on = true;

    board.turn_on_led(Led::BLUE);
    board.delay(2000);
    board.turn_off_led(Led::BLUE);
    board.lcd_clear();
    board.enable_low_power_timer(); // has 1hz frequency
//    board.start_fan();
    // expect 5000 rpm = 83,333333333 rps
    loop{
//        if MultiPurposeClockGenerator::osc_is_ok() {
//            board.turn_on_led(Led::RED);
//            board.delay(1000);
//            board.turn_off_led(Led::RED);
//            Tpm0::clear_tof();
//        }
        unsafe {
            while !PERIOD_ELAPSED.get() {

            }
            PERIOD_ELAPSED.set(false);
            board.lcd_clear();
            let counted_so_far = NUMBER_COUNTS_1_SEC.get()/7;
            for c in u32_to_str(counted_so_far).iter(){
                board.write_char(*c);
            }
            board.write_string_to_lcd(" RPS");
            board.lcd_set_cursor(1, 0);
            let counted_so_far_rpm = counted_so_far*60;
            for c in u32_to_str(counted_so_far_rpm).iter(){
                board.write_char(*c);
            }
            board.write_string_to_lcd(" RPM");
            NUMBER_COUNTS_1_SEC.set(0);

//            board.write_string_to_lcd(count_as_str);
//            if was_on{
//                board.turn_off_led(Led::BLUE);
//                was_on = false;
//            }else{
//                board.turn_on_led(Led::BLUE);
//                was_on = true;
//            }
        }
    }
}


#[link_section = ".vector_table.interrupts"]
#[used]
pub static INTERRUPTS: [unsafe extern "C" fn(); 31] =
    [
        default_handler, // 0
        default_handler, // 1
        default_handler, // 2
        default_handler, // 3
        default_handler, // 4
        default_handler, // 5
        default_handler, // 6
        default_handler, // 7
        default_handler, // 8
        default_handler, // 9
        default_handler, // 10
        default_handler, // 11
        uart0_irq_handler, // 12
        default_handler, // 13
        default_handler, // 14
        default_handler, // 15
        default_handler, // 16
        tpm0_irq_handler, // 17
        default_handler, // 18
        default_handler, // 19
        default_handler, // 20
        default_handler, // 21
        default_handler, // 22
        default_handler, // 23
        default_handler, // 24
        default_handler, // 25
        default_handler, // 26
        default_handler, // 27
        lptm_irq_handler, // 28
        default_handler, // 29
        default_handler, // 30
    ]
;

pub extern "C" fn default_handler() {
    asm::bkpt();
}

pub extern "C" fn uart0_irq_handler() {
    let rx_char =  Uart0::read_char();
    unsafe {
        match INTERRUPTS_DEQUE {
            None => {},
            Some(ref mut deque) => {
                if let Err(_) = deque.push_back(rx_char){
                    Uart0::send_string("Interrupt DEQUE is full!\n");
                }
            }
        }
    }
}

pub extern "C" fn lptm_irq_handler() {
    unsafe {
        PERIOD_ELAPSED.set(true);
    }
    Es670Board::clear_lptm_interrupt();
}


pub extern "C" fn tpm0_irq_handler() {
    unsafe {
        NUMBER_COUNTS_1_SEC.set(NUMBER_COUNTS_1_SEC.get() + 1);
//        PERIOD_ELAPSED.set(true);
    }
    Es670Board::clear_tmp0_interrupt();
}

// problemas: não saber que tinha que dar "clear na interrupção"
// problemas: não saber que tinha que habilitar modificações no lptm no sim
// problemas: o watchdog timer não pode ter dois bits escritos em operações sequenciais

