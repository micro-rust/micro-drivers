//! L3GD20 gyroscope module.


#![allow(dead_code)]


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
pub enum Bandwidth {
    /// Low filter bandwidth.
    Low = 0b00,

    /// Medium filter bandwidth.
    Medium = 0b01,

    /// High filter bandwidth.
    High = 0b10,

    /// Very High filter bandwidth.
    VeryHigh = 0b11,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataRate {
    /// 95 Hz data rate.
    Hz95   = 0b00,

    /// 190 Hz data rate.
    Hz190  = 0b01,

    /// 380 Hz data rate.
    Hz380  = 0b10,

    /// 760 Hz data rate.
    Hz760  = 0b11,
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FIFOMode {
    /// FIFO is bypassed.
    Bypass = 0b000,

    /// FIFO mode.
    Fifo = 0b001,

    /// Stream mode.
    Stream = 0b010,

    /// Stream to FIFO mode.
    StreamToFifo = 0b011,

    /// Bypass to Stream mode.
    BypassToStream = 0b100,
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputSelect {
    /// Raw output. ADC -> Low Pass Filter 1 -> Output.
    Raw = 0b00,

    /// High Pass filter.
    /// ADC -> Low Pass Filter 1 -> High Pass Filter -> Output.
    /// The High Pass Filter can be bypassed and configured.
    HighPassFilter = 0b01,

    /// High Pass filter.
    /// ADC -> Low Pass Filter 1 -> High Pass Filter -> LowPass Filter 2 -> Output.
    /// The High Pass Filter can be bypassed and configured.
    /// This is the default option.
    LowPassFilter = 0b10,
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Range {
    Dps250   = 0b00,
    Dps500   = 0b01,
    Dps2000  = 0b10,
}

pub type Scale = Range;

impl Range {
    /// Returns the parameters to calculate the real acceleration.
    #[inline]
    pub(super) fn params(&self) -> f32 {
        match *self {
            Range::Dps250  => 0.00875,
            Range::Dps500  => 0.01750,
            Range::Dps2000 => 0.07000,
        }
    }
}

impl core::convert::From<u8> for Range {
    fn from(s: u8) -> Range {
        match s {
            0b00 =>  Range::Dps250,
            0b01 =>  Range::Dps500,
            0b10 =>  Range::Dps2000,
            0b11 =>  Range::Dps2000,

            _ => panic!(),
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Register {
    /// Who Am I register.
    WhoAmI = 0x0F,

    /// Control register 1.
    Ctrl1 = 0x20,
    /// Control register 2.
    Ctrl2 = 0x21,
    /// Control register 3.
    Ctrl3 = 0x22,
    /// Control register 4.
    Ctrl4 = 0x23,
    /// Control register 5.
    Ctrl5 = 0x24,

    /// Reference register.
    Reference = 0x25,

    /// Temperature Output register.
    TempOut = 0x26,

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

    /// Interrupt Configuration register.
    IntCfg = 0x30,
    /// Interrupt Source register.
    IntSrc = 0x31,

    /// Interrupt X-axis Threshold High register.
    IntXHTsh = 0x32,
    /// Interrupt X-axis Threshold Low register.
    IntXLTsh = 0x33,
    /// Interrupt Y-axis Threshold High register.
    IntYHTsh = 0x34,
    /// Interrupt Y-axis Threshold Low register.
    IntYLTsh = 0x35,
    /// Interrupt Z-axis Threshold High register.
    IntZHTsh = 0x36,
    /// Interrupt Z-axis Threshold Low register.
    IntZLTsh = 0x37,

    /// Interrupt Duration register.
    IntDur = 0x38,
}
