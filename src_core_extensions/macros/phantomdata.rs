use std_::marker::PhantomData;

/// Maps a `PhantomData<T>` to a `PhantomData<U>` by using a `FnOnce(T) -> U` closure.
///
/// # Example
///
/// ```rust
/// use core_extensions::{as_phantom, map_phantomdata};
///
/// use std::{
///     borrow::Borrow,
///     fmt::Debug,
///     marker::PhantomData,
/// };
///
/// fn assert_impls<T>(_: PhantomData<T>) 
/// where
///     T: AsRef<str> + Borrow<str> + Debug
/// {}
///
/// let tuple = (100, ["hello"]);
///
/// // ghost is a `PhantomData<&'static str>`
/// let ghost = map_phantomdata!(as_phantom(&tuple), |x| x.1[0] );
///
/// assert_impls(ghost);
///
/// ```
#[macro_export]
macro_rules! map_phantomdata {
    ($expr:expr, $closure:expr) => (
        $crate::macros::phantomdata::ClosureTypes {
            param: $expr,
            closure: $closure,            
            returns: $crate::std_::marker::PhantomData,
        }.returns
    )
}


#[doc(hidden)]
#[repr(transparent)]
pub struct ClosureTypes<P, C: FnOnce(P) -> R, R> {
    pub param: PhantomData<P>,
    pub returns: PhantomData<R>,
    pub closure: C,
}



/// Gets the type of an expression as a `PhantomData`, without evaluating the expression.
///
/// # Example
///
/// ```rust
/// use core_extensions::expr_as_phantom;
///
/// use std::marker::PhantomData;
///
/// fn type_name<T>(_: PhantomData<T>) -> &'static str {
///     std::any::type_name::<T>()
/// }
///
/// let mut list = vec![0, 1];
///
/// // This block passed to the `expr_as_phantom` macro doesn't run.
/// let name = type_name(expr_as_phantom!({
///     list.extend(2..1_000u16);
///     list
/// }));
/// 
/// assert!(name.contains("Vec"));
/// 
/// assert_eq!(list, [0, 1])
///
/// ```
///
#[macro_export]
macro_rules! expr_as_phantom {
    ($e:expr) => ({
        let mut marker = $crate::std_::marker::PhantomData;

        if false {
            loop {}

            marker = $crate::as_phantom(&$e);
        }
        
        marker
    })
}


/// Gets the return type of a parameterless closure as a `PhantomData`
///
/// # Example
///
/// ```rust
/// use core_extensions::{IteratorExt, return_type_phantom};
///
/// use std::{
///     collections::HashSet,
///     iter::FromIterator,
///     marker::PhantomData,
/// };
///
/// fn collect<I, F>(_: PhantomData<F>, iter: I) -> F
/// where
///     I: IntoIterator,
///     F: FromIterator<I::Item>
/// {
///     iter.into_iter().collect()
/// }
///
/// let ty = return_type_phantom!(||{
///     let mut set = HashSet::new();
///     set.insert(100);
///     set
/// });
/// 
/// // `set` is a `HashSet<i32>`
/// let set = collect(ty, 1..=10);
/// 
/// assert_eq!(set.into_iter().sum_same(), 55);
///
/// ```
#[macro_export]
macro_rules! return_type_phantom {
    ($closure:expr) => (
        $crate::macros::phantomdata::UnitClosureReturnType {
            closure: $closure,            
            returns: $crate::std_::marker::PhantomData,
        }.returns
    )
}

#[doc(hidden)]
#[repr(transparent)]
pub struct UnitClosureReturnType<C: FnOnce() -> R, R> {
    pub closure: C,
    pub returns: PhantomData<R>,
}

