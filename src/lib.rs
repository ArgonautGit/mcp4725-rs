#![no_std]

//! MCP4725 DAC driver library
//!
//! This library provides both blocking and async interfaces for the MCP4725 12-bit DAC.

use embedded_hal::i2c::I2c as I2c_Blocking;
use embedded_hal::i2c::SevenBitAddress;
use embedded_hal_async::i2c::I2c as I2c_Async;

pub trait Dac {}

pub trait DacBlocking: Dac {
    type Error;

    fn set_voltage_blocking(&mut self, voltage: f32) -> Result<(), Self::Error>;
    fn get_voltage_blocking(&mut self) -> Result<f32, Self::Error>;
}

#[allow(async_fn_in_trait)]
pub trait DacAsync: Dac {
    type Error;

    async fn set_voltage(&mut self, voltage: f32) -> Result<(), Self::Error>;
    async fn get_voltage(&mut self) -> Result<f32, Self::Error>;
}

pub struct Mcp4725<I2C> {
    address: SevenBitAddress,
    supply_voltage: f32,
    i2c: I2C,
}

impl<I2C> Dac for Mcp4725<I2C> {}

impl<I2C> Mcp4725<I2C> {
    pub fn new(address: SevenBitAddress, supply_voltage: f32, i2c: I2C) -> Self {
        Mcp4725 {
            address,
            supply_voltage,
            i2c,
        }
    }
}

impl<I2C: I2c_Async> DacAsync for Mcp4725<I2C> {
    type Error = I2C::Error;

    async fn set_voltage(&mut self, voltage: f32) -> Result<(), Self::Error> {
        #[cfg(feature = "defmt")]
        defmt::debug!("Setting voltage: {}", voltage);

        let voltage: u16 = ((voltage / self.supply_voltage) * 4095_f32) as u16;
        let bytes = [(voltage >> 8) as u8, (voltage & 0xFF) as u8];

        self.i2c.write(self.address, &bytes).await
    }

    async fn get_voltage(&mut self) -> Result<f32, Self::Error> {
        let mut bytes = [0; 5];
        self.i2c.read(self.address, &mut bytes).await?;

        let result: u16 = u16::from_be_bytes([bytes[1], bytes[2]]) >> 4;
        let result: f32 = (result as f32 / 4095_f32) * self.supply_voltage;

        #[cfg(feature = "defmt")]
        defmt::debug!("Got voltage: {}", result);

        Ok(result)
    }
}

impl<I2C: I2c_Blocking> DacBlocking for Mcp4725<I2C> {
    type Error = I2C::Error;

    fn set_voltage_blocking(&mut self, voltage: f32) -> Result<(), Self::Error> {
        #[cfg(feature = "defmt")]
        defmt::trace!("Setting voltage: {} V", voltage);
        let voltage: u16 = ((voltage / self.supply_voltage) * 4095_f32) as u16;
        let bytes = [(voltage >> 8) as u8, (voltage & 0xFF) as u8];

        self.i2c.write(self.address, &bytes)
    }

    fn get_voltage_blocking(&mut self) -> Result<f32, Self::Error> {
        let mut bytes = [0; 5];
        self.i2c.read(self.address, &mut bytes)?;

        let result: u16 = u16::from_be_bytes([bytes[1], bytes[2]]) >> 4;
        let result: f32 = (result as f32 / 4095_f32) * self.supply_voltage;

        #[cfg(feature = "defmt")]
        defmt::debug!("Got voltage: {}", result);

        Ok(result)
    }
}
