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


/////////////////////////////////////////////////////////


/// A wrapper type to run a closure at the end of the scope.
///
/// This allows construction with an explicitly captured value,
/// so that it can be used before the end of the scope.
///
/// ```rust
/// use core_extensions::utils::RunOnDrop;
///
/// fn main() { 
///     let mut guard = RunOnDrop::new("Hello".to_string(), |string|{
///         assert_eq!(string, "Hello, world!");
///     });
///
///     assert_eq!(guard.get(), "Hello");
///     
///     guard.get_mut().push_str(", world!");
/// }   
///
/// ```
pub struct RunOnDrop<T, F>
where
    F: FnOnce(T),
{
    value: ManuallyDrop<T>,
    function: ManuallyDrop<F>,
}

impl<T, F> RunOnDrop<T, F>
where
    F: FnOnce(T),
{
    /// Constructs this RunOnDrop.
    #[inline(always)]
    pub fn new(value: T, function: F) -> Self {
        Self {
            value: ManuallyDrop::new(value),
            function: ManuallyDrop::new(function),
        }
    }
}

impl<T, F> RunOnDrop<T, F>
where
    F: FnOnce(T),
{
    /// Reborrows the wrapped value.
    #[inline(always)]
    pub fn get(&self) -> &T {
        &*self.value
    }

    /// Reborrows the wrapped value mutably.
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut T {
        &mut *self.value
    }

    /// Extracts the wrapped value, preventing the closure from running at the end of the scope.
    pub fn into_inner(self) -> T {
        let mut this = ManuallyDrop::new(self);
        unsafe{
            let ret = take_manuallydrop(&mut this.value);
            ManuallyDrop::drop(&mut this.function);
            ret
        }
    }

}

impl<'a, T, F> Drop for RunOnDrop<T, F>
where
    F: FnOnce(T),
{
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            let value = take_manuallydrop(&mut self.value);
            let function = take_manuallydrop(&mut self.function);
            function(value);
        }
    }
}


////////////////////////////////////////////////////////////////////////////////


/// Takes the contents out of a `ManuallyDrop<T>`.
///
/// # Safety
///
/// After this function is called `slot` becomes uninitialized and
/// must not be used again.
unsafe fn take_manuallydrop<T>(slot: &mut ManuallyDrop<T>) -> T {
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
    fn drop_guard() {
        let count = Cell::new(0);
        
        {
            let guard = RunOnDrop::new(DecOnDrop::new(&count), |rod|{
                assert_eq!(count.get(), 15);
                drop(rod);
                assert_eq!(count.get(), 14);
            });

            assert_eq!(count.get(), 0);
            count.set(16);

            let clone = guard.get().clone();
            assert_eq!(count.get(), 16);
            drop(clone);
            assert_eq!(count.get(), 15);

        }

        assert_eq!(count.get(), 14);
    }

    #[test]
    fn unwrap_run_on_drop() {
        let count = Cell::new(0);
        
        {
            let guard = RunOnDrop::new(DecOnDrop::new(&count), |rod|{
                assert_eq!(count.get(), 15);
                drop(rod);
                assert_eq!(count.get(), 14);
            });

            assert_eq!(count.get(), 0);
            count.set(16);

            let clone = guard.get().clone();
            assert_eq!(count.get(), 16);
            drop(clone);
            assert_eq!(count.get(), 15);

            let rod = guard.into_inner();
            assert_eq!(count.get(), 15);
            drop(rod);
            assert_eq!(count.get(), 14);
        }

        assert_eq!(count.get(), 14);
    }


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








