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
