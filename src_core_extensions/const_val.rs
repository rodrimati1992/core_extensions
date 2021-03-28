/// For types that represent constants.
/// 
/// # Examples
/// 
/// ### Quasiconstants
/// 
/// Using the [`quasiconst`] macro to declare types that implement this trait,
/// and emulate generic constants.
/// 
/// This example requires Rust 1.46.0, because it uses a while loop in a const context
/// 
#[cfg_attr(not(feature = "rust_1_46"), doc = " ```ignore")]
#[cfg_attr(feature = "rust_1_46", doc = " ```rust")]
/// use core_extensions::{ConstVal, getconst, quasiconst};
/// 
/// const LEN: usize = 8;
///
/// quasiconst!{
///     const FIBNUMS[T: ConstVal<Ty = u128>]: &'static [u128; LEN] = {
///         let mut ret = [T::VAL; LEN];
///         let mut i = 2;
///         while i < LEN {
///             ret[i] = ret[i - 1] + ret[i - 2];
///             i += 1;
///         }
///         &{ret}
///     };
///     
///     const ONE: u128 = 1;
///     const FOUR: u128 = 4;
///     const SEVEN: u128 = 7;
/// }
/// 
/// assert_eq!(getconst!(FIBNUMS<ONE>), &[1, 1, 2, 3, 5, 8, 13, 21]);
/// assert_eq!(getconst!(FIBNUMS<FOUR>), &[4, 4, 8, 12, 20, 32, 52, 84]);
/// assert_eq!(getconst!(FIBNUMS<SEVEN>), &[7, 7, 14, 21, 35, 56, 91, 147]);
/// 
/// 
/// ```
/// 
/// ### Manual impl
/// 
/// ```rust
/// use core_extensions::{ConstVal, getconst};
/// 
/// struct Foo;
///
/// struct Bar<T>(std::marker::PhantomData<T>);
/// 
/// impl ConstVal for Foo {
///     type Ty = u32;
///     const VAL: Self::Ty = 3;
/// }
/// 
/// impl<T> ConstVal for Bar<T> 
/// where
///     T: ConstVal<Ty = u32>,
/// {
///     type Ty = u32;
///
///     const VAL: Self::Ty = T::VAL * 3 / 2;
/// }
/// 
/// assert_eq!(getconst!(Foo), 3);
/// assert_eq!(getconst!(Bar<Foo>), 4);
/// assert_eq!(getconst!(Bar<Bar<Foo>>), 6);
/// assert_eq!(getconst!(Bar<Bar<Bar<Foo>>>), 9);
/// 
/// ```
/// 
/// 
/// [`quasiconst`]: ./macro.quasiconst.html
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_val")))]
pub trait ConstVal {
    /// The type of the constant this represents.
    type Ty;

    /// The constant this represents.
    const VAL: Self::Ty;

    #[doc(hidden)]
    const __CORE_EXTENSIONS__05FFE5XDEJHD07CTUSQMW: Self::Ty = Self::VAL;

    /// Gets the constant this represents.
    fn const_val(&self) -> Self::Ty {
        Self::VAL
    }
}