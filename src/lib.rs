#![no_std]

//! MCP4725 DAC driver library
//!
//! This library provides both blocking and async interfaces for the MCP4725 12-bit DAC.

use embedded_hal::i2c::SevenBitAddress;

pub struct Mcp4725 {
    address: SevenBitAddress,
    supply_voltage: f32,
}

pub trait Dac {
    fn new(address: SevenBitAddress, supply_voltage: f32) -> Self;
}

impl Dac for Mcp4725 {
    fn new(address: SevenBitAddress, supply_voltage: f32) -> Self {
        Mcp4725 {
            address,
            supply_voltage,
        }
    }
}

pub trait DacBlocking: Dac {
    fn set_voltage_blocking<I2C>(&mut self, i2c: &mut I2C, voltage: f32) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c;

    fn get_voltage_blocking<I2C>(&mut self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c;
}

#[allow(async_fn_in_trait)]
pub trait DacAsync: Dac {
    async fn set_voltage<I2C>(&mut self, i2c: &mut I2C, voltage: f32) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal_async::i2c::I2c;

    async fn get_voltage<I2C>(&mut self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: embedded_hal_async::i2c::I2c;
}

impl DacBlocking for Mcp4725 {
    fn set_voltage_blocking<I2C>(&mut self, i2c: &mut I2C, voltage: f32) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        #[cfg(feature = "defmt")]
        defmt::trace!("Setting voltage: {} V", voltage);
        let voltage: u16 = ((voltage / self.supply_voltage) * 4095 as f32) as u16;
        let bytes = [(voltage >> 8) as u8, (voltage & 0xFF) as u8];

        i2c.write(self.address, &bytes)
    }

    fn get_voltage_blocking<I2C>(&mut self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let mut bytes = [0; 5];
        i2c.read(self.address, &mut bytes)?;

        let result: u16 = u16::from_be_bytes([bytes[1], bytes[2]]) >> 4;
        let result: f32 = (result as f32 / 4085 as f32) * 3.3;

        #[cfg(feature = "defmt")]
        defmt::debug!("Got voltage: {}", result);

        Ok(result)
    }
}

impl DacAsync for Mcp4725 {
    async fn set_voltage<I2C>(&mut self, i2c: &mut I2C, voltage: f32) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal_async::i2c::I2c,
    {
        #[cfg(feature = "defmt")]
        defmt::debug!("Setting voltage: {}", voltage);

        let voltage: u16 = ((voltage / self.supply_voltage) * 4095 as f32) as u16;
        let bytes = [(voltage >> 8) as u8, (voltage & 0xFF) as u8];

        i2c.write(self.address, &bytes).await
    }

    async fn get_voltage<I2C>(&mut self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: embedded_hal_async::i2c::I2c,
    {
        let mut bytes = [0; 5];
        i2c.read(self.address, &mut bytes).await?;

        let result: u16 = u16::from_be_bytes([bytes[1], bytes[2]]) >> 4;
        let result: f32 = (result as f32 / 4085 as f32) * 3.3;

        #[cfg(feature = "defmt")]
        defmt::debug!("Got voltage: {}", result);
        
        Ok(result)
    }
}
