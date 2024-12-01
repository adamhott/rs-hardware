#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::i2c::{self, I2c};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, peripherals};
use embassy_time::{Timer, Duration};
use embedded_hal_async::i2c::I2c as HalI2c;
use embedded_hal_async::delay::DelayNs;
use icm426xx::{ICM42688, Ready, Uninitialized};
use panic_halt as _;

const ICM42688_ADDRESS: u8 = 0x68;

pub struct IcmDelay;

impl IcmDelay {
    pub fn new() -> Self {
        IcmDelay
    }
}

impl DelayNs for IcmDelay {
    async fn delay_ns(&mut self, ns: u32) {
        Timer::after(Duration::from_nanos(ns as u64)).await;
    }

    async fn delay_us(&mut self, us: u32) {
        Timer::after(Duration::from_micros(us as u64)).await;
    }

    async fn delay_ms(&mut self, ms: u32) {
        Timer::after(Duration::from_millis(ms as u64)).await;
    }
}

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Starting ICM42688 example!");

    let p = embassy_stm32::init(Default::default());

    let i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        p.DMA1_CH4,
        p.DMA1_CH5,
        Hertz(400_000),
        Default::default(),
    );

    let mut sensor = ICM42688::new(i2c);
    let mut delay = IcmDelay::new();

    match sensor.initialize(&mut delay).await {
        Ok(mut ready_sensor) => {
            defmt::info!("Sensor initialized successfully!");

            let count = ready_sensor.read_fifo_count().await; // Directly returns u16
            defmt::info!("FIFO count: {}", count);

            let mut buffer = [0u32; 32];
            match ready_sensor.read_fifo(&mut buffer).await {
                Ok(samples) => defmt::info!("Read {} samples from FIFO", samples),
                Err(_) => defmt::error!("Failed to read FIFO data"),
            }
        },
        Err(e) => {
            defmt::error!("Failed to initialize sensor: {:?}", e);
        }
    }
}
