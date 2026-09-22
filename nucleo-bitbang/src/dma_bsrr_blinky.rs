use embassy_stm32::{
    Peri,
    gpio::{AnyPin, Output, Pin},
};
use embassy_time::{Duration, Timer};

pub async fn bsrr_set(pin: u8) -> usize {
    1 << pin
}

pub async fn bsrr_rst(pin: u8) -> usize {
    1 << (pin + 16)
}
pub async fn dma_blinky(target: Peri<'_, AnyPin>) {
    let x = target.pin();
    let delay = Duration::from_millis(500);
    loop {
        bsrr_set(x).await;
        Timer::after(delay).await;
        bsrr_rst(x).await;
    }
}
