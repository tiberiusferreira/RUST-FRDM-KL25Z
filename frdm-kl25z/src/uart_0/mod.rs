use io::VolatileRW;
use super::system_integration_module::SystemIntegrationModule;
use ::*;
const BASE_UART_0 : u32 = 0x4006_A000;
// System integration Module
#[repr(C)]
pub struct Uart_0 {
    pub baud_rate_register_high: VolatileRW<u8>,
    pub baud_rate_register_low: VolatileRW<u8>,
    pub control_register_1 : VolatileRW<u8>,
    pub control_register_2: VolatileRW<u8>,
    pub status_register_1 : VolatileRW<u8>,
    pub status_register_2: VolatileRW<u8>,
    pub control_register_3: VolatileRW<u8>,
    pub data_register : VolatileRW<u8>,
    pub match_address_register_1: VolatileRW<u8>,
    pub match_address_register_2 : VolatileRW<u8>,
    pub control_register_4: VolatileRW<u8>, //read only
    pub control_register_5 : VolatileRW<u8>,
}

impl Uart_0 {
    fn get() -> &'static Uart_0 {
        unsafe {
            &*(BASE_UART_0 as *const Uart_0)
        }
    }

    fn disable_tx_rx(){
        Self::get().control_register_2.clear_bit(2);
        Self::get().control_register_2.clear_bit(3);
    }

    fn enable_tx_rx(){
        Self::get().control_register_2.set_bit(2);
        Self::get().control_register_2.set_bit(3);
    }

    fn set_default_settings(){
        Self::get().control_register_2.set(0b0000_0000);
        Self::get().control_register_1.set(0b0000_0000);
        Self::get().control_register_3.set(0b0000_0000);
        Self::get().status_register_2.set(0b0000_0000);
    }

    fn set_uart_baud_rate_using_default_mcgfllclk_clock(baud_rate: i32){
        let over_sample = 16;
        let uart0 = Self::get();
        /*
        * MCGFLLCLK clock drives UART0
        * The default settings give 20.97Mhz clock
        * See section 24.5.2 "Using a 32.768 kHz reference" of the datasheet for more info
        */
        const MCGFLLCLK_CLOCK :i32= 20_970_000;
        let divisor: u16 = ((MCGFLLCLK_CLOCK / (over_sample)) / baud_rate) as u16;

        uart0.control_register_4.bitwise_inc_or_u8(0b0000_1111);
        uart0.control_register_4.bitwise_and_u8(!0b0001_0000);

        uart0.baud_rate_register_high.set(((divisor>>8) & 0x1F) as u8);
        uart0.baud_rate_register_low.set((divisor & 0xff) as u8);
    }

    fn tx_buffer_empty() -> bool{
        return Self::get().status_register_1.get_bit(7);
    }

    pub fn send_char(bytes: char){
        while !Self::tx_buffer_empty(){}
        Self::get().data_register.set(bytes as u8);
    }

    pub fn rx_buffer_full() -> bool{
        return Self::get().status_register_1.get_bit(5);
    }

    pub fn read_char() -> char{
        while !Self::rx_buffer_full(){}
        return Self::get().data_register.get() as char;
    }

    pub fn enable_rx_interrupts(){
        nvic::Nvic::enable_uart0_interrupt();
        Self::get().control_register_2.set_bit(5);
    }

    pub fn disable_rx_interrupts(){
        Self::get().control_register_2.clear_bit(5);
    }


    pub fn enable_uart(baud_rate: i32){

        let port_a = SystemIntegrationModule::enable_port_for_use(PortLetter::PortA);

        port_a.set_pin_as_alt2(Pin::Pin1);
        port_a.set_pin_as_alt2(Pin::Pin2);
        SystemIntegrationModule::enable_uart0_clock();
        Self::disable_tx_rx();
        SystemIntegrationModule::set_uart0_clock_to_mcgfllclk();
        Self::set_default_settings();
        Self::set_uart_baud_rate_using_default_mcgfllclk_clock(baud_rate);
        Self::enable_tx_rx();

    }

}