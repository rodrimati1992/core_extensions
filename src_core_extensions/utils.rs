//! Miscelaneous utility functions

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use std_::mem::{self, ManuallyDrop};

/// Allows transmuting between types of different sizes.
///
/// Necessary for transmuting in generic functions, since (as of Rust 1.51.0) 
/// transmute doesn't work well with generic types.
///
/// # Safety
///
/// This function has the same safety requirements as [`std::mem::transmute_copy`].
///
/// # Example
///
/// ```rust
/// use core_extensions::utils::transmute_ignore_size;
/// 
/// use std::mem::MaybeUninit;
/// 
/// unsafe fn transmute_into_init<T>(array: [MaybeUninit<T>; 3]) -> [T; 3] {
///     transmute_ignore_size(array)
/// }
/// 
/// let array = [MaybeUninit::new(3), MaybeUninit::new(5), MaybeUninit::new(8)];
/// 
/// unsafe{ assert_eq!(transmute_into_init(array), [3, 5, 8]); }
///
/// ```
///
/// This is the error you get if you tried to use `std::mem::transmute`.
///
/// ```text
/// error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
///  --> src/lib.rs:4:5
///   |
/// 4 |     std::mem::transmute(array)
///   |     ^^^^^^^^^^^^^^^^^^^
///   |
///   = note: source type: `[MaybeUninit<T>; 3]` (size can vary because of T)
///   = note: target type: `[T; 3]` (size can vary because of T)
/// ```
/// 
///
/// [`std::mem::transmute_copy`]: https://doc.rust-lang.org/std/mem/fn.transmute_copy.html
#[inline(always)]
pub unsafe fn transmute_ignore_size<T, U>(v: T) -> U {
    let v=ManuallyDrop::new(v);
    mem::transmute_copy::<T, U>(&v)
}

/// Transmutes a `Vec<T>` into a `Vec<U>`
///
/// # Safety
///
/// This function has the safety requirements of [`std::mem::transmute`] 
/// regarding transmuting from `T` to `U`.
/// `T` must also have the same alignment as `U`.
///
/// # Example
///
/// ```rust
/// use core_extensions::utils::transmute_vec;
///
/// use std::mem::ManuallyDrop;
///
/// unsafe{
///     assert_eq!(transmute_vec::<u32, i32>(vec![!0, 0, 1]), vec![-1, 0, 1]);
/// }
///
/// fn make(s: &str) -> ManuallyDrop<String> {
///     ManuallyDrop::new(String::from(s))
/// }
/// unsafe{
///     assert_eq!(
///         transmute_vec::<String, ManuallyDrop<String>>(vec!["hello".into(), "world".into()]),
///         vec![make("hello"), make("world")],
///     );
/// }
///
/// ```
///
/// [`std::mem::transmute`]: https://doc.rust-lang.org/std/mem/fn.transmute.html
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
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
/// undefined behaviour when reached you can use
/// [`std::hint::unreachable_unchecked`].
///
/// # Safety
///
/// It is undefined behaviour for this function to be reached at runtime at all.
///
/// The compiler is free to delete any code that reaches and depends on this function,
/// on the assumption that this branch can't be reached.
///
/// # Example
#[cfg_attr(feature = "bools", doc = " ```rust")]
#[cfg_attr(not(feature = "bools"), doc = " ```ignore")]
/// use core_extensions::BoolExt;
/// use core_extensions::utils::impossible;
///
/// mod non_zero{
///     use super::*;
///     #[derive(Debug,Copy,Clone)]
///     pub struct NonZero(usize);
///
///     impl NonZero{
///         pub fn new(value:usize) -> Option<NonZero> {
///             (value!=0).if_true(|| NonZero(value))
///         }
///         pub fn value(&self)->usize{
///             self.0
///         }
///     }
/// }
/// use self::non_zero::NonZero;
///
/// # fn main(){
///
/// fn div(numerator: usize, denom: Option<NonZero>) -> usize{
///     let denom = match denom {
///         Some(v) if v.value() == 0 => unsafe{
///             // unreachable: NonZero::value() can never be 0,
///             impossible()
///         },
///         Some(v) => v.value(),
///         None => 1,
///     };
///     numerator / denom
/// }
///
/// assert_eq!(div(60, NonZero::new(0)), 60);
/// assert_eq!(div(60, NonZero::new(1)), 60);
/// assert_eq!(div(60, NonZero::new(2)), 30);
/// assert_eq!(div(60, NonZero::new(3)), 20);
/// assert_eq!(div(60, NonZero::new(4)), 15);
/// assert_eq!(div(60, NonZero::new(5)), 12);
/// assert_eq!(div(60, NonZero::new(6)), 10);
///
///
/// # }
///
///
///
/// ```
///
/// [`std::hint::unreachable_unchecked`]:
/// https://doc.rust-lang.org/std/hint/fn.unreachable_unchecked.html
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
        std_::hint::unreachable_unchecked()
    }
}


////////////////////////////////////////////////////////////////////////////////


/// Takes the contents out of a `ManuallyDrop<T>`.
///
/// # Safety
///
/// After this function is called `slot` becomes uninitialized,
/// and must not be used again.
#[allow(dead_code)]
pub(crate) unsafe fn take_manuallydrop<T>(slot: &mut ManuallyDrop<T>) -> T {
    #[cfg(feature = "rust_1_42")]
    {
        ManuallyDrop::take(slot)
    }
    #[cfg(not(feature = "rust_1_42"))]
    {
        ::std_::ptr::read(slot as *mut ManuallyDrop<T> as *mut T)
    }
}


////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests{
    use super::*;

    use std_::cell::Cell;  
    use test_utils::DecOnDrop;  

    #[test]
    fn take_manuallydrop_test(){
        let count = Cell::new(10);
        let mut md = ManuallyDrop::new(DecOnDrop::new(&count));

        assert_eq!(count.get(), 10);

        let dod = unsafe{ take_manuallydrop(&mut md) };
        assert_eq!(count.get(), 10);

        drop(dod);
        assert_eq!(count.get(), 9);
    }
}

