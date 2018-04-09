use io::VolatileRW;

const NVIC_BASE : u32 = 0xE000E100;


// System integration Module
#[repr(C)]
pub struct Nvic {
    pub interrupt_set_enable_register: VolatileRW<u32>,
    pub reserved_0: [u8; 124],
    pub interrupt_clear_enable_register: VolatileRW<u32>,
    pub reserved_1: [u8; 124],
    pub interrupt_set_pending_register: VolatileRW<u32>,
    pub reserved_2: [u8; 124],
    pub interrupt_clear_pending_register: VolatileRW<u32>,
    pub reserved_3: [u8; 380],
    pub interrupt_priority_register: VolatileRW<u32>,
}

impl Nvic {
    pub(in super) fn get() -> &'static Nvic {
        unsafe {
            &*(NVIC_BASE as *const Nvic)
        }
    }

    pub(in super) fn enable_uart0_interrupt(){
        Self::get().interrupt_clear_pending_register.set_bit(12);
        Self::get().interrupt_set_enable_register.set_bit(12);
    }


}