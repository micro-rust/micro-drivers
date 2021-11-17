//! LSM303DLHC accelerometer, magnetometer and thermometer device.


pub mod accel;
pub mod mag;

mod config;


pub use self::config::Config;

use crate::{ Accelerometer, Magnetometer, Thermometer };

use core::{ mem::MaybeUninit, ops::* };

use embedded_hal::i2c::SevenBitAddress;
use embedded_hal::i2c::blocking::{
    Write, WriteRead
};



/// LSM303DLHC driver.
pub struct Lsm303dlhc<I> {
    /// I2C interface.
    interface: I,

    /// Scales of the accelerometer and magnetometer.
    scale: (accel::Scale, mag::Scale),
}

impl<I: Write<SevenBitAddress> + WriteRead<SevenBitAddress>> Lsm303dlhc<I> {

    /// Creates a new driver and configures the device.
    pub fn create(interface: I, cfg: Config) -> Result<Self, <I as Write>::Error> {
        let mut device = Lsm303dlhc { interface, scale: cfg.get_scales() };

        // Configure accelrometer module.
        device.wr(accel::ACCEL, &[accel::Register::Ctrl1 as u8, cfg.ctrl1])?;
        device.wr(accel::ACCEL, &[accel::Register::Ctrl2 as u8, cfg.ctrl2])?;
        device.wr(accel::ACCEL, &[accel::Register::Ctrl3 as u8, cfg.ctrl3])?;
        device.wr(accel::ACCEL, &[accel::Register::Ctrl4 as u8, cfg.ctrl4])?;
        device.wr(accel::ACCEL, &[accel::Register::Ctrl5 as u8, cfg.ctrl5])?;
        device.wr(accel::ACCEL, &[accel::Register::Ctrl6 as u8, cfg.ctrl6])?;

        device.wr(mag::MAG, &[mag::Register::Cra as u8, cfg.cra])?;
        device.wr(mag::MAG, &[mag::Register::Crb as u8, cfg.crb])?;
        device.wr(mag::MAG, &[mag::Register::Mr  as u8, cfg.mr ])?;

        Ok(device)
    }

    /// Sleeps the device.
    /// To wake it up, the user must select again the output data rate
    /// or reset the device.
    pub fn sleep(&mut self) -> Result<(), <I as Write>::Error> {
        // Configure accelrometer module.
        self.wr(accel::ACCEL, &[accel::Register::Ctrl1 as u8, 0b00001000])?;

        self.wr(mag::MAG, &[mag::Register::Mr  as u8, 0b00000011 ])?;

        Ok(())
    }
}


impl<I: Write<SevenBitAddress> + WriteRead<SevenBitAddress>> Accelerometer for Lsm303dlhc<I> {
    type Error = <I as WriteRead>::Error;
    type Output = i16;

    fn accelraw(&mut self) -> Result<[i16; 3], <I as WriteRead>::Error> {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 6] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        self.wrrd(accel::ACCEL, &[accel::Register::OutXL as u8 | (1 << 7)], &mut data).unwrap();

        // Get the raw i16 data.
        let rawx = (data[0] as i16) | ((data[1] as i16) << 8);
        let rawy = (data[2] as i16) | ((data[3] as i16) << 8);
        let rawz = (data[4] as i16) | ((data[5] as i16) << 8);

        Ok([rawx, rawy, rawz])

    }

    fn accel<F>(&mut self) -> Result<[F; 3], <I as WriteRead>::Error>
        where F: Clone + Copy +
            From<f32> + From<Self::Output> +
            Add<F, Output=F> + Sub<F, Output=F> +
            Mul<F, Output=F> + Div<F, Output=F>
    {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 6] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        self.wrrd(accel::ACCEL, &[accel::Register::OutXL as u8 | (1 << 7)], &mut data).unwrap();

        // Get the raw i16 data.
        let rawx = ((data[0] as i16) | ((data[1] as i16) << 8)) >> 4;
        let rawy = ((data[2] as i16) | ((data[3] as i16) << 8)) >> 4;
        let rawz = ((data[4] as i16) | ((data[5] as i16) << 8)) >> 4;

        // Get the resolution multiplier.
        let mul: F = match self.scale.0 {
            accel::Scale::G2  => F::from(0.001),
            accel::Scale::G4  => F::from(0.002),
            accel::Scale::G8  => F::from(0.004),
            accel::Scale::G16 => F::from(0.012),
        };

        let accx = (F::from(rawx) * mul) * F::from(9.80665);
        let accy = (F::from(rawy) * mul) * F::from(9.80665);
        let accz = (F::from(rawz) * mul) * F::from(9.80665);

        Ok([accx, accy, accz])

    }
}


impl<I: Write<SevenBitAddress> + WriteRead<SevenBitAddress>> Magnetometer for Lsm303dlhc<I> {
    type Error = <I as WriteRead>::Error;
    type Output = i16;

    fn magraw(&mut self) -> Result<[i16; 3], <I as WriteRead>::Error> {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 6] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        self.wrrd(mag::MAG, &[mag::Register::OutXH as u8 | (1 << 7)], &mut data)?;

        // Get the raw i16 data.
        let rawx = ((data[0] as i16) << 8) | (data[1] as i16);
        let rawy = ((data[4] as i16) << 8) | (data[5] as i16);
        let rawz = ((data[2] as i16) << 8) | (data[3] as i16);

        Ok([rawx, rawy, rawz])
    }

    fn mag<F>(&mut self) -> Result<[F; 3], <I as WriteRead>::Error>
        where F: Clone + Copy +
            From<f32> + From<Self::Output> +
            Add<F, Output=F> + Sub<F, Output=F> +
            Mul<F, Output=F> + Div<F, Output=F>
    {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 6] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        self.wrrd(mag::MAG, &[mag::Register::OutXH as u8 | (1 << 7)], &mut data)?;

        // Get the raw i16 data.
        let rawx = ((data[0] as i16) << 8) | (data[1] as i16);
        let rawy = ((data[4] as i16) << 8) | (data[5] as i16);
        let rawz = ((data[2] as i16) << 8) | (data[3] as i16);

        // Get the resolution multiplier.
        let (xy, z): (F, F) = match self.scale.1 {
            mag::Scale::Gauss1_3  => (F::from(1.0 / 1100.0), F::from(1.0 / 980.0) ),
            mag::Scale::Gauss1_9  => (F::from(1.0 /  855.0), F::from(1.0 / 760.0) ),
            mag::Scale::Gauss2_5  => (F::from(1.0 /  670.0), F::from(1.0 / 600.0) ),
            mag::Scale::Gauss4_0  => (F::from(1.0 /  450.0), F::from(1.0 / 400.0) ),
            mag::Scale::Gauss4_7  => (F::from(1.0 /  400.0), F::from(1.0 / 355.0) ),
            mag::Scale::Gauss5_6  => (F::from(1.0 /  330.0), F::from(1.0 / 295.0) ),
            mag::Scale::Gauss8_1  => (F::from(1.0 /  230.0), F::from(1.0 / 205.0) ),
        };

        let magx = (F::from(rawx) * xy) * F::from(100);
        let magy = (F::from(rawy) * xy) * F::from(100);
        let magz = (F::from(rawz) *  z) * F::from(100);

        Ok([magx, magy, magz])
    }

}


impl<I: Write<SevenBitAddress> + WriteRead<SevenBitAddress>> Thermometer for Lsm303dlhc<I> {
    type Error = <I as WriteRead>::Error;
    type Output = i16;

    fn tempraw(&mut self) -> Result<i16, <I as WriteRead>::Error> {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 2] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        self.wrrd(mag::MAG, &[mag::Register::TempOutH as u8], &mut data[0..1])?;
        self.wrrd(mag::MAG, &[mag::Register::TempOutL as u8], &mut data[1.. ])?;

        // Get the raw i16 data.
        let raw = (((data[0] as i16) << 8) | (data[1] as i16)) >> 4;

        Ok(raw)
    }

    fn temp<F>(&mut self) -> Result<F, <I as WriteRead>::Error>
        where F: Clone + Copy +
            From<f32> + From<Self::Output> +
            Add<F, Output=F> + Sub<F, Output=F> +
            Mul<F, Output=F> + Div<F, Output=F>
    {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 2] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        self.wrrd(mag::MAG, &[mag::Register::TempOutH as u8], &mut data[0..1])?;
        self.wrrd(mag::MAG, &[mag::Register::TempOutL as u8], &mut data[1.. ])?;

        // Get the raw i16 data.
        let raw = (((data[0] as i16) << 8) | (data[1] as i16)) >> 4;

        let temp = F::from(20.0) + (F::from(raw) / F::from(8.0));

        Ok(temp)
    }
}



impl<I: Write<SevenBitAddress>> Lsm303dlhc<I> {
    /// Internal write function.
    #[inline(always)]
    pub(crate) fn wr(&mut self, a: SevenBitAddress, bytes: &[u8]) -> Result<(), I::Error> {
        self.interface.write(a, bytes)
    }
}

impl<I: WriteRead<SevenBitAddress>> Lsm303dlhc<I> {
    /// Internal write function.
    #[inline(always)]
    pub(crate) fn wrrd(&mut self, a: SevenBitAddress, bytes: &[u8], buffer: &mut [u8]) -> Result<(), I::Error> {
        self.interface.write_read(a, bytes, buffer)
    }
}
