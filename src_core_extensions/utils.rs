//! Miscelaneous utility functions

#[cfg(feature = "alloc")]
use alloc_::vec::Vec;

use std_::mem::{self, ManuallyDrop};

/// Allows transmuting between types of different sizes.
///
/// # Safety
///
/// This function has the same safety concerns as [::std::mem::transmute_copy].
#[inline(always)]
pub unsafe fn transmute_ignore_size<T, U>(v: T) -> U {
    let v=ManuallyDrop::new(v);
    mem::transmute_copy::<T, U>(&v)
}

/// Transmutes a `Vec<T>` into a `Vec<U>`
///
/// # Safety
///
/// This function has the same safety requirements as [`std::mem::transmute`] 
/// regarding transmuting from `T` to `U`.
///
/// [`std::mem::transmute`]: https://doc.rust-lang.org/std/mem/fn.transmute.html
pub unsafe fn transmute_vec<T, U>(vector: Vec<T>) -> Vec<U> {
    let len = vector.len();
    let capacity = vector.capacity();
    let mut vector = ManuallyDrop::new(vector);
    Vec::from_raw_parts(vector.as_mut_ptr() as *mut U, len, capacity)
}



/// Use this function to mark to the compiler that this branch is impossible.
///
/// This function panics when debug assertions are enabled,
/// if debug assertions are disabled then reaching this is undefined behaviour.
///
/// For a version which doesn't panic in debug builds but instead always causes
/// undefined behaviour when reached use
/// [unreachable_unchecked](::std::hint::unreachable_unchecked).
///
/// # Safety
///
/// It is undefined behaviour for this function to be reached at runtime at all.
///
/// The compiler is free to delete any code that reaches and depends on this function
/// on the assumption that this branch can't be reached.
///
/// # Example
/// ```
/// use core_extensions::BoolExt;
/// use core_extensions::utils::impossible;
///
/// mod only_even{
///     use super::*;
///     #[derive(Debug,Copy,Clone)]
///     pub struct NonZero(usize);
///
///     impl NonZero{
///         pub fn new(value:usize)->Option<NonZero> {
///             (value!=0).if_true(|| NonZero( value ) )
///         }
///         pub fn value(&self)->usize{
///             self.0
///         }
///     }
/// }
/// use self::only_even::NonZero;
///
/// # fn main(){
///
/// fn div(numerator:usize,denom:Option<NonZero>)->usize{
///     let denom=match denom {
///         Some(v)if v.value()==0 => unsafe{
///             // unreachable: NonZero::value() can never be 0,
///             impossible()
///         },
///         Some(v)=>v.value(),
///         None=>1,
///     };
///     numerator / denom
/// }
///
/// assert_eq!(div(60,NonZero::new(0)) , 60);
/// assert_eq!(div(60,NonZero::new(1)) , 60);
/// assert_eq!(div(60,NonZero::new(2)) , 30);
/// assert_eq!(div(60,NonZero::new(3)) , 20);
/// assert_eq!(div(60,NonZero::new(4)) , 15);
/// assert_eq!(div(60,NonZero::new(5)) , 12);
/// assert_eq!(div(60,NonZero::new(6)) , 10);
///
///
/// # }
///
///
///
/// ```
///
///
#[inline(always)]
pub unsafe fn impossible() -> ! {
    #[cfg(debug_assertions)]
    {
        panic!("reached core_extensions::impossible() ")
    }
    #[cfg(not(debug_assertions))]
    {
        std::hint::unreachable_unchecked()
    }
}
