#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![cfg_attr(not(doctest), doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md")))]

pub mod fifo;
pub mod ll;
pub mod ready;
pub mod register_bank;
pub mod uninitialized;

#[derive(Debug)]
pub struct Uninitialized;

/// Indicates that the `ICM42688` instance is ready to be used
#[derive(Debug)]
pub struct Ready;

/// ICM42688 top-level driver
///
/// Usage:
///
/// ```rust,ignore
/// # use async_std::prelude::*; // Just for the runtime
/// # use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
/// # use embedded_hal_mock::eh1::delay::NoopDelay as Delay;
/// # #[async_std::main]
/// async fn main() {
///     // Example using I2C
///     let i2c = I2cMock::new(&[]);
///     let mut icm = icm426xx::ICM42688::new(i2c);
///     let mut icm = icm.initialize(Delay).await.unwrap();
///
/// }
/// ```
pub struct ICM42688<BUS, State> {
    ll: crate::ll::ICM42688<BUS>,
    _state: State,
}
