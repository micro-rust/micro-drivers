//! L3GD20 gyroscope and thermometer.


pub mod gyro;

mod config;
mod error;


pub use self::config::Config;
pub use self::error::Error;

use crate::{ Gyroscope, Thermometer };

use core::{ mem::MaybeUninit, ops::* };

use embedded_hal::i2c::SevenBitAddress;
use embedded_hal::i2c::blocking::{
    Write, WriteRead
};


/// L3GD20 driver.
pub struct L3gd20<I> {
    /// I2C interface.
    interface: I,

    /// Address of the device.
    /// L3GD20 has a selectable address bit.
    addr: u8,

    /// Gyroscope range.
    gyro: gyro::Range,
}

impl<I: Write<SevenBitAddress> + WriteRead<SevenBitAddress>> L3gd20<I> {

    /// Creates a new driver and configures the device.
    pub fn create(interface: I, addr: u8, cfg: Config) -> Result<Self, <I as Write>::Error> {
        // Get the parameters of the accelerometer and magnetometer.
        let gyro = cfg.params();

        // Create the device.
        let mut device = L3gd20 { interface, addr, gyro };

        // Configure gyroscope module.
        device.wr(&[gyro::Register::Ctrl1 as u8, cfg.ctrl1])?;
        device.wr(&[gyro::Register::Ctrl2 as u8, cfg.ctrl2])?;
        device.wr(&[gyro::Register::Ctrl3 as u8, cfg.ctrl3])?;
        device.wr(&[gyro::Register::Ctrl4 as u8, cfg.ctrl4])?;
        device.wr(&[gyro::Register::Ctrl5 as u8, cfg.ctrl5])?;

        Ok(device)
    }

    /// Sleeps the device.
    /// To wake it up, the user must select again the output data rate
    /// or reset the device.
    pub fn sleep(&mut self) -> Result<(), <I as Write>::Error> {
        // Configure accelrometer module.
        self.wr(&[gyro::Register::Ctrl1 as u8, 0b00001000])?;

        Ok(())
    }
}


impl<I: Write<SevenBitAddress> + WriteRead<SevenBitAddress>> Gyroscope for L3gd20<I> {
    type Error = Error<<I as WriteRead>::Error>;
    type Output = i16;

    fn gyroraw(&mut self) -> Result<[i16; 3], Error<<I as WriteRead>::Error>> {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 6] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        match self.wrrd(&[gyro::Register::OutXL as u8 | (1 << 7)], &mut data) {
            Err(e) => return Err( Error::BusError(e) ),
            _ => (),
        };

        // Get the raw i16 data.
        let rawx: i16 = unsafe { core::mem::transmute( (data[0] as u16) | ((data[1] as u16) << 8) ) };
        let rawy: i16 = unsafe { core::mem::transmute( (data[2] as u16) | ((data[3] as u16) << 8) ) };
        let rawz: i16 = unsafe { core::mem::transmute( (data[4] as u16) | ((data[5] as u16) << 8) ) };

        Ok([rawx, rawy, rawz])
    }

    fn gyro<F>(&mut self) -> Result<[F; 3], Error<<I as WriteRead>::Error>>
        where F: Clone + Copy +
            From<f32> + From<Self::Output> +
            Add<F, Output=F> + Sub<F, Output=F> +
            Mul<F, Output=F> + Div<F, Output=F>
    {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 6] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        match self.wrrd(&[gyro::Register::OutXL as u8 | (1 << 7)], &mut data) {
            Err(e) => return Err( Error::BusError(e) ),
            _ => (),
        };

        // Get the raw i16 data.
        let rawx: i16 = unsafe { core::mem::transmute( (data[0] as u16) | ((data[1] as u16) << 8) ) };
        let rawy: i16 = unsafe { core::mem::transmute( (data[2] as u16) | ((data[3] as u16) << 8) ) };
        let rawz: i16 = unsafe { core::mem::transmute( (data[4] as u16) | ((data[5] as u16) << 8) ) };

        // Check for saturation.
        if (rawx >= 32760) || (rawx <= -32760) { return Err( Error::RangeOverflowX ) }
        if (rawy >= 32760) || (rawy <= -32760) { return Err( Error::RangeOverflowY ) }
        //if (rawz >= 32760) || (rawz <= -32760) { return Err( Error::RangeOverflowZ ) }

        // Get the resolution multiplier.
        let mul = self.gyro.params();

        let gyrox = F::from(rawx) * F::from(mul) * F::from(0.017453293);
        let gyroy = F::from(rawy) * F::from(mul) * F::from(0.017453293);
        let gyroz = F::from(rawz) * F::from(mul) * F::from(0.017453293);

        Ok([gyrox, gyroy, gyroz])
    }
}



impl<I: Write<SevenBitAddress> + WriteRead<SevenBitAddress>> Thermometer for L3gd20<I> {
    type Error = Error<<I as WriteRead>::Error>;
    type Output = i8;

    fn tempraw(&mut self) -> Result<i8, Error<<I as WriteRead>::Error>> {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        match self.wrrd(&[gyro::Register::TempOut as u8], &mut data) {
            Err(e) => return Err( Error::BusError(e) ),
            _ => (),
        };

        Ok( unsafe { core::mem::transmute(data) } )
    }

    fn temp<F>(&mut self) -> Result<F, Error<<I as WriteRead>::Error>>
        where F: Clone + Copy +
            From<f32> + From<Self::Output> +
            Add<F, Output=F> + Sub<F, Output=F> +
            Mul<F, Output=F> + Div<F, Output=F>
    {
        // Create the input buffer.
        // This is safe, as this buffer will be written to before we read.
        let mut data: [u8; 1] = unsafe { MaybeUninit::uninit().assume_init() };

        // Read in the output data.
        match self.wrrd(&[gyro::Register::TempOut as u8], &mut data) {
            Err(e) => return Err( Error::BusError(e) ),
            _ => (),
        };

        let raw: i8 = unsafe { core::mem::transmute(data) };

        let temp = F::from(raw);

        Ok(temp)
    }
}



impl<I: Write<SevenBitAddress>> L3gd20<I> {
    /// Internal write function.
    #[inline(always)]
    pub(crate) fn wr(&mut self, bytes: &[u8]) -> Result<(), I::Error> {
        self.interface.write(self.addr, bytes)
    }
}

impl<I: WriteRead<SevenBitAddress>> L3gd20<I> {
    /// Internal write function.
    #[inline(always)]
    pub(crate) fn wrrd(&mut self, bytes: &[u8], buffer: &mut [u8]) -> Result<(), I::Error> {
        self.interface.write_read(self.addr, bytes, buffer)
    }
}
