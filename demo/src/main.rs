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
#![feature(panic_implementation)]
#![no_main]
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
use core::intrinsics;
use core::panic::PanicInfo;
extern crate es670_board;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate arraydeque;
mod controller;
use controller::*;
//mod serial_state_machine;
//use serial_state_machine::*;
use cortex_m::asm;
use es670_board::*;
use arraydeque::{ArrayDeque, Saturating};
use core::cell::UnsafeCell;
use rt::ExceptionFrame;

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


pub union Vector {
    handler: extern "C" fn(),
    reserved: usize,
}

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 31] = [
    Vector { reserved: 0 }, // 0
    Vector { reserved: 0 }, // 1
    Vector { reserved: 0 }, // 2
    Vector { reserved: 0 }, // 3
    Vector { reserved: 0 }, // 4
    Vector { reserved: 0 }, // 5
    Vector { reserved: 0 }, // 6
    Vector { reserved: 0 }, // 7
    Vector { reserved: 0 }, // 8
    Vector { reserved: 0 }, // 9
    Vector { reserved: 0 }, // 10
    Vector { reserved: 0 }, // 11
    Vector { handler: uart0_irq_handler }, // 12
    Vector { reserved: 0 }, // 13
    Vector { reserved: 0 }, // 14
    Vector { reserved: 0 }, // 15
    Vector { reserved: 0 }, // 16
    Vector { handler: tpm0_irq_handler }, // 17
    Vector { reserved: 0 }, // 18
    Vector { reserved: 0 }, // 19
    Vector { reserved: 0 }, // 20
    Vector { reserved: 0 }, // 21
    Vector { reserved: 0 }, // 22
    Vector { reserved: 0 }, // 23
    Vector { reserved: 0 }, // 24
    Vector { reserved: 0 }, // 25
    Vector { reserved: 0 }, // 26
    Vector { reserved: 0 }, // 27
    Vector { handler: lptm_irq_handler }, // 28
    Vector { reserved: 0 }, // 29
    Vector { reserved: 0 }, // 30
];

entry!(main);
const LOOP_PERIOD_MS: u16 = 300;
const TARGET_RPM: u32 = 5000;
fn main() -> ! {
    let mut board  = Es670Board::new();

    board.turn_on_led(Led::RED);
    board.delay(100);
    board.turn_off_led(Led::RED);
    board.enable_low_power_timer(LOOP_PERIOD_MS);
    Uart0::enable_uart(115200);
    board.init_fan_n_heater_as_pwm();
    board.tachometer_start_counter();
    board.set_fan_speed(100);
    board.lcd_clear();

    unsafe {
        INTERRUPTS_DEQUE = Some(ArrayDeque::<[char; 20]>::new());
    }
    let mut controller = controller::Controller{
        kp: 5,
        ki: 1,
        kd: 0,
        accumulated_error: 0,
        last_error: 0,
    };
    loop {
        unsafe {
            while !PERIOD_ELAPSED.get() {}
            PERIOD_ELAPSED.set(false);
        }
        let rpm = get_rpm_and_print_to_lcd(&board);
        let output = controller.tick(TARGET_RPM, rpm);
        board.set_fan_speed(output);
    }

}

fn get_rpm_and_print_to_lcd(board: &es670_board::Es670Board) -> u32{
    let rotations_since_last_iteration = board.tachometer_counter_get_current_value();
    let rotations_per_second = rotations_since_last_iteration as f32 / (LOOP_PERIOD_MS as f32 / 1000.0);
    let rotations_per_minute = rotations_per_second * 60.0;
    let rpm = u32_to_str(rotations_per_minute as u32);
    board.tachometer_counter_reset();
    board.lcd_clear();
    board.write_string_to_lcd("RPM:");
    rpm.iter().for_each(|c| board.write_char(*c));
    rotations_per_minute as u32
}

pub extern "C" fn default_handler() {
    asm::bkpt();
}

exception!(*, exception_default_handler);

fn exception_default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
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

#[no_mangle]
#[panic_implementation]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}