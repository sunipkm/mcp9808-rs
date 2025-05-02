//! Device Address
const DEFAULT_ADDRESS: u8 = 0b1_1000;

/// I2C device address
///
/// # Example
///
/// ## Default slave address
/// ```
/// use mcp9808::address::SlaveAddress;
///
/// # let DefaultAddress = SlaveAddress::Default.into();
/// assert_eq!(0b1_1000u8, DefaultAddress);
/// ```
///
/// ## Alternative slave address
/// ```
/// use mcp9808::address::SlaveAddress;
///
/// # let TestAddress = SlaveAddress::Alternative { a2: true, a1: false, a0: true }.into();
/// assert_eq!(0b1_1101u8, TestAddress);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum SlaveAddress {
    /// Default slave address 0b1_1000 - all pins are disconnected
    Default,
    /// Alternative slave address with configurable pins A2, A1, A0.
    /// The address pins correspond to the Least Significant
    /// bits (LSbs) of the address bits.
    Alternative { a2: bool, a1: bool, a0: bool },
}

impl From<SlaveAddress> for u8 {
    fn from(slave_address: SlaveAddress) -> Self {
        match slave_address {
            SlaveAddress::Default => DEFAULT_ADDRESS,
            SlaveAddress::Alternative { a2, a1, a0 } => {
                DEFAULT_ADDRESS | (a2 as u8) << 2 | (a1 as u8) << 1 | (a0 as u8)
            }
        }
    }
}

impl SlaveAddress {
    /// Convert a u8 value to a SlaveAddress.
    pub const fn from_u8(value: u8) -> Result<SlaveAddress, u8> {
        match value {
            DEFAULT_ADDRESS => Ok(SlaveAddress::Default),
            0b1_1001..=0b1_1111 => Ok(SlaveAddress::Alternative {
                a2: (value >> 2) & 0b1 == 1,
                a1: (value >> 1) & 0b1 == 1,
                a0: value & 0b1 == 1,
            }),
            _ => Err(value),
        }
    }
}

impl PartialEq for SlaveAddress {
    fn eq(&self, other: &Self) -> bool {
        let (lhs, rhs): (u8, u8) = ((*self).into(), (*other).into());
        lhs == rhs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_address() {
        let default_address = SlaveAddress::Default.into();
        assert_eq!(0b1_1000u8, default_address);
    }

    #[test]
    fn test_alternative_address() {
        let alternative_address = SlaveAddress::Alternative {
            a2: true,
            a1: false,
            a0: true,
        }
        .into();
        assert_eq!(0b1_1101u8, alternative_address);
    }

    #[test]
    fn test_invalid_address() {
        assert_eq!(SlaveAddress::from_u8(0b1_0000), Err(0b1_0000));
        assert_eq!(
            SlaveAddress::from_u8(0b1_1111),
            Ok(SlaveAddress::Alternative {
                a2: true,
                a1: true,
                a0: true
            })
        );
        assert_eq!(
            SlaveAddress::from_u8(0b1_1001),
            Ok(SlaveAddress::Alternative {
                a2: false,
                a1: false,
                a0: true
            })
        );
        assert_eq!(
            SlaveAddress::from_u8(0b1_1010),
            Ok(SlaveAddress::Alternative {
                a2: false,
                a1: true,
                a0: false
            })
        );
        assert_eq!(
            SlaveAddress::from_u8(0b1_1100),
            Ok(SlaveAddress::Alternative {
                a2: true,
                a1: false,
                a0: false
            })
        );
        assert_eq!(
            SlaveAddress::from_u8(0b1_1110),
            Ok(SlaveAddress::Alternative {
                a2: true,
                a1: true,
                a0: false
            })
        );
        assert_eq!(
            SlaveAddress::from_u8(0b1_1011),
            Ok(SlaveAddress::Alternative {
                a2: false,
                a1: true,
                a0: true
            })
        );
        assert_eq!(
            SlaveAddress::from_u8(0b1_1101),
            Ok(SlaveAddress::Alternative {
                a2: true,
                a1: false,
                a0: true
            })
        );
        assert_eq!(SlaveAddress::from_u8(0b1_1000), Ok(SlaveAddress::Default));
        assert_eq!(SlaveAddress::from_u8(0b1_0001), Err(0b1_0001));
        assert_eq!(SlaveAddress::from_u8(0x51), Err(0x51));
    }
}
