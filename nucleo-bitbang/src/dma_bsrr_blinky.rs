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

// fn tim6_start() {
//     embassy_stm32::rcc::enable_and_reset::<peripherals::TIM6>();
//     let tim = pac::TIM6;
//     // 16 MHz / 16000 = 1 kHz counter, update every 250 counts = 4 Hz.
//     tim.psc().write_value(16_000 - 1);
//     tim.arr().write(|w| w.set_arr(250 - 1));
//     tim.egr().write(|w| w.set_ug(true)); // latch PSC/ARR
//     tim.dier().modify(|w| w.set_ude(true)); // raise a DMA request on update
//     tim.cr1().modify(|w| w.set_cen(true));
// }

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
    // tim6_start();
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
