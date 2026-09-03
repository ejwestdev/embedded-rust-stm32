use defmt_rtt as _;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_time::{Duration, Timer};
pub struct BitBangUart<'a> {
    tx: Output<'a>,
    rx: Input<'a>,
    bit_duration: Duration,
}

impl<'a> BitBangUart<'a> {
    pub fn new(tx: Output<'a>, rx: Input<'a>, baud: u32) -> Self {
        let bit_duration = Duration::from_nanos(1_000_000_000 / baud as u64);
        Self {
            tx,
            rx,
            bit_duration,
        }
    }
    pub async fn start_transfer_8n1(&mut self, send_buf: &[u8], message: &[u8]) {
        for &byte in message {
            let mut ones_count = 0;
            self.tx.set_high(); //start bit
            self.tx.set_low(); // start
            Timer::after(self.bit_duration).await;

            for i in 0..8 {
                if (byte >> i) & 1 == 1 {
                    ones_count += 1;
                    self.tx.set_high();
                } else {
                    self.tx.set_low();
                }
                Timer::after(self.bit_duration).await;
            }
            if ones_count % 2 == 0 { //odd parity - if even amount of bits sent then add one
                self.tx.set_high();
                Timer::after(self.bit_duration).await;
            }
        }
        self.tx.set_high(); // stop bit 
        Timer::after(self.bit_duration).await;
        self.tx.set_low(); // reset for next byte
    }
}
pub async fn uart_bitbang(tx: Output<'_>, rx: Input<'_>) {
    let mut uart = BitBangUart::new(tx, rx, 9600);

    let msg: &[u8] = b"Hello, Uart!";
    uart.start_transfer_8n1(&[0xf9], msg).await;
}
