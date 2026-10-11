use embassy_stm32::{
    Peri, bind_interrupts,
    dma::{Channel, TransferOptions},
    gpio::{AnyPin, Pin},
    pac,
    peripherals::{self, TIM6},
    time::Hertz,
    timer::low_level::{MasterMode, RoundTo::Faster, Timer},
};

bind_interrupts!(struct Irqs {
    GPDMA1_CHANNEL0 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH0>;
});

const TIM6_UPDATE_REQUEST: u8 = 4;

pub const fn bsrr_set(pin: u8) -> u32 {
    1 << pin
}

pub const fn bsrr_rst(pin: u8) -> u32 {
    1 << (pin + 16)
}

pub async fn dma_blinky(
    target: Peri<'_, AnyPin>,
    dma_ch: Peri<'_, peripherals::GPDMA1_CH0>,
    timer_pin: Peri<'_, TIM6>,
) {
    let pin = target.pin();
    let mut ch = Channel::new(dma_ch, Irqs);

    let pattern: [u32; 6] = [
        bsrr_set(pin),
        bsrr_rst(pin),
        bsrr_set(pin),
        bsrr_set(pin),
        bsrr_rst(pin),
        bsrr_rst(pin),
    ];
    let options = TransferOptions::default();
    let peri_addr = pac::GPIOB.bsrr().as_ptr() as *mut u32;
    let timer = Timer::new(timer_pin);
    timer.set_frequency(Hertz(4), Faster);
    timer.enable_update_dma(true);
    timer.start();
    loop {
        unsafe {
            ch.write(TIM6_UPDATE_REQUEST, &pattern, peri_addr, options)
                .await;
        };
    }
}
