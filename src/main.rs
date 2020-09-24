#![no_std]
#![no_main]

// pick a panicking behavior
use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use core::ptr;

#[entry]
fn main() -> ! {
    // STM23WLE Led is on PA4 pin
    const RCC_AHB2ENR: *mut u32 = 0x5800_004C as *mut u32;
    const GPIOA_MODER: *mut u32 = 0x4800_0000 as *mut u32;
    const GPIOA_BSRR: *mut u32 = 0x4800_0018 as *mut u32;

    unsafe {
        // Enable gpioa clock
        ptr::write_volatile(RCC_AHB2ENR, 0b1 << 0);

        let val = ptr::read_volatile(GPIOA_MODER);
        let mask = !(0b11 << 8);
        let pa4_mode = 0b01 << 8;

        // Set PA4 to output
        ptr::write_volatile(GPIOA_MODER, (val & mask) | pa4_mode);

        // Turn on led
        ptr::write_volatile(GPIOA_BSRR, 0b1 << 4);
    }

    loop {
        for _x in 0..1_000 {
            asm::nop();
        }

        unsafe {
            ptr::write_volatile(GPIOA_BSRR, 0b1 << 20);
        }

        for _x in 0..5_000 {
            asm::nop();
        }

        unsafe {
            ptr::write_volatile(GPIOA_BSRR, 0b1 << 4);
        }
    }
}
