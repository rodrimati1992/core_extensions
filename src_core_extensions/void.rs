//! Contains types and functions for impossible situations.

use std_::{cmp, fmt};

/// Type for impossible situations.
///
/// Use this as a type parameter to enums to make the variants that use it unconstructible.
///
/// This type is used in [ResultLike](../option_result_ext/trait.ResultLike.html)
/// to unwrap values that can only be either the ok or error variants of the type.
///
/// # Interaction with unsafe code
///
/// It is only valid to convert to Void from other Void-like types,
/// it is undefined behavior to convert from any constructible type,even if zero-sized.
///
/// # Example,infallible FromStr implementation.
///
/// ```
/// use std::str::FromStr;
/// use core_extensions::{SelfOps,ResultLike,Void};
///
/// #[derive(Debug,PartialEq)]
/// pub struct Double(pub String);
/// impl FromStr for Double{
///     type Err=Void;
///     fn from_str(s:&str)->Result<Self,Void>{
///         s.repeat(2)
///             .piped(Double)
///             .piped(Ok)
///     }
/// }
///
/// assert_eq!(
///     "12345".parse::<Double>().unwrap_safe(),
///     Double("12345".repeat(2))
/// );
///
/// ```
///
/// # Example,infinite loop which only returns on error.
///
/// ```
/// use core_extensions::{ResultLike,Void};
///
/// #[derive(Debug,PartialEq)]
/// enum Error<T>{
///     InvalidItem(T),
///     IteratorWasntInfinite,
/// }
/// fn reading_numbers<I>(i:I)->Result<Void,Error<usize>>
/// where I:IntoIterator<Item=usize>
/// {
///     for elem in i{
///         if elem==0 { return Err(Error::InvalidItem(elem)) }
///         println!("{}",elem);
///     }
///     Err(Error::IteratorWasntInfinite)
/// }
///
/// assert_eq!(reading_numbers(1..100).unwrap_err_safe() , Error::IteratorWasntInfinite);
/// assert_eq!(reading_numbers(0..).unwrap_err_safe() , Error::InvalidItem(0));
///
///
/// ```
#[derive(Debug, Copy, Clone, Hash)]
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

// Conflicts with the built-in `impl Into<T> for T{...}`
// impl<T> Into<T> for Void{
//     fn into(_:Self)->T{
//         self.to()
//     }
// }

#[cfg(std)]
impl ::std::error::Error for Void {
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
