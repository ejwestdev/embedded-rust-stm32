#![no_std]
#![no_main]
mod i2c;
mod panic;
mod spi;
mod uart;
use crate::i2c::i2c_bitbang;
use crate::spi::spi_bitbang;
use crate::uart::uart_bitbang;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, OutputOpenDrain, Pull, Speed};
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    //uart
    let tx = Output::new(p.PB0, Level::High, Speed::High);
    let rx = Input::new(p.PB1, Pull::Up);
    //spi
    let sck = Output::new(p.PA5, Level::Low, Speed::High);
    let pico = Output::new(p.PA7, Level::Low, Speed::High);
    let poci = Input::new(p.PA6, Pull::None);
    let cs = Output::new(p.PA4, Level::High, Speed::High);
    //i2c
    let sda = OutputOpenDrain::new(p.PC0, Level::High, Speed::High);
    let scl = OutputOpenDrain::new(p.PC1, Level::High, Speed::High);
    spi_bitbang(sck, pico, poci, cs).await;
    uart_bitbang(tx, rx).await;
    i2c_bitbang(sda, scl).await;
}
