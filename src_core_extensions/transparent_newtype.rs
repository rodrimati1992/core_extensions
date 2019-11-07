//! Contains the TransparentNewtype trait to enable safer transmutation between types.
//!
//! # Safety for Users
//!
//! Given that T impls TransparentNewtype\<Inner=I>.
//!
//! It is always safe to convert from I to T directly.
//!
//! It is always safe to convert from pointers to I,eg.:*const I,*mut I,&I,&mut I, to
//! pointers to T (the same kind of pointer/reference).
//!
//! This also includes these smart pointers Box,Rc,Arc in the standard library.
//!
//! Transmuting a generic type C\<I> to C\<T> is not always safe ,because:
//!
//! - It may store a `\<I as SomeTrait>::AssocType` instead of storing `I` directly.
//!
//! - It has to be a type that doesn't change in layout when its type parameter changes.
//!
//! This has to be evaluated on an individual basis.
//!
//! # Example
//!
//! Casting a `[T]` to a `[Wrapper<T>]`,to use its Debug implementation.
//!
//! ```
//! use core_extensions::{SelfOps,TransparentNewtype};
//! use std::fmt;
//! use std::mem::ManuallyDrop;
//!
//! pub struct DebugFromDisplay<T:?Sized>(pub T);
//!
//! impl<T:?Sized+fmt::Display> fmt::Debug for DebugFromDisplay<T>{
//!     fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
//!         fmt::Display::fmt(&self.0,f)
//!     }
//! }
//! unsafe impl<T:?Sized> TransparentNewtype for DebugFromDisplay<T>{
//!     type Inner=T;
//! }
//! fn slice_as<T>(v:&[T])->&[DebugFromDisplay<T>]{
//!     unsafe{
//!         let ptr=v.as_ptr() as *const DebugFromDisplay<T>;
//!         std::slice::from_raw_parts( ptr, v.len() )
//!     }
//! }
//! let list=vec!["\"hello\"","\"world\""];
//! for elem in slice_as(&list) {
//!     assert_eq!(elem.0,format!("{:?}",elem));
//! }
//!
//! ```
//!
//! # Example
//!
//! A totally ordered 32 bit float.
//!
//! ```
//! use core_extensions::{SelfOps,TransparentNewtype};
//! # use std::cmp::{Eq,Ord,PartialOrd,PartialEq,Ordering};
//!
//! pub struct TotalF32(pub f32);
//!
//! //Eq,Ord,PartialEq,PartialOrd impls not shown
//! # impl Eq for TotalF32{}
//! #
//! # impl Ord for TotalF32{
//! #     fn cmp(&self, other: &Self) -> Ordering {
//! #         self.0.partial_cmp(&other.0).unwrap_or_else(||{
//! #             match (self.0.is_nan(),other.0.is_nan()) {
//! #                 (false,_    )=>Ordering::Less,
//! #                 (true ,false)=>Ordering::Greater,
//! #                 (true ,true )=>Ordering::Equal,
//! #             }
//! #         })
//! #     }
//! # }
//! # impl PartialOrd for TotalF32{
//! #     fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
//! #         Some(self.cmp(other))
//! #     }
//! # }
//! # impl PartialEq for TotalF32 {
//! #     fn eq(&self, other: &Self) -> bool {
//! #         self.cmp(other)==Ordering::Equal
//! #     }
//! # }
//!
//! unsafe impl TransparentNewtype for TotalF32{
//!     type Inner=f32;
//! }
//!
//! fn mut_slice_as(v:&mut [f32])->&mut [TotalF32]{
//!     unsafe{
//!         let ptr=v.as_mut_ptr() as *mut TotalF32;
//!         let len=v.len();
//!         std::slice::from_raw_parts_mut(ptr,len)
//!     }
//! }
//!
//! let mut list=vec![1.0,0.0,2.0];
//!
//! // This avoids the problem with using sort_by_key ,
//! // in which the borrow can't be returned from the closure.
//! mut_slice_as(&mut list).sort();
//!
//! assert_eq!(list,vec![0.0,1.0,2.0]);
//!
//! ```

use std_::mem;
use utils::transmute_ignore_size;

/// Trait for `#[repr(transparent)]` newtypes (stable since Rust 1.28) 
/// which are safe to transmute to/from their contents.
///
/// # Safety for users of TransparentNewtype types
///
/// Look at the [module-level documentation](index.html#safety-pitfalls)
///
pub unsafe trait TransparentNewtype {
    /// The wrapped type
    type Inner: ?Sized;

    /// Converts a T value to Self.
    #[inline(always)]
    fn convert_from(v: Self::Inner) -> Self
    where
        Self: Sized,
        Self::Inner: Sized,
    {
        check_same_size_alignment::<Self::Inner, Self>();
        unsafe { transmute_ignore_size::<Self::Inner, Self>(v) }
    }
    /// Converts a reference to T to a reference to Self.
    #[inline(always)]
    fn convert_ref_from(v: &Self::Inner) -> &Self {
        unsafe { transmute_ignore_size(v) }
    }
    /// Converts a mutable reference to T to a mutable reference to Self.
    #[inline(always)]
    fn convert_mut_from(v: &mut Self::Inner) -> &mut Self {
        unsafe { transmute_ignore_size(v) }
    }
    /// Converts self to a T value.
    #[inline(always)]
    fn convert_into(self) -> Self::Inner
    where
        Self: Sized,
        Self::Inner: Sized,
    {
        check_same_size_alignment::<Self::Inner, Self>();
        unsafe { transmute_ignore_size::<Self, Self::Inner>(self) }
    }
    /// Converts a reference to Self to a reference to T.
    #[inline(always)]
    fn convert_ref_to(&self) -> &Self::Inner {
        unsafe { transmute_ignore_size(self) }
    }
    /// Converts a mutable reference to Self to a mutable reference to T.
    #[inline(always)]
    fn convert_mut_to(&mut self) -> &mut Self::Inner {
        unsafe { transmute_ignore_size(self) }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[inline(always)]
fn check_same_size_alignment<T, U>() {
    assert_eq!(mem::size_of::<T>(), mem::size_of::<U>());
    assert_eq!(mem::align_of::<T>(), mem::align_of::<U>());
}
