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

fn main() -> ! {
    let mut board  = Es670Board::new();

    board.turn_on_led(Led::RED);
    board.delay(100);
    board.turn_off_led(Led::RED);
    board.enable_low_power_timer(500);
    Uart0::enable_uart(115200);
    board.init_fan_n_heater_as_pwm();

    board.set_fan_speed(40);
    board.set_heater_intensity(50);

    unsafe {
        INTERRUPTS_DEQUE = Some(ArrayDeque::<[char; 20]>::new());
    }
//    Uart0::enable_rx_interrupts();
//    let mut state_machine = StateMachine::new();
//    board.tachometer_start_counter();

    loop {
        unsafe {
            while !PERIOD_ELAPSED.get() {}
            PERIOD_ELAPSED.set(false);
        }
        let (raw_adc, temp) = board.get_heater_temp();
        Uart0::send_string("Valor ADC: ");
        for c in u32_to_str(raw_adc as u32).iter(){
            Uart0::send_char(*c);
        }
        Uart0::send_char('\n');

        Uart0::send_string("Valor Graus: ");
        for c in u32_to_str(temp as u32).iter(){
            Uart0::send_char(*c);
        }
        Uart0::send_char('\n');

//        board.delay(100);
//        board.turn_on_led(Led::RED);
//        board.delay(100);
//        board.turn_off_led(Led::RED);

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