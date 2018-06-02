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
mod serial_state_machine;
use serial_state_machine::*;
use cortex_m::asm;
use es670_board::*;
use arraydeque::{ArrayDeque, Saturating};
use core::cell::UnsafeCell;

static mut INTERRUPTS_DEQUE: Option<ArrayDeque<[char; 20], Saturating>> = None;

static mut PERIOD_ELAPSED: VolatileRW<bool> = es670_board::VolatileRW{
    value: UnsafeCell::new(false)
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

fn u32_to_str(u_int: u32) -> [char; 5]{
    let mut copy = u_int;
    let first_digit = digit_to_char(copy%10);
    copy = copy/10;
    let second_digit = digit_to_char(copy%10);
    copy = copy/10;
    let third_digit = digit_to_char(copy%10);
    copy = copy/10;
    let forth_digit = digit_to_char(copy%10);
    copy = copy/10;
    let fifth_digit = digit_to_char(copy%10);
    [fifth_digit, forth_digit, third_digit, second_digit, first_digit]
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


fn main() {
    let board  = Es670Board::new();

    board.turn_on_led(Led::BLUE);
    board.delay(100);
    board.turn_off_led(Led::BLUE);
    board.enable_low_power_timer_1hz(); // has 1hz frequency
    Uart0::enable_uart(115200);
    board.init_fan_n_heater_as_pwm();


    board.delay(300);
    board.turn_on_led(Led::BLUE);
    board.delay(100);
    board.turn_off_led(Led::BLUE);

    board.set_fan_speed(30);
    board.set_heater_intensity(50);
//    Adc::init_adc();


    unsafe {
        INTERRUPTS_DEQUE = Some(ArrayDeque::<[char; 20]>::new());
    }



//    Uart0::enable_rx_interrupts();
//    let mut state_machine = StateMachine::new();
//    board.tachometer_start_counter();

//    board.write("Ok!");
    loop {


//        Adc::init_conversion();
//        board.delay(100);
//        let result = Adc::get_result();
//        for c in u32_to_str(result as u32).iter(){
//            Uart0::send_char(*c);
//        }
//        Uart0::send_char('\n');
//        board.delay(1000);
//        unsafe {
//            while !PERIOD_ELAPSED.get() {}
//
//            PERIOD_ELAPSED.set(false);
//        }
//        Uart0::disable_rx_interrupts();
//
//        unsafe {
//            if let Some(ref mut deque) = INTERRUPTS_DEQUE{
//                state_machine = mutate_state_machine_with_deque_chars(deque, state_machine);
//            }
//        }
//        Uart0::enable_rx_interrupts();
//
//
//        board.lcd_clear();
//        let counted_so_far = board.tachometer_counter_get_current_value();
//        for c in u32_to_str(counted_so_far as u32).iter(){
//            board.write_char(*c);
//        }
//        board.write_string_to_lcd(" RPS");
//        board.lcd_set_cursor(1, 0);
//        let counted_so_far_rpm = counted_so_far*60;
//        for c in u32_to_str(counted_so_far_rpm as u32).iter(){
//            board.write_char(*c);
//        }
//        board.write_string_to_lcd(" RPM");
//        board.tachometer_counter_reset();
    }

}

pub extern "C" fn default_handler() {
    asm::bkpt();
}

pub extern "C" fn uart0_irq_handler() {
    let rx_char =  Uart0::read_char();
    unsafe {
        match INTERRUPTS_DEQUE {
            None => {
                Uart0::send_string("DEQUE not initialized yet!\n");
            },
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
    Es670Board::clear_tmp0_interrupt();
}


