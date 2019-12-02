#[cfg(any(feature = "alloc", test))]
use std_::fmt;

#[cfg(any(feature = "alloc", test))]
use alloc_::string::String;

use super::ResultLike;
use type_identity::TypeIdentity;
#[allow(unused_imports)]
use SelfOps;

/// Extension trait for [Result].
pub trait ResultExt<T, E>: Sized + ResultLike + TypeIdentity<Type = Result<T, E>> {
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    /// Maps Err(e) to a Debug `{:?}` formated String.
    ///
    /// # Example
    /// ```
    /// use core_extensions::ResultExt;
    ///
    /// let err_="what \"is\" this";
    ///
    /// assert_eq!(
    ///     Err::<(),&str>(err_).format_debug_err(),
    ///     Err(format!("{:?}",err_))
    /// );
    ///
    /// ```
    fn format_debug_err(self) -> Result<T, String>
    where
        E: fmt::Debug,
    {
        self.into_type_val().map_err(|e| format!("{:?}", e))
    }
    #[cfg(any(feature = "alloc", test))]
    /// Maps Err(e) to an alternate Debug `{:#?}` formated String.
    ///
    /// # Example
    /// ```
    /// use core_extensions::ResultExt;
    ///
    /// let err_="what \"is\" this";
    ///
    /// assert_eq!(
    ///     Err::<(),&str>(err_).format_alt_debug_err(),
    ///     Err(format!("{:#?}",err_))
    /// );
    ///
    /// ```
    fn format_alt_debug_err(self) -> Result<T, String>
    where
        E: fmt::Debug,
    {
        self.into_type_val().map_err(|e| format!("{:#?}", e))
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
    fn to_result_(self) -> Result<Self::Item, Self::Error> {
        self
    }
}
