/// For implementing the [`TransparentNewtype`] trait.
///
/// # Example
///
/// ```rust
/// use core_extensions::{TransparentNewtype, TransparentNewtypeExt, impl_transparent_newtype};
///
/// use std::cmp::{Ordering, Ord, PartialOrd};
///
///
/// let mut list = vec![3, 13, 21, 5, 8, 34];
/// 
/// <[Reverse<u64>]>::from_inner_mut(&mut list).sort();
///
/// assert_eq!(list, vec![34, 21, 13, 8, 5, 3]);
///
///
/// #[derive(PartialEq, Eq)]
/// struct Reverse<T: ?Sized>(T);
///
/// unsafe impl<T: ?Sized> TransparentNewtype for Reverse<T> {
///     type Inner = T;
///     
///     impl_transparent_newtype!{Self}
/// }
/// 
/// impl<T> PartialOrd for Reverse<T> 
/// where
///     T: ?Sized + PartialOrd
/// {
///     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
///         self.0.partial_cmp(&other.0)
///             .map(Ordering::reverse)
///     }
/// }
/// 
/// impl<T> Ord for Reverse<T> 
/// where
///     T: ?Sized + Ord
/// {
///     fn cmp(&self, other: &Self) -> Ordering {
///         self.0.cmp(&other.0).reverse()
///     }
/// }
///
/// ```
///
/// [`TransparentNewtype`]: ./transparent_newtype/trait.TransparentNewtype.html#example 
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "transparent_newtype")))]
#[macro_export]
macro_rules! impl_transparent_newtype {
    ($S:ty) => (
        #[inline(always)]
        fn from_inner_raw(from: *const <$S as $crate::TransparentNewtype>::Inner) -> *const $S {
            from as _
        }

        #[inline(always)]
        fn from_inner_raw_mut(from: *mut <$S as $crate::TransparentNewtype>::Inner) -> *mut $S {
            from as _
        }

        fn as_inner_raw(this: *const $S) -> *const <$S as $crate::TransparentNewtype>::Inner {
            this as _
        }

        fn as_inner_raw_mut(this: *mut $S) -> *mut <$S as $crate::TransparentNewtype>::Inner {
            this as _
        }
    )
}