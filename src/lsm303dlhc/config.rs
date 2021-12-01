//! Configuration structure for the LSM303DLHC device.


use super::accel;
use super::mag;



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

    /// Ctrl6 register.
    pub(super) ctrl6: u8,



    /// Cra register.
    pub cra: u8,

    /// Crb register.
    pub(super) crb: u8,

    /// Mr register.
    pub(super) mr: u8,
}

impl Config {
    /// Static initializer.
    pub const fn new() -> Self {
        Self::default()
    }

    /// Sets the enabled axis for the accelerometer.
    #[inline(always)]
    pub const fn axis(mut self, axis: u8) -> Self {
        self.ctrl1 |= axis & 0x7;
        self
    }

    /// Sets the LSM303DLHC in low power mode.
    #[inline(always)]
    pub const fn lowpower(mut self) -> Self {
        self.ctrl1 |= 1 << 3;
        self
    }

    /// Sets the LSM303DLHC in normal power mode.
    #[inline(always)]
    pub const fn normal(mut self) -> Self {
        self.ctrl1 &= !(1 << 3);
        self.ctrl4 &= !(1 << 3);
        self
    }

    /// Sets the LSM303DLHC in high resolution mode.
    #[inline(always)]
    pub const fn highres(mut self) -> Self {
        self.ctrl1 &= !(1 << 3);
        self.ctrl4 |= 1 << 3;
        self
    }

    /// Sets the accelerator and magnetometer output data rates.
    #[inline(always)]
    pub const fn datarate(mut self, accel: Option<accel::DataRate>, mag: Option<mag::DataRate>) -> Self {
        if let Some(accel) = accel {
            self.ctrl1 &= !0xF0;
            self.ctrl1 |= (accel as u8) << 4;
        }

        if let Some(mag) = mag {
            self.cra &= !(0x7 << 2);
            self.cra |= (mag as u8) << 2;

            self.mr &= 0xFC;
        }

        self
    }

    /// Sets the accelerator and magnetometer scale.
    #[inline(always)]
    pub const fn scale(mut self, accel: Option<accel::Scale>, mag: Option<mag::Scale>) -> Self {
        if let Some(accel) = accel {
            self.ctrl4 &= !(0x3 << 4);
            self.ctrl4 |= (accel as u8) << 4;
        }

        if let Some(mag) = mag {
            self.crb &= !(0x7 << 5);
            self.crb |= (mag as u8) << 5;
        }

        self
    }

    /// Enables / Disables temperature reading.
    #[inline(always)]
    pub const fn temperature(mut self, s: bool) -> Self {
        if s { self.cra |= 0x80 }
        else { self.cra &= 0x7F }

        self
    }


    /// Returns the parameters.
    pub fn params(&self) -> ((accel::Mode, accel::Range), mag::Range) {
        // Get the accelerometer mode.
        let mode = match (self.ctrl1 >> 3) & 1 {
            0 => match (self.ctrl4 >> 3) & 1 {
                0 => accel::Mode::Normal,
                _ => accel::Mode::HighResolution,
            },
            _ => accel::Mode::LowPower,
        };

        // Get the accelerometer range.
        let arange = accel::Range::from( (self.ctrl4 >> 4) & 0x3 );

        // Get the magnetometer range.
        let mrange = mag::Range::from( (self.crb >> 5) & 0x7 );


        (
            (
                mode,
                arange,
            ),

            mrange,
        )
    }
}

impl const Default for Config {
    fn default() -> Self {
        Config {
            ctrl1: 0b00000000,
            ctrl2: 0b00000000,
            ctrl3: 0b00000000,
            ctrl4: 0b10001000,
            ctrl5: 0b01000000,
            ctrl6: 0b00000000,



            cra: 0b10000000,
            crb: 0b00100000,
            mr:  0b00000011,
        }
    }
}
