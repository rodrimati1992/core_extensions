//! Extension traits for `bool`.

use type_identity::TypeIdentity;

/// Extension trait for `bool`.
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "bools")))]
pub trait BoolExt: TypeIdentity<Type = bool> + Sized {
    /// Returns `Some(some())` if `self` is `true`, otherwise returns `None`.
    ///
    /// This method is usable in all versions supported by this library, 
    /// and is equivalent to the [`bool::then`] method, which was stabilized in Rust 1.50.
    ///
    /// [`bool::then`]: https://doc.rust-lang.org/std/primitive.bool.html#method.then
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::BoolExt;
    ///
    /// assert_eq!(true .if_true(|| 100 ), Some(100));
    /// assert_eq!(false.if_true(|| 100 ), None);
    ///
    /// ```
    ///
    #[inline]
    fn if_true<T, F>(self, some: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        if self.into_type() {
            Some(some())
        } else {
            None
        }
    }
    /// Returns `Some(some())` if `self` is `false`, otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::BoolExt;
    ///
    /// assert_eq!(false.if_false(|| 100 ), Some(100));
    /// assert_eq!(true .if_false(|| 100 ), None);
    ///
    /// ```
    ///
    #[inline]
    fn if_false<T, F>(self, some: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        if self.into_type() {
            None
        } else {
            Some(some())
        }
    }
}

impl BoolExt for bool {}
