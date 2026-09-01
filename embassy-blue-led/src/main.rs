#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::sync::atomic;
use core::sync::atomic::Ordering;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut led = Output::new(p.PB7, Level::High, Speed::Low);
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(20)).await;
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
