#![no_std]
#![no_main]
mod dma_bsrr_blinky;
mod i2c;
mod panic;
mod spi;
mod uart;
use crate::dma_bsrr_blinky::dma_blinky;
use crate::i2c::i2c_bitbang;
use crate::spi::spi_bitbang;
use crate::uart::uart_bitbang;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Input, Level, Output, OutputOpenDrain, Pull, Speed},
    timer::low_level::Timer,
};
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    // PB7 (blue LED) must be configured as an output, otherwise the BSRR
    // writes from the DMA only change ODR and never drive the pin.
    // `mem::forget` keeps the pin configured for the rest of the program.
    let led = Output::new(unsafe { p.PB7.clone_unchecked() }, Level::Low, Speed::Low);
    core::mem::forget(led);

    let dma = p.PB7.into();
    dma_blinky(dma, p.GPDMA1_CH0, p.TIM6).await;

    //uart
    let tx = Output::new(p.PB0, Level::High, Speed::High);
    let rx = Input::new(p.PB1, Pull::Up);
    //uart_bitbang(tx, rx).await;

    //spi
    let sck = Output::new(p.PA5, Level::Low, Speed::High);
    let pico = Output::new(p.PA7, Level::Low, Speed::High);
    let poci = Input::new(p.PA6, Pull::None);
    let cs = Output::new(p.PA4, Level::High, Speed::High);
    //spi_bitbang(sck, pico, poci, cs).await;

    //i2c
    let sda = OutputOpenDrain::new(p.PC0, Level::High, Speed::High);
    let scl = OutputOpenDrain::new(p.PC1, Level::High, Speed::High);
    //i2c_bitbang(sda, scl).await;
}
