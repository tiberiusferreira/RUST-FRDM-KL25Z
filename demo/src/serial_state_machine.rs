extern crate es670_board;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate arraydeque;
use es670_board::*;
use super::Controller;
use arraydeque::{ArrayDeque, Saturating, Array};
const MAX_RPS : u32 = 100;
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
    CoolerPwmFreq,
    CoolerPwmFreqDig1(u32),
    CoolerPwmFreqDig2(u32),
    Kp,
    KpDig1(u32),
    KpDig2(u32),
    Ki,
    KiDig1(u32),
    KiDig2(u32),
    Kd,
    KdDig1(u32),
    KdDig2(u32),
    Goal,
    GoalDig1(u32),
    GoalDig2(u32),
}


pub struct StateMachine{
    state: State,
    board: Es670Board,
    pub controller: Controller
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
    fn next(self, input: char, board: &Es670Board, controller: &mut Controller) -> State {
        use self::State::*;

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
                    'F' | 'f' => {
                        State::CoolerPwmFreq
                    },
                    'P' | 'p' => {
                        State::Kp
                    },
                    'I' | 'i' => {
                        State::Ki
                    },
                    'D' | 'd' => {
                        State::Kd
                    },
                    'G' | 'g' => {
                        State::Goal
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
            },
            CoolerPwmFreq => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        State::CoolerPwmFreqDig1(digit.clone()*100)
                    },
                }
            },
            CoolerPwmFreqDig1(digit2) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        State::CoolerPwmFreqDig2(digit2 + digit.clone()*10)
                    },
                }
            },
            CoolerPwmFreqDig2(digit21) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        Self::send_ack();
                        let freq = digit21 + digit;
                        board.set_fan_n_heater_pwm_freq(freq as u8);
                        State::Idle
                    },
                }
            },
            Kp => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        State::KpDig1(digit.clone()*100)
                    },
                }
            },
            KpDig1(digit2) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        State::KpDig2(digit2 + digit.clone()*10)
                    },
                }
            },
            KpDig2(digit21) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        Self::send_ack();
                        let kp = digit21 + digit;
                        controller.kp = (kp as f32)/100.0;
                        State::Idle
                    },
                }
            },
            Ki => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        State::KiDig1(digit.clone()*100)
                    },
                }
            },
            KiDig1(digit2) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        State::KiDig2(digit2 + digit.clone()*10)
                    },
                }
            },
            KiDig2(digit21) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        Self::send_ack();
                        let ki = digit21 + digit;
                        controller.ki = (ki as f32)/100.0;
                        State::Idle
                    },
                }
            },
            Kd => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        State::KdDig1(digit.clone()*100)
                    },
                }
            },
            KdDig1(digit2) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        State::KdDig2(digit2 + digit.clone()*10)
                    },
                }
            },
            KdDig2(digit21) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        Self::send_ack();
                        let kd = digit21 + digit;
                        controller.kd = (kd as f32)/100.0;
                        State::Idle
                    },
                }
            },
            Goal => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        State::GoalDig1(digit.clone()*100)
                    },
                }
            },
            GoalDig1(digit2) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        State::GoalDig2(digit2 + digit.clone()*10)
                    },
                }
            },
            GoalDig2(digit21) => {
                match input.to_digit(10) {
                    None => {
                        Self::send_err();
                        State::Idle
                    },
                    Some(digit) => {
                        Self::send_ack();
                        let mut goal = digit21 + digit;
                        if goal > MAX_RPS{
                            Uart0::send_string("\nWARNING: Requested RPS too high, defaulting to 100 RPS\n");
                            goal = MAX_RPS;
                        }
                        controller.goal = goal;
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
            controller: Controller::new()
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
    pub fn handle_input(mut self, input: char) -> StateMachine{
        let state = self.state.next(input, &self.board,&mut self.controller);
        StateMachine{
            state,
            board: self.board,
            controller: self.controller
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
