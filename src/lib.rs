#![deny(warnings)]
#![no_std]

extern crate bit_field;
extern crate cast;
extern crate embedded_hal;

use crate::address::SlaveAddress;
use crate::error::Error;
use crate::reg_conf::Configuration;
use crate::reg_device_id::DeviceId;
use crate::reg_manuf_id::ManufacturerId;
use crate::reg_res::Resolution;
use crate::reg_res::ResolutionVal;
use crate::reg_temp::Temperature;
use crate::reg_temp_alert_crit::CriticalTemperatureAlert;
use crate::reg_temp_alert_lower::LowerTemperatureAlert;
use crate::reg_temp_alert_upper::UpperTemperatureAlert;
use embedded_hal::i2c::{I2c, SevenBitAddress};

pub mod address;
pub mod error;
mod prelude;
pub mod reg;
pub mod reg_conf;
pub mod reg_device_id;
pub mod reg_manuf_id;
pub mod reg_res;
pub mod reg_temp;
pub mod reg_temp_alert_crit;
pub mod reg_temp_alert_lower;
pub mod reg_temp_alert_upper;
pub mod reg_temp_generic;

/// MCP9808 Driver
pub struct MCP9808 {
    addr: u8,
    res: ResolutionVal,
}

impl MCP9808 {
    /// Change i2c address
    pub fn set_address(&mut self, addr: SlaveAddress) -> u8 {
        self.addr = addr.into();
        self.addr
    }

    pub fn new<I2C>(
        addr: SlaveAddress,
        res: ResolutionVal,
        i2c: &mut I2C,
    ) -> Result<Self, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        let mut mcp = MCP9808 {
            addr: addr.into(),
            res,
        };
        let mut conf = reg_conf::new();
        conf.set_resolution(res);
        mcp.write_register(conf, i2c)?;
        Ok(mcp)
    }

    /// Get the current temperature resolution
    #[inline(always)]
    pub fn resolution(&self) -> ResolutionVal {
        self.res
    }

    fn read_register<T, I2C>(&mut self, mut reg: T, i2c: &mut I2C) -> Result<T, Error<I2C::Error>>
    where
        T: prelude::Read,
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        reg.read_from_device(i2c, self.addr)?;
        Ok(reg)
    }

    pub fn write_register<R: prelude::Write, I2C>(
        &mut self,
        reg: R,
        i2c: &mut I2C,
    ) -> Result<(), Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        reg.write_to_device(i2c, self.addr)?;
        Ok(())
    }

    pub fn read_configuration<I2C>(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<impl Configuration, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_conf::new(), i2c)
    }

    pub fn read_device_id<I2C>(&mut self, i2c: &mut I2C) -> Result<impl DeviceId, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_device_id::new(), i2c)
    }

    pub fn read_manufacturer_id<I2C>(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<impl ManufacturerId, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_manuf_id::new(), i2c)
    }

    pub fn read_resolution<I2C>(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<impl Resolution, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_res::new(), i2c)
    }

    /// Read temperature register. Its double-buffered so no wait required.
    pub fn read_temperature<I2C>(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<impl Temperature, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_temp::new(), i2c)
    }

    pub fn read_alert_critical<I2C>(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<impl CriticalTemperatureAlert, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_temp_alert_crit::new(), i2c)
    }

    pub fn read_alert_lower<I2C>(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<impl LowerTemperatureAlert, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_temp_alert_lower::new(), i2c)
    }

    pub fn read_alert_upper<I2C>(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<impl UpperTemperatureAlert, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_temp_alert_upper::new(), i2c)
    }
}
