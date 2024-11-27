#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

#[cfg(not(feature = "async"))]
use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::{register_bank::Register, Ready, ICM42688};

#[cfg(feature = "async")]
impl<I2C> ICM42688<I2C, Ready>
where
    I2C: I2c,
{
    pub async fn reset_fifo(&mut self) {
        let mut bank0 = self.ll.bank::<0>();
        bank0
            .signal_path_reset()
            .async_modify(|_, w| w.fifo_flush(1))
            .await
            .unwrap();
    }

    pub async fn read_fifo_count(&mut self) -> u16 {
        let mut bank0 = self.ll.bank::<0>();
        let count_h = bank0
            .fifo_counth()
            .async_read()
            .await
            .unwrap()
            .fifo_count_15_8();
        let count_l = bank0
            .fifo_countl()
            .async_read()
            .await
            .unwrap()
            .fifo_count_7_0();
        ((count_h as u16) << 8) | count_l as u16
    }


    /// Read data from the FIFO
    ///
    /// NOTE: Only the 4-byte packet mode is supported
    ///
    /// Buffer must hold at least
    pub async fn read_fifo(&mut self, buffer: &mut [u32]) -> Result<usize, ()> {
        /// We read INT_STATUS, FIFO_COUNT_H, FIFO_COUNT_L, and then the data in one go
        const INT_STATUS_ADDR: u8 = crate::register_bank::bank0::INT_STATUS::ID;
        let buffer = bytemuck::cast_slice_mut::<u32, u8>(buffer);
        buffer[0] = INT_STATUS_ADDR;

        let buf = [buffer[0]];

        self.ll
            .bus
            .write_read(0x68, &buf, buffer)
            .await
            .map_err(|_| ())?;

        // Buffer now contains [INT_STATUS, FIFO_COUNT_H, FIFO_COUNT_L, DATA, DATA, ...]
        // We need to check the FIFO_COUNT and then return the number of samples read
        let fifo_count = ((buffer[1] as u16) << 8) | (buffer[2] as u16);

        Ok(fifo_count as usize / 20)
    }

    /// Direct low level access to the underlying peripheral
    pub fn ll(&mut self) -> &mut crate::ll::ICM42688<I2C> {
        &mut self.ll
    }

    pub fn release(self) -> I2C {
        self.ll.release()
    }
}

#[cfg(not(feature = "async"))]
impl<I2C> ICM42688<I2C, Ready>
where
    I2C: Write + WriteRead,
{
    pub fn reset_fifo(&mut self) {
        let mut bank0 = self.ll.bank::<0>();
        bank0
            .signal_path_reset()
            .modify(|_, w| w.fifo_flush(1))
            .unwrap();
    }

    pub fn read_fifo_count(&mut self) -> u16 {
        let mut bank0 = self.ll.bank::<0>();
        let count_h = bank0.fifo_counth().read().unwrap().fifo_count_15_8();
        let count_l = bank0.fifo_countl().read().unwrap().fifo_count_7_0();
        ((count_h as u16) << 8) | count_l as u16
    }

    pub fn read_fifo(&mut self, buffer: &mut [u32]) -> Result<usize, ()> {
        /// We read INT_STATUS, FIFO_COUNT_H, FIFO_COUNT_L, and then the data in one go
        const INT_STATUS_ADDR: u8 = crate::register_bank::bank0::INT_STATUS::ID;

        let buffer = bytemuck::cast_slice_mut::<u32, u8>(buffer);
        buffer[0] = INT_STATUS_ADDR;

        self.ll
            .bus
            .write_read(0x68, &buffer[0..1], buffer)
            .map_err(|_| ())?;

        // Buffer now contains [INT_STATUS, FIFO_COUNT_H, FIFO_COUNT_L, DATA, DATA, ...]
        // We need to check the FIFO_COUNT and then return the number of samples read
        let fifo_count = ((buffer[1] as u16) << 8) | (buffer[2] as u16);

        Ok(fifo_count as usize / 20)
    }

    /// Direct low level access to the underlying peripheral
    pub fn ll(&mut self) -> &mut crate::ll::ICM42688<I2C> {
        &mut self.ll
    }

    pub fn release(self) -> I2C {
        self.ll.release()
    }
}
