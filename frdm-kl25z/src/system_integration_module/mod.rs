use io::VolatileRW;
mod port;
mod gpio;
pub use self::port::PortLetter;
pub use self::port::PortWrapper;
pub use self::gpio::Gpio;
pub use self::port::Pin;
pub use self::gpio::Direction;
pub use self::gpio::Value;
use self::port::PortRegisters;
const BASE_SIM : u32 = 0x4004_7000;
// System integration Module
#[repr(C)]
pub struct SystemIntegrationModule {
    pub system_option_register_1: VolatileRW<u32>,
    pub system_config_register: VolatileRW<u32>,
    pub reserved_0 : [u8; 4092],
    pub system_option_register_2: VolatileRW<u32>,
    pub reserved_1 : [u8; 4],
    pub system_option_register_4: VolatileRW<u32>,
    pub system_option_register_5: VolatileRW<u32>,
    pub reserved_2 : [u8; 4],
    pub system_option_register_7: VolatileRW<u32>,
    pub reserved_3 : [u8; 8],
    pub system_device_identification_register: VolatileRW<u32>, //read only
    pub reserved_4 : [u8; 12],
    pub system_clock_gating_control_register_4: VolatileRW<u32>,
    pub system_clock_gating_control_register_5: VolatileRW<u32>,
    pub system_clock_gating_control_register_6: VolatileRW<u32>,
    pub system_clock_gating_control_register_7: VolatileRW<u32>,
    pub system_clock_divider_register_1: VolatileRW<u32>,
    pub reserved_5 : [u8; 4],
    pub system_flash_config_register_1: VolatileRW<u32>,
    pub system_flash_config_register_2: VolatileRW<u32>,
    pub reserved_6 : VolatileRW<u32>,
    pub unique_identification_register_mid_high: VolatileRW<u32>,
    pub unique_identification_register_mid_low: VolatileRW<u32>,
    pub unique_identification_register_low: VolatileRW<u32>,
    pub reserved_7 : [u8; 156],
    pub cop_control_register: VolatileRW<u32>,
    pub service_cop_register: VolatileRW<u32>,
}

impl SystemIntegrationModule {
    fn get() -> &'static SystemIntegrationModule {
        unsafe {
            &*(BASE_SIM as *const SystemIntegrationModule)
        }
    }
    pub fn disable_watchdog_timer(){
        SystemIntegrationModule::get().cop_control_register.set(00 << 2);

    }
    pub fn enable_port_for_use(port: PortLetter) -> PortWrapper{
        // Enabling clock on Port
        // For port A set bit 9 (starting from 0) to 1
        // for B set bit 10, for C set bit 11 and so on
        SystemIntegrationModule::get()
            .system_clock_gating_control_register_5
            .bitwise_inc_or(1 << (9 + (port.clone() as u8)));
        PortWrapper::new(port)
    }
}