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
static mut INTERRUPTS_DEQUE: Option<ArrayDeque<[char; 20], Saturating>> = None;


fn main() {
    let board = Es670Board::new();
    board.lcd_clear();
    board.lcd_set_cursor(0, 0);
    board.write_string("Tiberio Ferreira");
    board.lcd_set_cursor(1, 5);
    board.write_string("139187");

    loop{

    }
}


#[link_section = ".vector_table.interrupts"]
#[used]
pub static INTERRUPTS: [unsafe extern "C" fn(); 20] =
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
        default_handler, // 17
        default_handler, // 18
        default_handler, // 19
    ]
;

pub extern "C" fn default_handler() {
    asm::bkpt();
}

pub extern "C" fn uart0_irq_handler() {
    let rx_char =  Uart_0::read_char();
    unsafe {
        match INTERRUPTS_DEQUE {
            None => {},
            Some(ref mut deque) => {
                if let Err(_) = deque.push_back(rx_char){
                    Uart_0::send_string("Interrupt DEQUE is full!\n");
                }
            }
        }
    }
}

