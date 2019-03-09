//! Miscelaneous utility functions

use std_::mem;

/// Allows transmuting between types of different sizes.
///
/// # Safety
///
/// This function has the same safety concerns as [::std::mem::transmute_copy].
#[inline(always)]
pub unsafe fn transmute_ignore_size<T, U>(v: T) -> U {
    let ret: U = mem::transmute_copy::<T, U>(&v);
    mem::forget(v);
    ret
}

#[inline(always)]
/// Converts a reference to T to a slice of 1 T.
pub fn as_slice<T>(v: &T) -> &[T] {
    unsafe { ::std_::slice::from_raw_parts(v, 1) }
}

#[inline(always)]
/// Converts a mutable reference to T to a mutable slice of 1 T.
pub fn as_slice_mut<T>(v: &mut T) -> &mut [T] {
    unsafe { ::std_::slice::from_raw_parts_mut(v, 1) }
}

/// Use this function to mark to the compiler that this branch is impossible.
///
/// This function panics when debug assertions are enabled,
/// if debug assertions are disabled then reaching this is undefined behaviour.
///
/// For a version which doesn't panic in debug builds but instead always causes
/// undefined behaviour when reached use
/// [unreachable_unchecked](::std::hint::unreachable_unchecked)
/// which was stabilized in Rust 1.27.
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
        use void::Void;
        match *(1 as *const Void) {}
    }
}
