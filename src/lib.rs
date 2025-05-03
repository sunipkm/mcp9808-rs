#![deny(warnings)]
#![no_std]

extern crate bit_field;
extern crate cast;
extern crate embedded_hal;

use core::marker::PhantomData;

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
pub struct MCP9808<I2C> {
    addr: u8,
    res: ResolutionVal,
    _marker: PhantomData<I2C>,
}

impl<I2C> MCP9808<I2C> {
    /// Change i2c address
    pub fn set_address(&mut self, addr: SlaveAddress) -> u8 {
        self.addr = addr.into();
        self.addr
    }

    /// Get the current temperature resolution
    #[inline(always)]
    pub fn resolution(&self) -> ResolutionVal {
        self.res
    }
}

impl<I2C> MCP9808<I2C>
where
    I2C: I2c<SevenBitAddress>,
    I2C::Error: Into<Error<I2C::Error>>,
{
    pub fn new(
        addr: SlaveAddress,
        res: ResolutionVal,
        i2c: &mut I2C,
    ) -> Result<Self, Error<I2C::Error>> {
        let mut mcp = MCP9808 {
            addr: addr.into(),
            res,
            _marker: PhantomData,
        };
        let mut conf = reg_conf::new();
        conf.set_resolution(res);
        mcp.write_register(conf, i2c)?;
        Ok(mcp)
    }

    fn read_register<T>(&mut self, mut reg: T, i2c: &mut I2C) -> Result<T, Error<I2C::Error>>
    where
        T: prelude::Read,
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        reg.read_from_device(i2c, self.addr)?;
        Ok(reg)
    }

    pub fn write_register<R: prelude::Write>(
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

    pub fn read_configuration(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<impl Configuration, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_conf::new(), i2c)
    }

    pub fn read_device_id(&mut self, i2c: &mut I2C) -> Result<impl DeviceId, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_device_id::new(), i2c)
    }

    pub fn read_manufacturer_id(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<impl ManufacturerId, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_manuf_id::new(), i2c)
    }

    pub fn read_resolution(&mut self, i2c: &mut I2C) -> Result<impl Resolution, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_res::new(), i2c)
    }

    /// Read temperature register. Its double-buffered so no wait required.
    pub fn read_temperature(&mut self, i2c: &mut I2C) -> Result<impl Temperature, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_temp::new(), i2c)
    }

    pub fn read_alert_critical(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<impl CriticalTemperatureAlert, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_temp_alert_crit::new(), i2c)
    }

    pub fn read_alert_lower(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<impl LowerTemperatureAlert, Error<I2C::Error>>
    where
        I2C: I2c<SevenBitAddress>,
        I2C::Error: Into<Error<I2C::Error>>,
    {
        self.read_register(reg_temp_alert_lower::new(), i2c)
    }

    pub fn read_alert_upper(
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
