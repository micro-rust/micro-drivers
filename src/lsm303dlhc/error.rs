//! Errors that can occur with the LSM3030DLHC device.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<E> {
	/// The data output was saturated. Consider increasing the range of measurements.
	RangeOverflowX,
	RangeOverflowY,
	RangeOverflowZ,

	/// An error ocurred in the underlying bus subsystem.
	BusError(E)
}