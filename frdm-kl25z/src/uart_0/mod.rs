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

    pub fn send_char(bytes: char){
        while (Uart_0::get().status_register_1.get() & 0b1000_0000)==0b0000_0000{

        }
        Uart_0::get().data_register.set(bytes as u8);
        while (Uart_0::get().status_register_1.get() & 0b1000_0000)==0b0000_0000{

        }
    }

    pub fn enable_uart(baud_rate: i32){

        // Enable Port A clock
        let port_a = SystemIntegrationModule::enable_port_for_use(PortLetter::PortA);

        // Enable UART0 Clock
        SystemIntegrationModule::get().system_clock_gating_control_register_4.set_bit(10);

        // Select source as MCGFLLCLK clock or MCGPLLCLK/2 clock
        SystemIntegrationModule::get().system_option_register_2.set_bit(26);
        SystemIntegrationModule::get().system_option_register_2.clear_bit(27);


        port_a.set_pin_as_alt2(Pin::Pin1);
        port_a.set_pin_as_alt2(Pin::Pin2);



//        SystemIntegrationModule::get().system_option_register_2.set_bit(16);

        // Disable transmiter / receiver
        Uart_0::get().control_register_2.clear_bit(2);
        Uart_0::get().control_register_2.clear_bit(3);


        Uart_0::get().control_register_2.set(0b0000_0000);
        Uart_0::get().control_register_1.set(0b0000_0000);
        Uart_0::get().control_register_3.set(0b0000_0000);
        Uart_0::get().status_register_2.set(0b0000_0000);


//        Uart_0::get().match_address_register_1.set(0b0000_0000);


//        Uart_0::get().status_register_1.bitwise_inc_or_u8(0b11111);
//        Uart_0::get().status_register_2.bitwise_inc_or_u8(0b1100_0000);
//        Uart_0::get().control_register_4.bitwise_and_u8(!(0b1 << 4));
//        Uart_0::get().status_register_2.bitwise_inc_or_u8(0b1111);
        let over_sample = 16;
        const CORE_CLOCK :i32= 20_970_000;
        let divisor: u16 = ((CORE_CLOCK / (over_sample)) / baud_rate) as u16;
//      (48000000/16)/19200 = 156,25
//      156 = 1001_1100

        Uart_0::get().control_register_4.set_bit(0);
        Uart_0::get().control_register_4.set_bit(1);
        Uart_0::get().control_register_4.set_bit(2);
        Uart_0::get().control_register_4.set_bit(3);
        Uart_0::get().control_register_4.clear_bit(4);


//        let sysclk: u32= 24_000_000;
//        let sbr = ((sysclk) / (baud * osr as u32)) as u16;
//        let tmp = Uart_0::get().baud_rate_register_high.get() & (!0b11111);

        Uart_0::get().baud_rate_register_high.set(((divisor>>8) & 0x1F) as u8);
        Uart_0::get().baud_rate_register_low.set((divisor & 0xff) as u8);
//        Uart_0::get().baud_rate_register_high.set(0x0);
//        Uart_0::get().baud_rate_register_low.set(0x44);

        Uart_0::get().control_register_2.set_bit(2);
        Uart_0::get().control_register_2.set_bit(3);



    }

}