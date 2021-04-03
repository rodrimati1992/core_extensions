use crate::utils::take_manuallydrop;

use std_::mem::ManuallyDrop;

#[cfg(test)]
mod tests;


/// A wrapper type that runs a closure at the end of the scope.
///
/// This takes both a value and a closure(that takes the value as a parameter),
/// allowing you to access the value before the closure runs.
///
/// # Example
///
/// ```rust
/// use core_extensions::RunOnDrop;
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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "on_drop")))]
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



