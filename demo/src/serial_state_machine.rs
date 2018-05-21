extern crate es670_board;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate arraydeque;
use es670_board::*;
use arraydeque::{ArrayDeque, Saturating, Array};
pub enum State{
    Idle,
    LedCmd,
    LedCmdTurnOff,
    LedCmdTurnOn,
    SwitchCmd,
    BuzzerCmd,
    BuzzerCmdDig1(u32),
    BuzzerCmdDig2(u32),
    CoolerPwm,
    CoolerPwmDig1(u32),
    CoolerPwmDig2(u32),
}


pub struct StateMachine{
    state: State,
    board: Es670Board,
}

impl State {


    fn send_ack(){
        for c in "ACK".chars() {
            Uart0::send_char(c);
        }
    }
    fn send_err(){
        for c in "ERR".chars() {
            Uart0::send_char(c);
        }
    }
    fn next(self, input: char, board: &Es670Board) -> State {
        use State::*;

        match self {
            Idle => {
                match input {
                    'L' | 'l' => {
                        State::LedCmd
                    },
                    'S' | 's' => {
                        State::SwitchCmd
                    },
                    'B' | 'b' => {
                        State::BuzzerCmd
                    },
                    'C' | 'c' => {
                        State::CoolerPwm
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
                        State::LedCmdTurnOff
                    },
                    'S' | 's' => {
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
                    'R' | 'r' => {
                        Self::send_ack();
                        board.turn_on_led(Led::RED);
                        State::Idle
                    },
                    'G' | 'g' => {
                        Self::send_ack();
                        board.turn_on_led(Led::GREEN);
                        State::Idle
                    }
                    ,
                    'B' | 'b' => {
                        Self::send_ack();
                        board.turn_on_led(Led::BLUE);
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
                    'R' | 'r' => {
                        Self::send_ack();
                        board.turn_off_led(Led::RED);
                        State::Idle
                    },
                    'G' | 'g' => {
                        Self::send_ack();
                        board.turn_off_led(Led::GREEN);
                        State::Idle
                    },
                    'B' | 'b' => {
                        Self::send_ack();
                        board.turn_off_led(Led::BLUE);
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
                                Uart0::send_char('O');
                            },
                            Low => {
                                Self::send_ack();
                                Uart0::send_char('C');
                            }
                        }
                        State::Idle
                    },
                    '4' => {
                        match board.get_switch_state(Switch::S4){
                            High => {
                                Self::send_ack();
                                Uart0::send_char('O');
                            },
                            Low => {
                                Self::send_ack();
                                Uart0::send_char('C');
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
                        board.turn_on_buzzer(duration);
                        State::Idle
                    },
                }
            },
            CoolerPwm => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        State::CoolerPwmDig1(digit.clone()*100)
                    },
                }
            },
            CoolerPwmDig1(digit2) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        State::CoolerPwmDig2(digit2 + digit.clone()*10)
                    },
                }
            },
            CoolerPwmDig2(digit21) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        Self::send_ack();
                        let duty_cycle = digit21 + digit;
                        board.set_fan_speed(duty_cycle as u8);
                        State::Idle
                    },
                }
            }
        }

    }
}

impl StateMachine{
    /* ***************************************************** */
    /* Method name:        new                               */
    /* Method description: Creates a new StateMachine        */
    /*                     instance                          */
    /* Input params:                                         */
    /* Output params:      StateMachine instance             */
    /* ***************************************************** */
    pub fn new() -> StateMachine{
        StateMachine{
            state: State::Idle,
            board: Es670Board::new(),
        }
    }
    /* ***************************************************** */
    /* Method name:        handle_input                      */
    /* Method description: Mutates the StateMachine          */
    /*                     consuming it according to the     */
    /*                     input                             */
    /* Input params:       input is the character input      */
    /* Output params:      A new mutated StateMachine        */
    /* ***************************************************** */
    pub fn handle_input(self, input: char) -> StateMachine{
        StateMachine{
            state: self.state.next(input, &self.board),
            board: self.board,
        }
    }
}

    /* ********************************************************* */
    /* Method name:        mutate_state_machine_with_deque_chars */
    /* Method description: Mutates the StateMachine              */
    /*                     consuming it according to the         */
    /*                     input in the double ended queue       */
    /* Input params:       input is the deque containing the     */
    /*                     character inputs                      */
    /* Output params:      A new mutated StateMachine            */
    /* ********************************************************  */
pub fn mutate_state_machine_with_deque_chars<A>(deque: &mut ArrayDeque<A, Saturating>, mut state_machine: StateMachine) -> StateMachine
    where A: Array<Item = char>{
    loop{
        match deque.pop_front(){
            None=>{
                break;
            },
            Some(rx_char) => {
                state_machine = state_machine.handle_input(rx_char);
            }
        }
    }
    state_machine
}
