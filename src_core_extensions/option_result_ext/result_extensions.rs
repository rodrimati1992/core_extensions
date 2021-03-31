#[cfg(any(feature = "alloc"))]
use std_::fmt;

#[cfg(any(feature = "alloc"))]
use alloc_::string::String;

use super::ResultLike;

use type_identity::TypeIdentity;

/// Extension trait for [`Result`].
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
pub trait ResultExt<T, E>: Sized + ResultLike + TypeIdentity<Type = Result<T, E>> {
    /// Maps `Err` variants to a `Debug` formated String.
    ///
    /// Equivalent to `.map_err(|e| format!("{:?}", e))`.
    ///
    /// # Example
    /// ```
    /// use core_extensions::ResultExt;
    ///
    /// let err_msg = "what \"is\" this";
    /// let err_: Result<(), &str> = Err(err_msg);
    ///
    /// assert_eq!(err_.format_debug_err(), Err(format!("{:?}", err_msg)));
    ///
    /// ```
    #[inline]
    #[cfg(any(feature = "alloc"))]
    fn format_debug_err(self) -> Result<T, String>
    where
        E: fmt::Debug,
    {
        self.into_type().map_err(|e| format!("{:?}", e))
    }
    /// Maps `Err` variants to an alternate `Debug` formated String.
    ///
    /// Equivalent to `.map_err(|e| format!("{:#?}", e))`.
    ///
    /// # Example
    /// ```
    /// use core_extensions::ResultExt;
    ///
    /// let err_msg = "what \"is\" this";
    /// let err_: Result<(), &str> = Err(err_msg);
    ///
    /// assert_eq!(err_.format_alt_debug_err(), Err(format!("{:#?}", err_msg)));
    ///
    /// ```
    #[inline]
    #[cfg(any(feature = "alloc"))]
    fn format_alt_debug_err(self) -> Result<T, String>
    where
        E: fmt::Debug,
    {
        self.into_type().map_err(|e| format!("{:#?}", e))
    }
}

impl<E, T> ResultExt<T, E> for Result<T, E> {}

impl<E, T> ResultLike for Result<T, E> {
    type Item = T;
    type Error = E;
    
    #[inline]
    fn is_item(&self) -> bool {
        self.is_ok()
    }
    #[inline]
    fn into_result_(self) -> Result<Self::Item, Self::Error> {
        self
    }

    #[inline]
    fn from_result_(from: Self) -> Self {
        from
    }

    #[inline]
    fn from_item(item: Self::Item) -> Self {
        Ok(item)
    }
    
    #[inline]
    fn from_error(err: Self::Error) -> Self {
        Err(err)
    }
}
