/* ************************************************************ */
/* File name:        io.rs                                      */
/* File description: This module                                */
/*                   implements the io functionality for the    */
/*                   registers                                  */
/* Author name:      tiberioferreira                            */
/* Creation date:    14abr2018                                  */
/* Revision date:    23abr2018                                  */
/* ************************************************************ */


use core::intrinsics::{volatile_load, volatile_store};
use core::cell::UnsafeCell;

pub struct VolatileRW<T> {
    pub value: UnsafeCell<T>,
}

impl<T> VolatileRW<T> {
    #[inline]
    pub fn get(&self) -> T {
        unsafe {
            volatile_load(self.value.get() as *const T)
        }
    }

    #[inline]
    pub fn set(&self, value: T) {
        unsafe {
            volatile_store(self.value.get(), value);
        }
    }
}

impl VolatileRW<u32> {
    #[inline]
    pub fn bitwise_inc_or(&self, value: u32) {
        let read = self.get();
        self.set(read | value);
    }

    #[inline]
    pub fn get_bit(&self, bit_number: u32) -> bool {
        if bit_number > 31{
            return false;
        }
        return self.get() & (1 << bit_number) != 0;
    }

    #[inline]
    pub fn set_bit(&self, bit_number: u8) {
        if bit_number > 31{
            return;
        }
        let read = self.get();
        self.set(read | (0b1 << bit_number));
    }

    #[inline]
    pub fn clear_bit(&self, bit_number: u8) {
        if bit_number > 31{
            return;
        }
        let read = self.get();
        self.set(read & !(0b1 << bit_number));
    }

    #[inline]
    pub fn bitwise_and(&self, value: u32) {
        let read = self.get();
        self.set(read & value);
    }
}

impl VolatileRW<u8> {
    pub fn bitwise_inc_or_u8(&self, value: u8) {
        let read = self.get();
        self.set(read | value);
    }

    #[inline]
    pub fn get_bit(&self, bit_number: u32) -> bool {
        if bit_number > 7{
            return false;
        }
        return self.get() & (1 << bit_number) != 0;
    }

    #[inline]
    pub fn set_bit(&self, bit_number: u8) {
        if bit_number > 7{
            return;
        }
        let read = self.get();
        self.set(read | (0b1 << bit_number));
    }

    #[inline]
    pub fn clear_bit(&self, bit_number: u8) {
        if bit_number > 7{
            return;
        }
        let read = self.get();
        self.set(read & !(0b1 << bit_number));
    }

    #[inline]
    pub fn bitwise_and_u8(&self, value: u8) {
        let read = self.get();
        self.set(read & value);
    }
}

pub struct VolatileR<T> {
    value: UnsafeCell<T>,
}

impl<T> VolatileR<T> {
    #[inline]
    pub fn get(&self) -> T {
        unsafe {
            volatile_load(self.value.get() as *const T)
        }
    }
}
