//! A version of TryFrom/TryInto in stable Rust before they are stabilized.
//! These traits are used for fallible conversions.
//!
//!

/// Attempts to convert from T to Self,returning Err(Self::Error) on failure.
pub trait TryFrom<T>: Sized {
    /// The error type returned when the conversion fails.
    type Error;

    /// Performs the conversion
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

/// Attempts to convert from Self to T,returning Err(Self::Error) on failure.
pub trait TryInto<T>: Sized {
    /// The error type returned when the conversion fails.
    type Error;

    /// Performs the conversion
    fn try_into(self) -> Result<T, Self::Error>;
}

impl<T, U> TryInto<U> for T
where
    U: TryFrom<T>,
{
    type Error = U::Error;

    fn try_into(self) -> Result<U, U::Error> {
        U::try_from(self)
    }
}
