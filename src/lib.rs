//! Collection of drivers for external components.

#![no_std]

#![allow(incomplete_features)]


#![feature(adt_const_params)]
#![feature(const_trait_impl)]



// Reexport all the main traits for the drivers.
pub use self::accel::Accelerometer;
pub use self::mag::Magnetometer;
pub use self::temp::Thermometer;



/// Module for accelerometers and combined peripherals.
mod accel;

/// Module for all magnetometers and combined peripherals.
mod mag;

/// Module for all thermometers and combined peripherals.
mod temp;




pub mod lsm303dlhc;