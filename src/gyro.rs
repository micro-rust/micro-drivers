//! Module for gyroscopes and combined peripherals.


use core::ops::*;

use micro::drivers::Data;


/// Common trait for all accelerometers.
pub trait Gyroscope {
    type Error;
    type Output: Data;

    /// Returns the raw accelerometer data.
    fn gyroraw(&mut self) -> Result<[Self::Output; 3], Self::Error>;

    /// Returns the accelerometer data after a normalization process.
    /// The associated `F` type must be an `f32` wrapper with platform support
    /// for `f32` operations.
    /// For example, if the ISA does not define float support, but the device
    /// has a propietary peripheral to operate on `f32`, this wrapper must
    /// implement this functionality.
    /// In case of hardware support for `f32` (e.g. amr-none-eabihf) `F=f32`.
    fn gyro<F>(&mut self) -> Result<[F; 3], Self::Error>
        where F: Clone + Copy +
            From<f32> + From<Self::Output> +
            Add<F, Output=F> + Sub<F, Output=F> +
            Mul<F, Output=F> + Div<F, Output=F>;
}
