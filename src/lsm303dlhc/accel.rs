//! LSM303DLHC accelerometer module.


#![allow(dead_code)]



use embedded_hal::i2c::SevenBitAddress;

/// Accelerometer module I2C address.
pub(crate) const ACCEL: SevenBitAddress = 0b0011001;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Axis {
    /// X-axis.
    XAxis = 0b001,

    /// Y-axis.
    YAxis = 0b010,

    /// Z-axis.
    ZAxis = 0b100,
}

impl const core::ops::Add<Axis> for Axis {
    type Output = u8;

    #[inline(always)]
    fn add(self, rhs: Axis) -> Self::Output {
        (self as u8) | (rhs as u8)
    }
}

impl const core::ops::Add<u8> for Axis {
    type Output = u8;

    #[inline(always)]
    fn add(self, rhs: u8) -> Self::Output {
    (self as u8) | rhs
    }
}

impl const core::ops::Add<Axis> for u8 {
    type Output = u8;

    #[inline(always)]
    fn add(self, rhs: Axis) -> Self::Output {
        self | (rhs as u8)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataRate {
    /// 1 Hz data rate.
    Hz1   = 0b0001,

    /// 10 Hz data rate.
    Hz10  = 0b0010,

    /// 25 Hz data rate.
    Hz25  = 0b0011,

    /// 50 Hz data rate.
    Hz50  = 0b0100,

    /// 100 Hz data rate.
    Hz100 = 0b0101,

    /// 200 Hz data rate.
    Hz200 = 0b0110,

    /// 400 Hz data rate.
    Hz400 = 0b0111,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Scale {
    G2  = 0b00,
    G4  = 0b01,
    G8  = 0b10,
    G16 = 0b11,
}

impl core::convert::From<u8> for Scale {
    fn from(s: u8) -> Scale {
        match s {
            0b00 =>  Scale::G2,
            0b01 =>  Scale::G4,
            0b10 =>  Scale::G8,
            0b11 => Scale::G16,

            _ => panic!(),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Register {
    /// Control Register 1.
    Ctrl1 = 0x20,
    /// Control Register 2.
    Ctrl2 = 0x21,
    /// Control Register 3.
    Ctrl3 = 0x22,
    /// Control Register 4.
    Ctrl4 = 0x23,
    /// Control Register 5.
    Ctrl5 = 0x24,
    /// Control Register 6.
    Ctrl6 = 0x25,



    /// Reference value register.
    Reference = 0x26,
    /// Status register.
    Status = 0x27,

    /// X-axis LSB Output.
    OutXL = 0x28,
    /// Y-axis MSB Output.
    OutYH = 0x29,
    /// Y-axis LSB Output.
    OutYL = 0x2A,
    /// X-axis MSB Output.
    OutXH = 0x2B,
    /// Z-axis LSB Output.
    OutZL = 0x2C,
    /// Z-axis MSB Output.
    OutZH = 0x2D,



    /// FIFO Control register.
    FIFOCtrl = 0x2E,
    /// FIFO Source register.
    FIFOSrc = 0x2F,



    /// Interrupt 1 Configuration register.
    Int1Cfg = 0x30,
    /// Interrupt 1 Source register.
    Int1Src = 0x31,
    /// Interrupt 1 Threshold register.
    Int1Ths = 0x32,
    /// Interrupt 1 Duration register.
    Int1Dur = 0x33,



    /// Interrupt 2 Configuration register.
    Int2Cfg = 0x34,
    /// Interrupt 2 Source register.
    Int2Src = 0x35,
    /// Interrupt 2 Threshold register.
    Int2Ths = 0x36,
    /// Interrupt 2 Duration register.
    Int2Dur = 0x37,



    /// Click detection Configuration register.
    ClickCfg = 0x38,
    /// Click detection Source register.
    ClickSrc = 0x39,
    /// Click detection Threshold register.
    ClickThs = 0x3A,



    /// Detection time Limit.
    TimeLimit = 0x3B,
    /// Detection time Latency.
    TimeLatency = 0x3C,
    /// Detection time Window.
    TimeWindow = 0x3D,
}
