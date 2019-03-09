//! Extension traits for `bool`.

use type_identity::TypeIdentity;

/// Extension trait for `bool`.
pub trait BoolExt: TypeIdentity<Type = bool> + Sized {
    /// Returns Some(`some`()) if self==true.Otherwise returns None.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::bool_extensions::BoolExt;
    ///
    /// assert_eq!(true .if_true(|| 100 ),Some(100));
    /// assert_eq!(false.if_true(|| 100 ),None);
    ///
    /// ```
    ///
    #[inline]
    fn if_true<T, F>(self, some: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        if self.into_type_val() {
            Some(some())
        } else {
            None
        }
    }
    /// Returns Some(`some`()) if self==false.Otherwise returns None.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::bool_extensions::BoolExt;
    ///
    /// assert_eq!(false.if_false(|| 100 ),Some(100));
    /// assert_eq!(true .if_false(|| 100 ),None);
    ///
    /// ```
    ///
    #[inline]
    fn if_false<T, F>(self, some: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        if self.into_type_val() {
            None
        } else {
            Some(some())
        }
    }
}

impl BoolExt for bool {}
