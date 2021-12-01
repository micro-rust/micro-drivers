//! Configuration structure for the L3GD20 device.


use super::gyro;


#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// Ctrl1 register.
    pub(super) ctrl1: u8,

    /// Ctrl2 register.
    pub(super) ctrl2: u8,

    /// Ctrl3 register.
    pub(super) ctrl3: u8,

    /// Ctrl4 register.
    pub(super) ctrl4: u8,

    /// Ctrl5 register.
    pub(super) ctrl5: u8,
}


impl Config {
    /// Static initializer.
    pub const fn new() -> Self {
        Self::default()
    }

    /// Sets the enabled axis for the gyroscope.
    #[inline(always)]
    pub const fn axis(mut self, axis: u8) -> Self {
        self.ctrl1 &= !0x7;
        self.ctrl1 |= axis & 0x7;
        self
    }

    /// Sets the gyroscope output data rates.
    #[inline(always)]
    pub const fn datarate(mut self, gyro: gyro::DataRate) -> Self {
        self.ctrl1 &= !0xC0;
        self.ctrl1 |= (gyro as u8) << 6;

        self
    }

    /// Sets the gyroscope filter bandwidth.
    #[inline(always)]
    pub const fn bandwidth(mut self, gyro: gyro::Bandwidth) -> Self {
        self.ctrl1 &= !0x30;
        self.ctrl1 |= (gyro as u8) << 4;

        self
    }

    /// Sets the accelerator and magnetometer scale.
    #[inline(always)]
    pub const fn scale(mut self, gyro: gyro::Scale) -> Self {
        self.ctrl4 &= !(0x3 << 4);
        self.ctrl4 |= (gyro as u8) << 4;

        self
    }

    /// Returns the parameters.
    pub fn params(&self) -> gyro::Range {
        // Get the gyroscope range.
        gyro::Range::from( (self.ctrl4 >> 4) & 0x3 )
    }
}

impl const Default for Config {
    fn default() -> Self {
        Config {
            ctrl1: 0b00001111,
            ctrl2: 0b00000000,
            ctrl3: 0b00000000,
            ctrl4: 0b10000000,
            ctrl5: 0b01000000,
        }
    }
}
