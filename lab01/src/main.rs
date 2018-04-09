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
enum State{
    Idle,
    LedCmd,
    LedCmdTurnOff,
    LedCmdTurnOn,
    SwitchCmd,
    BuzzerCmd,
    BuzzerCmdDig1(u32),
    BuzzerCmdDig2(u32),
}


struct StateMachine{
    state: State,
    board: Es670,
}

impl State {

    fn send_ack(){
        for c in "ACK".chars() {
            es670::Uart_0::send_char(c);
        }
    }
    fn send_err(){
        for c in "ERR".chars() {
            es670::Uart_0::send_char(c);
        }
    }
    fn next(self, input: char, board: &Es670) -> State {
        use State::*;

        match self {
            Idle => {
                match input {
                    'L' | 'l' => {
                        Self::send_ack();
                        State::LedCmd
                    },
                    'S' | 's' => {
                        Self::send_ack();
                        State::SwitchCmd
                    },
                    'B' | 'b' => {
                        Self::send_ack();
                        State::BuzzerCmd
                    },
                    _ => {
                        Self::send_err();
                        State::Idle
                    }
                }
            },
            LedCmd => {
                match input {
                    'C' | 'c' => {
                        Self::send_ack();
                        State::LedCmdTurnOff
                    },
                    'S' | 's' => {
                        Self::send_ack();
                        State::LedCmdTurnOn
                    },
                    _ => {
                        Self::send_err();
                        State::Idle
                    }
                }
            },
            LedCmdTurnOn => {
                match input {
                    '3' => {
                        Self::send_ack();
                        board.turn_on_led(Led::L3);
                        State::Idle
                    },
                    '4' => {
                        Self::send_ack();
                        board.turn_on_led(Led::L4);
                        State::Idle
                    },
                    _ => {
                        Self::send_err();
                        State::Idle
                    }
                }
            },
            LedCmdTurnOff => {
                match input {
                    '3' => {
                        Self::send_ack();
                        board.turn_off_led(Led::L3);
                        State::Idle
                    },
                    '4' => {
                        Self::send_ack();
                        board.turn_off_led(Led::L4);
                        State::Idle
                    },
                    _ => {
                        Self::send_err();
                        State::Idle
                    },
                }
            },
            SwitchCmd => {
                match input {
                    '3' => {
                        match board.get_switch_state(Switch::S3){
                            High => {
                                Self::send_ack();
                                es670::Uart_0::send_char('O');
                            },
                            Low => {
                                Self::send_ack();
                                es670::Uart_0::send_char('C');
                            }
                        }
                        State::Idle
                    },
                    '4' => {
                        match board.get_switch_state(Switch::S4){
                            High => {
                                Self::send_ack();
                                es670::Uart_0::send_char('O');
                            },
                            Low => {
                                Self::send_ack();
                                es670::Uart_0::send_char('C');
                            }
                        }
                        State::Idle
                    },
                    _ => {
                        Self::send_err();
                        State::Idle
                    }
                }
            },
            BuzzerCmd => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        Self::send_ack();
                        State::BuzzerCmdDig1(digit.clone()*100)
                    },
                }
            },
            BuzzerCmdDig1(digit2) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        Self::send_ack();
                        State::BuzzerCmdDig2(digit2 + digit.clone()*10)
                    },
                }
            },
            BuzzerCmdDig2(digit21) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        Self::send_ack();

                        let duration = digit21 + digit;
//                        Uart_0::send_char(digit);
                        board.turn_on_buzzer(duration);

                        State::Idle
                    },
                }
            }
        }

    }
}

impl StateMachine{
    pub fn handle_input(self, input: char) -> StateMachine{
        StateMachine{
            state: self.state.next(input, &self.board),
            board: self.board,
        }
    }
}




fn main() {
    let board = es670::Es670::new();
    es670::Uart_0::enable_uart(115200);

    //    es670::Uart_0::enable_rx_interrupts();
    let mut state_machine: StateMachine = StateMachine{
        state: State::Idle,
        board: es670::Es670::new(),
    };
//    board.turn_on_buzzer(1000);
    loop {

        let rx_char =  es670::Uart_0::read_char();
        state_machine = state_machine.handle_input(rx_char);
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
//    STATE_MACHINE.handle_input(rx_char);
    es670::Uart_0::send_char(rx_char);
}

