#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::sync::atomic;
use core::sync::atomic::Ordering;
use cortex_m::asm::nop;
use cortex_m_rt::entry;

const RCC_AHB2ENR1: *mut u32 = 0x4602_0C8C as *mut u32;
const GPIOB_MODER: *mut u32 = 0x4202_0400 as *mut u32;
const GPIOB_BSRR: *mut u32 = 0x4202_0418 as *mut u32;

#[entry]
fn main() -> ! {
    unsafe {
        // Enable GPIOB clock
        let v = RCC_AHB2ENR1.read_volatile();
        RCC_AHB2ENR1.write_volatile(v | (1 << 1));
        let _ = RCC_AHB2ENR1.read_volatile(); // wait 2 cycles 

        // Set PB7 to output mode (bits 14-15 = 0b01)
        let v = GPIOB_MODER.read_volatile();
        GPIOB_MODER.write_volatile((v & !(0b11 << 14)) | (0b01 << 14));
        let _ = GPIOB_MODER.read_volatile(); // wait 2 cycles

        // Set PB7 high via BSRR (bit 7)
        GPIOB_BSRR.write_volatile(1 << 7);
    }
    loop {
        nop();
    }
}
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
