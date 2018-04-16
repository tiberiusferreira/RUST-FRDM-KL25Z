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
    let deque = Some(ArrayDeque::new());
    /* ARMv6 does not support synchronization instruction
     * But since we are using this before enabling interruption it should be safe
    */
    unsafe {
        INTERRUPTS_DEQUE = deque;
    }

    let mut state_machine: StateMachine = StateMachine::new();
    Uart_0::enable_uart(115200);
    loop {
        Uart_0::disable_rx_interrupts();
        unsafe {
            match INTERRUPTS_DEQUE {
                None => {
                    Uart_0::send_string("INTERRUPTS_DEQUE was not initialized!");
                },
                Some(ref mut deque) => {
                    state_machine = mutate_state_machine_with_deque_chars(deque, state_machine);
                }
            }
        }
        Uart_0::enable_rx_interrupts();
        board.delay(100);
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

