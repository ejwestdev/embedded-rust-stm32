#![no_std]
#![no_main]
mod panic;
mod spi;
use crate::spi::spi_bitbang_test as spi_bitbang;
use embassy_executor::Spawner;
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    spi_bitbang(p).await;
}
