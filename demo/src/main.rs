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

/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
*                   TABELA PARA USO DO SENSOR DE TEMPERATURA            *
* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
//const TABELA_TEMP :[u8; 256] = [
////    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 						//15
////    0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1, 						//31
////    2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9, 						//47
////    10,10,11,11,12,12,13,13,14,14,15,15,16,16,17,17, 				//63
////    18,18,19,19,20,20,21,21,22,22,23,23,23,24,24,25, 				//79
////    25,26,26,27,27,28,28,29,29,30,30,31,31,32,32,33, 				//95
////    33,34,34,35,35,36,36,37,37,38,38,39,39,40,40,41, 				//111
////    41,42,42,43,43,44,44,45,45,46,46,47,47,48,48,49, 				//127
////    49,50,50,51,51,52,52,53,53,54,54,55,55,56,56,57, 				//143
////    57,58,58,59,59,60,60,61,61,62,62,63,63,64,64,65, 				//159
////    65,66,66,67,67,68,68,69,69,70,70,71,71,72,72,73, 				//175
////    73,74,74,75,75,76,76,77,77,78,78,79,79,80,80,81, 				//191
////    81,82,82,83,83,84,84,85,85,86,86,87,87,88,88,089, 				//207
////    89,90,90,91,91,92,92,93,93,94,94,95,95,96,96,097, 				//223
////    97,98,98,99,99,100,100,101,101,102,102,103,103,104,104,104, 	                //239
////    105,105,106,106,107,107,108,108,109,109,110,110,111,111,112,112                 //255
////];
const TABELA_TEMP :[u8; 256] = [
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,					//15
0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,					//31
1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 5, 5, 6, 6, 6, 6,					//47
7, 7, 8, 8, 8, 8, 9, 9, 10, 10, 10, 10, 11, 11, 12, 12,			//63
12, 12, 13, 13, 14, 14, 15, 15, 15, 15, 16, 16, 16, 17, 17, 17,	//79
17, 18, 18, 19, 19, 19, 19, 20, 20, 21, 21, 21, 21, 22, 22, 23,	//95
23, 24, 24, 24, 24, 25, 25, 26, 26, 26, 26, 27, 27, 28, 28, 28,	//111
28, 29, 29, 30, 30, 30, 30, 31, 31, 32, 32, 32, 32, 33, 33, 34,	//127
34, 35, 35, 35, 35, 36, 36, 37, 37, 37, 37, 38, 38, 39, 39, 39,	//143
39, 40, 40, 41, 41, 41, 41, 42, 42, 43, 43, 44, 44, 44, 44, 45,	//159
45, 46, 46, 46, 46, 47, 47, 48, 48, 48, 48, 49, 49, 50, 50, 50,	//175
50, 51, 51, 52, 52, 53, 53, 53, 53, 54, 54, 55, 55, 55, 55, 56,	//191
56, 57, 57, 57, 57, 58, 58, 59, 59, 59, 59, 60, 60, 61, 61, 62,	//207
62, 62, 62, 63, 63, 64, 64, 64, 64, 65, 65, 66, 66, 66, 66, 67,	//223
67, 68, 68, 68, 68, 69, 69, 70, 70, 71, 71, 71, 71, 72, 72, 72,	//239
73, 73, 73, 73, 74, 74, 75, 75, 75, 75, 76, 76, 77, 77, 77, 77	//255
];
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


// 1.26v

    board.delay(300);
    board.turn_on_led(Led::BLUE);
    board.delay(100);
    board.turn_off_led(Led::BLUE);

//    board.set_fan_speed(100);
//    board.set_heater_intensity(60);
    Adc::init_adc();


    unsafe {
        INTERRUPTS_DEQUE = Some(ArrayDeque::<[char; 20]>::new());
    }



//    Uart0::enable_rx_interrupts();
//    let mut state_machine = StateMachine::new();
//    board.tachometer_start_counter();

//    board.write("Ok!");
    loop {


        Adc::init_conversion();
        board.delay(100);
        let result = Adc::get_result();
        let result_usize = Adc::get_result() as usize;

        let temp = TABELA_TEMP[result_usize];
        Uart0::send_string("Valor ADC: ");
        for c in u32_to_str(result as u32).iter(){
            Uart0::send_char(*c);
        }
        Uart0::send_char('\n');
//        Uart0::send_string("Valor Volts: ");
//        for c in u32_to_str(((result as u32)*(100/5))*128 as u32).iter(){
//            Uart0::send_char(*c);
//        }
        Uart0::send_char('\n');
        Uart0::send_string("Valor Graus: ");
        for c in u32_to_str(temp as u32).iter(){
            Uart0::send_char(*c);
        }
        Uart0::send_char('\n');




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


