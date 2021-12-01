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
pub enum Range {
    /// 1.3 Gauss Range.
    Gauss1_3 = 0b001,

    /// 1.9 Gauss Range.
    Gauss1_9 = 0b010,

    /// 2.5 Gauss Range.
    Gauss2_5 = 0b011,

    /// 4.0 Gauss Range.
    Gauss4_0 = 0b100,

    /// 4.7 Gauss Range.
    Gauss4_7 = 0b101,

    /// 5.6 Gauss Range.
    Gauss5_6 = 0b110,

    /// 8.1 Gauss Range.
    Gauss8_1 = 0b111,
}

pub type Scale = Range;

impl Range {
    /// Returns the parameters to calculate the real magnetic field value.
    pub(super) fn params(&self) -> (i16, i16) {
        match *self {
            Range::Gauss1_3 => (1100, 980),
            Range::Gauss1_9 => ( 855, 760),
            Range::Gauss2_5 => ( 670, 600),
            Range::Gauss4_0 => ( 450, 400),
            Range::Gauss4_7 => ( 400, 355),
            Range::Gauss5_6 => ( 330, 295),
            Range::Gauss8_1 => ( 230, 205),
        }
    }
}

impl core::convert::From<u8> for Range {
    fn from(s: u8) -> Range {
        match s {
            0b001 => Range::Gauss1_3,
            0b010 => Range::Gauss1_9,
            0b011 => Range::Gauss2_5,
            0b100 => Range::Gauss4_0,
            0b101 => Range::Gauss4_7,
            0b110 => Range::Gauss5_6,
            0b111 => Range::Gauss8_1,

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

    /// ID A register.
    IdA = 0x0A,
    /// ID B register.
    IdB = 0x0B,
    /// ID C register.
    IdC = 0x0C,

    /// Temperature MSB output.
    TempOutH = 0x31,
    /// Temperature LSB output.
    TempOutL = 0x32,
}
