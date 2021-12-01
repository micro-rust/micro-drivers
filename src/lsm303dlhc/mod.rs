//! LSM303DLHC accelerometer, magnetometer and thermometer device.


pub mod accel;
pub mod mag;

mod config;
mod error;


pub use self::config::Config;
pub use self::error::Error;

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

    /// Accelerometer mode and range.
    accel: (accel::Mode, accel::Range),

    /// Magnetometer range.
    mag: mag::Range,
}

impl<I: Write<SevenBitAddress> + WriteRead<SevenBitAddress>> Lsm303dlhc<I> {

    /// Creates a new driver and configures the device.
    pub fn create(interface: I, cfg: Config) -> Result<Self, <I as Write>::Error> {
        // Get the parameters of the accelerometer and magnetometer.
        let (accel, mag) = cfg.params();

        // Create the device.
        let mut device = Lsm303dlhc { interface, accel, mag };

        // Configure accelrometer module.
        device.wr(accel::ACCEL, &[accel::Register::Ctrl1 as u8, cfg.ctrl1])?;
        device.wr(accel::ACCEL, &[accel::Register::Ctrl2 as u8, cfg.ctrl2])?;
        device.wr(accel::ACCEL, &[accel::Register::Ctrl3 as u8, cfg.ctrl3])?;
        device.wr(accel::ACCEL, &[accel::Register::Ctrl4 as u8, cfg.ctrl4])?;
        device.wr(accel::ACCEL, &[accel::Register::Ctrl5 as u8, cfg.ctrl5])?;
        device.wr(accel::ACCEL, &[accel::Register::Ctrl6 as u8, cfg.ctrl6])?;


        // Reset the magnetometer gain.
        device.wr(mag::MAG, &[mag::Register::Crb as u8, 0x00])?;
        // Configure magnetometer gain.
        device.wr(mag::MAG, &[mag::Register::Crb as u8, cfg.crb])?;
        // Set output data rate and temperature.
        device.wr(mag::MAG, &[mag::Register::Cra as u8, cfg.cra])?;
        // Enable continous mode, single conversion or sleep mode.
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
    type Error = Error<<I as WriteRead>::Error>;
    type Output = i16;

    fn accelraw(&mut self) -> Result<[i16; 3], Error<<I as WriteRead>::Error>> {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 6] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        self.wrrd(accel::ACCEL, &[accel::Register::OutXL as u8 | (1 << 7)], &mut data).unwrap();

        // Get the raw i16 data.
        let rawx: i16 = unsafe { core::mem::transmute( (data[0] as u16) | ((data[1] as u16) << 8) ) };
        let rawy: i16 = unsafe { core::mem::transmute( (data[2] as u16) | ((data[3] as u16) << 8) ) };
        let rawz: i16 = unsafe { core::mem::transmute( (data[4] as u16) | ((data[5] as u16) << 8) ) };

        Ok([rawx, rawy, rawz])

    }

    fn accel<F>(&mut self) -> Result<[F; 3], Error<<I as WriteRead>::Error>>
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
        let rawx: i16 = unsafe { core::mem::transmute( (data[0] as u16) | ((data[1] as u16) << 8) ) };
        let rawy: i16 = unsafe { core::mem::transmute( (data[2] as u16) | ((data[3] as u16) << 8) ) };
        let rawz: i16 = unsafe { core::mem::transmute( (data[4] as u16) | ((data[5] as u16) << 8) ) };

        // Get the shift and LSB data.
        let (shift, lsb) = self.accel.0.params(self.accel.1);

        // Calculate the acceleration.
        let accx = F::from(rawx >> shift) * F::from(lsb) * F::from( 9.80665 );
        let accy = F::from(rawy >> shift) * F::from(lsb) * F::from( 9.80665 );
        let accz = F::from(rawz >> shift) * F::from(lsb) * F::from( 9.80665 );

        Ok([accx, accy, accz])
    }
}


impl<I: Write<SevenBitAddress> + WriteRead<SevenBitAddress>> Magnetometer for Lsm303dlhc<I> {
    type Error = Error<<I as WriteRead>::Error>;
    type Output = i16;

    fn magraw(&mut self) -> Result<[i16; 3], Error<<I as WriteRead>::Error>> {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 6] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        match self.wrrd(mag::MAG, &[mag::Register::OutXH as u8 | (1 << 7)], &mut data) {
            Err(e) => return Err( Error::BusError(e) ),
            _ => (),
        };

        // Get the raw i16 data.
        let rawx: i16 = unsafe { core::mem::transmute( ((data[0] as u16) << 8) | (data[1] as u16) ) };
        let rawy: i16 = unsafe { core::mem::transmute( ((data[4] as u16) << 8) | (data[5] as u16) ) };
        let rawz: i16 = unsafe { core::mem::transmute( ((data[2] as u16) << 8) | (data[3] as u16) ) };

        Ok([rawx, rawy, rawz])
    }

    fn mag<F>(&mut self) -> Result<[F; 3], Error<<I as WriteRead>::Error>>
        where F: Clone + Copy +
            From<f32> + From<Self::Output> +
            Add<F, Output=F> + Sub<F, Output=F> +
            Mul<F, Output=F> + Div<F, Output=F>
    {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 6] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        match self.wrrd(mag::MAG, &[mag::Register::OutXH as u8 | (1 << 7)], &mut data) {
            Err(e) => return Err( Error::BusError(e) ),
            _ => (),
        };

        // Get the raw i16 data.
        let rawx: i16 = unsafe { core::mem::transmute( ((data[0] as u16) << 8) | (data[1] as u16) ) };
        let rawy: i16 = unsafe { core::mem::transmute( ((data[4] as u16) << 8) | (data[5] as u16) ) };
        let rawz: i16 = unsafe { core::mem::transmute( ((data[2] as u16) << 8) | (data[3] as u16) ) };

        // Check for a range overflow.
        if (rawx >= 2048) || (rawx < -2048) { return Err( Error::RangeOverflowX ) }
        if (rawy >= 2048) || (rawy < -2048) { return Err( Error::RangeOverflowY ) }
        //if (rawz >= 2048) || (rawz < -2048) { return Err( Error::RangeOverflowZ ) }

        // Get the resolution multiplier.
        let (xy, z) = self.mag.params();

        let magx = (F::from(rawx) / F::from(xy)) * F::from(100);
        let magy = (F::from(rawy) / F::from(xy)) * F::from(100);
        let magz = (F::from(rawz) / F::from( z)) * F::from(100);

        Ok([magx, magy, magz])
    }
}


impl<I: Write<SevenBitAddress> + WriteRead<SevenBitAddress>> Thermometer for Lsm303dlhc<I> {
    type Error = Error<<I as WriteRead>::Error>;
    type Output = i16;

    fn tempraw(&mut self) -> Result<i16, Error<<I as WriteRead>::Error>> {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 2] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        match self.wrrd(mag::MAG, &[mag::Register::TempOutH as u8], &mut data[0..1]) {
            Err(e) => return Err( Error::BusError(e) ),
            _ => (),
        };

        match self.wrrd(mag::MAG, &[mag::Register::TempOutL as u8], &mut data[1..]) {
            Err(e) => return Err( Error::BusError(e) ),
            _ => (),
        };

        // Get the raw i16 data.
        let raw = (((data[0] as i16) << 8) | (data[1] as i16)) >> 4;

        Ok(raw)
    }

    fn temp<F>(&mut self) -> Result<F, Error<<I as WriteRead>::Error>>
        where F: Clone + Copy +
            From<f32> + From<Self::Output> +
            Add<F, Output=F> + Sub<F, Output=F> +
            Mul<F, Output=F> + Div<F, Output=F>
    {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 2] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        match self.wrrd(mag::MAG, &[mag::Register::TempOutH as u8], &mut data[0..1]) {
            Err(e) => return Err( Error::BusError(e) ),
            _ => (),
        };

        match self.wrrd(mag::MAG, &[mag::Register::TempOutL as u8], &mut data[1..]) {
            Err(e) => return Err( Error::BusError(e) ),
            _ => (),
        };

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
