//! Traits for newtype wrappers.

#[cfg(feature = "alloc")]
use alloc::{
    boxed::Box,
    rc::Rc,
    sync::Arc,
    vec::Vec,
};

use std_::mem;

use crate::utils::transmute_ignore_size;

/// Trait for `#[repr(transparent)]` newtypes,
/// which are safe to transmute to/from their contents.
///
/// For additional conversion methods, you can use the [`TransparentNewtypeExt`] extension trait.
///
/// # Safety
///
/// Implementors must only implement this trait for `#[repr(transparent)]` wrappers,
/// with the same alignment as its only non-zero-sized field,
/// and the type of that field must be the value of the [`TransparentNewtype::Inner`]
/// associated type.
///
/// The recommended way to implement this trait's required methods is with:
/// ```
/// # struct Foo;
/// 
/// # unsafe impl core_extensions::TransparentNewtype for Foo{
/// #    type Inner = ();
///     core_extensions::impl_transparent_newtype!{Self}
/// # }
/// ```
///
/// # Example
///
/// A totally ordered 32 bit float.
///
/// ```
/// use core_extensions::{TransparentNewtype, TransparentNewtypeExt};
/// # use std::cmp::{Eq,Ord,PartialOrd,PartialEq,Ordering};
///
/// pub struct TotalF32(pub f32);
///
/// // Eq, Ord, PartialEq, PartialOrd impls not shown
/// # impl Eq for TotalF32{}
/// #
/// # impl Ord for TotalF32{
/// #     fn cmp(&self, other: &Self) -> Ordering {
/// #         self.0.partial_cmp(&other.0).unwrap_or_else(||{
/// #             match (self.0.is_nan(),other.0.is_nan()) {
/// #                 (false,_    )=>Ordering::Less,
/// #                 (true ,false)=>Ordering::Greater,
/// #                 (true ,true )=>Ordering::Equal,
/// #             }
/// #         })
/// #     }
/// # }
/// # impl PartialOrd for TotalF32{
/// #     fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
/// #         Some(self.cmp(other))
/// #     }
/// # }
/// # impl PartialEq for TotalF32 {
/// #     fn eq(&self, other: &Self) -> bool {
/// #         self.cmp(other)==Ordering::Equal
/// #     }
/// # }
///
/// unsafe impl TransparentNewtype for TotalF32{
///     type Inner = f32;
///     core_extensions::impl_transparent_newtype!{Self}
/// }
///
/// let mut list = vec![1.0, 0.0, 2.0];
///
/// // This avoids the problem with using sort_by_key ,
/// // in which the borrow can't be returned from the closure.
/// <[TotalF32]>::from_inner_mut(&mut list).sort();
///
/// assert_eq!(list, vec![0.0, 1.0, 2.0]);
///
/// ```
///
/// [`TransparentNewtype::Inner`]: #associatedtype.Inner
/// [`TransparentNewtypeExt`]: ./trait.TransparentNewtypeExt.html
///
pub unsafe trait TransparentNewtype {
    /// The wrapped type
    type Inner: ?Sized;

    /// Converts `*const Self::Inner` to `*const Self`.
    fn from_inner_raw(from: *const Self::Inner) -> *const Self;
    
    /// Converts `*mut Self::Inner` to `*mut Self`.
    fn from_inner_raw_mut(v: *mut Self::Inner) -> *mut Self;
    
    /// Converts `*const Self` to `*const Self::Inner`.
    fn as_inner_raw(from: *const Self) -> *const Self::Inner;
    
    /// Converts `*mut Self` to `*mut Self::Inner`.
    fn as_inner_raw_mut(this: *mut Self) -> *mut Self::Inner;
}

/// Extension trait for [`TransparentNewtype`]s
/// 
/// [`TransparentNewtype`]: ./trait.TransparentNewtype.html
/// 
/// 
pub trait TransparentNewtypeExt: TransparentNewtype {
    /// Converts `Self::Inner` to `Self`.
    #[inline(always)]
    fn from_inner(v: Self::Inner) -> Self
    where
        Self: Sized,
        Self::Inner: Sized,
    {
        check_same_size_alignment::<Self::Inner, Self>();
        unsafe { transmute_ignore_size::<Self::Inner, Self>(v) }
    }

    /// Converts `&Self::Inner` to a `&Self`.
    #[inline(always)]
    fn from_inner_ref(v: &Self::Inner) -> &Self {
        unsafe { &*Self::from_inner_raw(v) }
    }
    
    /// Converts `&mut Self::Inner` to a `&mut Self`.
    #[inline(always)]
    fn from_inner_mut(v: &mut Self::Inner) -> &mut Self {
        unsafe { &mut *Self::from_inner_raw_mut(v) }
    }

    /// Converts `Box<Self::Inner>` to a `Box<Self>`.
    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn from_inner_box(v: Box<Self::Inner>) -> Box<Self> {
        unsafe { Box::from_raw(Self::from_inner_raw_mut(Box::into_raw(v))) }
    }

    /// Converts `Arc<Self::Inner>` to a `Arc<Self>`.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
    #[inline(always)]
    fn from_inner_arc(v: Arc<Self::Inner>) -> Arc<Self> {
        unsafe { Arc::from_raw(Self::from_inner_raw(Arc::into_raw(v))) }
    }
    
    /// Converts `Rc<Self::Inner>` to a `Rc<Self>`.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
    #[inline(always)]
    fn from_inner_rc(v: Rc<Self::Inner>) -> Rc<Self> {
        unsafe { Rc::from_raw(Self::from_inner_raw(Rc::into_raw(v))) }
    }

    /// Converts `self` to a `Self::Inner`.
    #[inline(always)]
    fn into_inner(self) -> Self::Inner
    where
        Self: Sized,
        Self::Inner: Sized,
    {
        check_same_size_alignment::<Self::Inner, Self>();
        unsafe { transmute_ignore_size::<Self, Self::Inner>(self) }
    }

    /// Converts `self` to a `&Self::Inner`.
    #[inline(always)]
    fn as_inner(&self) -> &Self::Inner {
        unsafe { &*Self::as_inner_raw(self) }
    }

    /// Converts `self` to a `&mut Self::Inner`.
    #[inline(always)]
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        unsafe { &mut *Self::as_inner_raw_mut(self) }
    }

    /// Converts `self` to a `Box<Self::Inner>`.
    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn into_inner_box(self: Box<Self>) -> Box<Self::Inner> {
        unsafe { Box::from_raw(Self::as_inner_raw_mut(Box::into_raw(self))) }
    }

    rc_shared_docs!{
        /// Converts `self` to a `Arc<Self::Inner>`.
        /// 
        /// # Self parameter
        /// 
        /// Enabling the "rust_1_46" feature changes this method to have a `self` parameter,
        /// allowing calling it with `.into_inner_arc()`
        /// 
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
        #[inline(always)]
        =>
        (
            fn into_inner_arc(this: Arc<Self>) -> Arc<Self::Inner> {
                unsafe { Arc::from_raw(Self::as_inner_raw(Arc::into_raw(this))) }
            }
        )
        (
            fn into_inner_arc(self: Arc<Self>) -> Arc<Self::Inner> {
                unsafe { Arc::from_raw(Self::as_inner_raw(Arc::into_raw(self))) }
            }
        )
    }

    rc_shared_docs!{
        /// Converts `self` to a `Rc<Self::Inner>`.
        /// 
        /// # Self parameter
        /// 
        /// Enabling the "rust_1_46" feature changes this method to have a `self` parameter,
        /// allowing calling it with `.into_inner_rc()`
        /// 
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
        #[inline(always)]
        =>
        (
            fn into_inner_rc(this: Rc<Self>) -> Rc<Self::Inner> {
                unsafe { Rc::from_raw(Self::as_inner_raw(Rc::into_raw(this))) }
            }
        )
        (
            fn into_inner_rc(self: Rc<Self>) -> Rc<Self::Inner> {
                unsafe { Rc::from_raw(Self::as_inner_raw(Rc::into_raw(self))) }
            }
        )
    }
}

impl<T> TransparentNewtypeExt for T
where
    T: TransparentNewtype + ?Sized
{}

///////////////////////////////////////////////////////////////////////////////

unsafe impl<T> TransparentNewtype for [T]
where
    T: TransparentNewtype,
    T::Inner: Sized,
{
    type Inner = [T::Inner];
    crate::impl_transparent_newtype!{Self}
}

///////////////////////////////////////////////////////////////////////////////

unsafe impl<T> TransparentNewtype for core::num::Wrapping<T> {
    type Inner = T;

    crate::impl_transparent_newtype!{Self}
}

///////////////////////////////////////////////////////////////////////////////

unsafe impl<T> TransparentNewtype for core::mem::ManuallyDrop<T> {
    type Inner = T;

    crate::impl_transparent_newtype!{Self}
}

///////////////////////////////////////////////////////////////////////////////

/// Converts a `Vec` of `T` into a `Vec` of the type that `T` wraps.
#[cfg(feature = "alloc")]
pub fn into_inner_vec<T>(this: Vec<T>) -> Vec<T::Inner>
where
    T: TransparentNewtype,
    T::Inner: Sized,
{
    unsafe{ crate::utils::transmute_vec(this) }
}

/// Converts a `Vec` of some type into a `Vec` of a wrapper around that type.
#[cfg(feature = "alloc")]
pub fn from_inner_vec<T>(this: Vec<T::Inner>) -> Vec<T>
where
    T: TransparentNewtype,
    T::Inner: Sized,
{
    unsafe{ crate::utils::transmute_vec(this) }
}

///////////////////////////////////////////////////////////////////////////////

#[inline(always)]
fn check_same_size_alignment<T, U>() {
    assert_eq!(mem::size_of::<T>(), mem::size_of::<U>());
    assert_eq!(mem::align_of::<T>(), mem::align_of::<U>());
}
