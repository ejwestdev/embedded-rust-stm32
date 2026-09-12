use embassy_stm32::gpio::OutputOpenDrain;
use embassy_time::{Duration, Timer};

#[derive(Debug, defmt::Format)]
pub enum I2cError {
    Nack,
}

pub struct BitBangI2C<'a> {
    sda: OutputOpenDrain<'a>,
    scl: OutputOpenDrain<'a>,
    delay: Duration,
}
impl<'a> BitBangI2C<'a> {
    pub fn new(sda: OutputOpenDrain<'a>, scl: OutputOpenDrain<'a>, half_period_ns: u32) -> Self {
        let delay = Duration::from_nanos(half_period_ns as u64);
        Self { sda, scl, delay }
    }
    pub async fn i2c_transfer(&mut self, data: u8) -> Result<(), I2cError> {
        for i in 0..8 {
            let bit = (data >> (7 - i)) & 1 == 1;
            if bit {
                self.sda.set_high();
            } else {
                self.sda.set_low();
            }
            Timer::after(self.delay).await;
            self.scl.set_high(); // slave samples the bit here
            Timer::after(self.delay).await;
            self.scl.set_low();
            Timer::after(self.delay).await;
        }

        self.sda.set_high();
        Timer::after(self.delay).await;
        self.scl.set_high();
        Timer::after(self.delay).await;
        let acked = self.sda.is_low();
        self.scl.set_low();
        Timer::after(self.delay).await;

        if acked { Ok(()) } else { Err(I2cError::Nack) }
    }
    pub async fn start_transfer(&mut self, ack_buf: &mut [u8], data_buf: &[u8]) {
        let addr = 0x3C;
        self.sda.set_low();
        Timer::after(self.delay).await;
        self.scl.set_low();
        Timer::after(self.delay).await;

        if self.i2c_transfer(addr << 1).await.is_err() {
            defmt::warn!("i2c: no ack on address phase");
        }
        for (i, &byte) in data_buf.iter().enumerate() {
            ack_buf[i] = self.i2c_transfer(byte).await.is_err() as u8;
        }
        defmt::info!("i2c ack bits (1 = nack): {:?}", ack_buf);

        self.sda.set_low();
        Timer::after(self.delay).await;
        self.scl.set_high();
        Timer::after(self.delay).await;
        self.sda.set_high();
        Timer::after(self.delay).await;
    }
}

pub async fn i2c_bitbang(sda: OutputOpenDrain<'_>, scl: OutputOpenDrain<'_>) {
    let mut i2c = BitBangI2C::new(sda, scl, 5_000);
    let data = [92, 134, 110, 97, 196, 95, 78, 243];
    let mut ack_buf = [0; 8];
    i2c.start_transfer(&mut ack_buf, &data).await;
}
