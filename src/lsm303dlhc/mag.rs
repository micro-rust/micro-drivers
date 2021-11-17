//! LSM303DLHC magnetometer module.

#![allow(dead_code)]




use embedded_hal::i2c::SevenBitAddress;

/// Magnetometer module I2C address.
pub(crate) const MAG: SevenBitAddress   = 0b0011110;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataRate {
    /// 0.75 Hz data rate.
    Hz0_75 = 0b000,

    /// 1.5 Hz data rate.
    Hz1_5  = 0b001,

    /// 3 Hz data rate.
    Hz3_0  = 0b010,

    /// 7.5 Hz data rate.
    Hz7_5  = 0b011,

    /// 15 Hz data rate.
    Hz15   = 0b100,

    /// 30 Hz data rate.
    Hz30   = 0b101,

    /// 75 Hz data rate.
    Hz75   = 0b110,

    /// 220 Hz data rate.
    Hz220  = 0b111,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Scale {
    /// 1.3 Gauss scale.
    Gauss1_3 = 0b001,

    /// 1.9 Gauss scale.
    Gauss1_9 = 0b010,

    /// 2.5 Gauss scale.
    Gauss2_5 = 0b011,

    /// 4.0 Gauss scale.
    Gauss4_0 = 0b100,

    /// 4.7 Gauss scale.
    Gauss4_7 = 0b101,

    /// 5.6 Gauss scale.
    Gauss5_6 = 0b110,

    /// 8.1 Gauss scale.
    Gauss8_1 = 0b111,
}

impl core::convert::From<u8> for Scale {
    fn from(s: u8) -> Scale {
        match s {
            0b001 => Scale::Gauss1_3,
            0b010 => Scale::Gauss1_9,
            0b011 => Scale::Gauss2_5,
            0b100 => Scale::Gauss4_0,
            0b101 => Scale::Gauss4_7,
            0b110 => Scale::Gauss5_6,
            0b111 => Scale::Gauss8_1,

            _ => panic!(),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Register {
    /// Control register A.
    Cra = 0x00,
    /// Control register B.
    Crb = 0x01,



    /// Operating mode register.
    Mr = 0x02,



    /// X-axis MSB output.
    OutXH = 0x03,
    /// X-axis LSB output.
    OutXL = 0x04,
    /// Z-axis MSB output.
    OutZH = 0x05,
    /// Z-axis LSB output.
    OutZL = 0x06,
    /// Y-axis MSB output.
    OutYH = 0x07,
    /// Y-axis LSB output.
    OutYL = 0x08,

    /// Status register.
    Status = 0x09,

    /// Interrupt A register.
    IntA = 0x0A,
    /// Interrupt B register.
    IntB = 0x0B,
    /// Interrupt C register.
    IntC = 0x0C,

    /// Temperature MSB output.
    TempOutH = 0x31,
    /// Temperature LSB output.
    TempOutL = 0x32,
}
