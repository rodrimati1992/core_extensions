/// For implementing the [`TransparentNewtype`] trait.
///
/// # Example
///
/// ```rust
/// use core_extensions::{TransparentNewtype, TransparentNewtypeExt, impl_transparent_newtype};
///
/// use std::cmp::{Ordering, Ord, PartialOrd};
///
/// fn main(){
///     let mut list = vec![3, 13, 21, 5, 8, 34];
///         
///     <[Reverse<u64>]>::from_inner_mut(&mut list).sort();
///    
///     assert_eq!(list, vec![34, 21, 13, 8, 5, 3]);
/// }
///
/// #[repr(transparent)]
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

        #[inline(always)]
        fn as_inner_raw(this: *const $S) -> *const <$S as $crate::TransparentNewtype>::Inner {
            this as _
        }

        #[inline(always)]
        fn as_inner_raw_mut(this: *mut $S) -> *mut <$S as $crate::TransparentNewtype>::Inner {
            this as _
        }
    )
}


/// For delegating the implementation of the [`TransparentNewtype`] trait to a field.
///
/// # Example
///
/// ```rust
/// use core_extensions::{TransparentNewtype, TransparentNewtypeExt};
///
/// use std::num::Wrapping;
///
/// #[derive(Debug, PartialEq, Eq)]
/// #[repr(transparent)]
/// struct Foo<T: ?Sized>(T);
///
/// unsafe impl<T: ?Sized + TransparentNewtype> TransparentNewtype for Foo<T> {
///     core_extensions::delegate_transparent_newtype_impl!{Self, T}
/// }
/// 
/// assert_eq!(
///     Foo::<Wrapping<u8>>::from_inner(3),
///     Foo(Wrapping(3)),
/// );
/// assert_eq!(
///     Foo::<Wrapping<bool>>::from_inner_ref(&true),
///     &Foo(Wrapping(true)),
/// );
/// assert_eq!(
///     Foo::<Wrapping<&str>>::from_inner_mut(&mut "hello"),
///     &mut Foo(Wrapping("hello")),
/// );
/// 
/// 
/// 
/// ```
///
/// [`TransparentNewtype`]: ./transparent_newtype/trait.TransparentNewtype.html#example 
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "transparent_newtype")))]
#[macro_export]
macro_rules! delegate_transparent_newtype_impl {
    ($S:ty, $Field:ty) => (
        type Inner = <$Field as $crate::TransparentNewtype>::Inner;

        #[inline(always)]
        fn from_inner_raw(from: *const <$S as $crate::TransparentNewtype>::Inner) -> *const $S {
            <$Field as $crate::TransparentNewtype>::from_inner_raw(from)
                as _
        }

        #[inline(always)]
        fn from_inner_raw_mut(from: *mut <$S as $crate::TransparentNewtype>::Inner) -> *mut $S {
            <$Field as $crate::TransparentNewtype>::from_inner_raw_mut(from)
                as _
        }

        #[inline(always)]
        fn as_inner_raw(this: *const $S) -> *const <$S as $crate::TransparentNewtype>::Inner {
            <$Field as $crate::TransparentNewtype>::as_inner_raw(this as _)
        }

        #[inline(always)]
        fn as_inner_raw_mut(this: *mut $S) -> *mut <$S as $crate::TransparentNewtype>::Inner {
            <$Field as $crate::TransparentNewtype>::as_inner_raw_mut(this as _)
        }
    )
}