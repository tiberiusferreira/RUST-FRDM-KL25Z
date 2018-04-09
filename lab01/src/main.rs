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
    let board = es670::Es670::new();
    es670::Uart_0::enable_uart(115200);
    es670::Uart_0::enable_rx_interrupts();
    loop {
        es670::Uart_0::send_char('f');
        board.delay(10000);
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
    let rx_char =  es670::Uart_0::read_char();
    es670::Uart_0::send_char(rx_char);
}

