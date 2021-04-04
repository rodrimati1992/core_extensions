//! Contains types and functions for impossible situations.

use std_::{cmp, fmt};

/// Type for impossible situations.
///
/// Use this as a type parameter to enums to make the variants that use it unconstructible.
///
/// # Interaction with unsafe code
///
/// It is only valid to convert to Void from other Void-like types,
/// it is undefined behavior to convert from any constructible type, even if zero-sized.
///
/// # Example, infinite loop which only returns on error.
///
#[cfg_attr(feature = "option_result", doc = " ```rust")]
#[cfg_attr(not(feature = "option_result"), doc = " ```ignore")]
/// use core_extensions::{ResultLikeExt, Void};
///
/// #[derive(Debug,PartialEq)]
/// enum Error<T>{
///     InvalidItem(T),
///     IteratorWasntInfinite,
/// }
///
/// fn reading_numbers<I>(i: I) -> Result<Void, Error<usize>>
/// where I: IntoIterator<Item = usize>
/// {
///     for elem in i{
///         if elem == 0 { return Err(Error::InvalidItem(elem)) }
///         println!("{}", elem);
///     }
///     Err(Error::IteratorWasntInfinite)
/// }
///
/// assert_eq!(reading_numbers(1..100).into_error(), Error::IteratorWasntInfinite);
/// assert_eq!(reading_numbers(0..).into_error(), Error::InvalidItem(0));
///
///
/// ```
#[derive(Debug, Copy, Clone, Hash)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "void")))]
pub enum Void {}

impl Void {
    /// Converts a `Void` to any type.
    ///
    /// Note that because `Void` is impossible to construct,
    /// this method is unreachable.
    pub fn to<T>(self) -> T {
        match self {}
    }
}

impl From<Void> for std_::convert::Infallible {
    #[inline(always)]
    fn from(this: Void) -> Self {
        match this {}
    }
}

/// There's also a `impl From<Void> for std_::convert::Infallible` impl
/// that's not appearing in the docs for some reason.
impl From<std_::convert::Infallible> for Void {
    #[inline(always)]
    fn from(this: std_::convert::Infallible) -> Self {
        match this {}
    }
}

#[cfg(std)]
impl std_::error::Error for Void {
    fn description(&self) -> &str {
        match *self {}
    }
}

impl fmt::Display for Void {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl Eq for Void {}

impl<T: ?Sized> PartialEq<T> for Void {
    fn eq(&self, _: &T) -> bool {
        self.to()
    }
}
impl Ord for Void {
    fn cmp(&self, _: &Self) -> cmp::Ordering {
        self.to()
    }
}
impl<T: ?Sized> PartialOrd<T> for Void {
    fn partial_cmp(&self, _: &T) -> Option<cmp::Ordering> {
        self.to()
    }
}

#[cfg(feature = "serde_")]
pub use self::serde_impl::DeserializeVoidError;
#[cfg(feature = "serde_")]
mod serde_impl {
    use super::*;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Represents a deserialization error,when trying to deserialize a struct or enum variant
    /// containing a `Void` field.
    ///
    /// Returned by serde::Deserialize::deserialize every time it's called.
    #[derive(Debug, Copy, Clone)]
    pub struct DeserializeVoidError;

    impl fmt::Display for DeserializeVoidError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str(
                "Cant deserialize a struct or \
                 enum variant containing a core_extensions::Void.",
            )
        }
    }

    /// This impl is only enabled if the "serde_" feature is enabled.
    ///
    /// This always Returns an `Err(D::Error::custom(DeserializeVoidError))`.
    impl<'de> Deserialize<'de> for Void {
        fn deserialize<D>(_: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Err(D::Error::custom(DeserializeVoidError))
        }
    }

    /// This impl is only enabled if the "serde_" feature is enabled.
    ///
    impl Serialize for Void {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.to()
        }
    }
}
