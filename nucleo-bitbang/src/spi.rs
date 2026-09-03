use defmt_rtt as _;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_time::{Duration, Timer};
#[allow(dead_code)]
pub struct BitBangSpi<'a> {
    sck: Output<'a>,
    pico: Output<'a>,
    poci: Input<'a>,
    cs: Output<'a>,
    delay: Duration,
}

impl<'a> BitBangSpi<'a> {
    pub fn new(
        sck: Output<'a>,
        pico: Output<'a>,
        poci: Input<'a>,
        cs: Output<'a>,
        time: u32,
    ) -> Self {
        let delay = Duration::from_nanos(time as u64);
        Self {
            sck,
            pico,
            poci,
            cs,
            delay,
        }
    }
    pub async fn spi_transfer(&mut self, data: u8) -> u8 {
        let mut received: u8 = 0;
        for i in 0..8 {
            let bit = (data >> (7 - i)) & 1 == 1;
            //the POCI in production would read this
            if bit {
                self.pico.set_high();
            } else {
                self.pico.set_low();
            }

            Timer::after(self.delay).await;
            self.sck.set_high();
            Timer::after(self.delay).await;

            //in production we would get from poci here
            if self.pico.is_set_high() {
                received |= 1 << (7 - i); //msb
            }
            self.sck.set_low();
        }
        defmt::info!("{}", received);
        received
    }

    pub async fn start_transfer(&mut self, send_buf: &[u8], recv_buf: &mut [u8]) {
        self.cs.set_low();
        for (i, &byte) in send_buf.iter().enumerate() {
            recv_buf[i] = self.spi_transfer(byte).await;
        }
        self.cs.set_high();
    }
}
pub async fn spi_bitbang(sck: Output<'_>, pico: Output<'_>, poci: Input<'_>, cs: Output<'_>) {
    let mut spi = BitBangSpi::new(sck, pico, poci, cs, 500);

    let mut recv_buf = [1, 1, 1, 1, 1, 1, 1, 1];
    spi.start_transfer(&[0xf9], &mut recv_buf).await;
}
