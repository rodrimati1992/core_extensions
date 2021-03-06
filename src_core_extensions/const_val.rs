/// For types that represent constants.
/// 
/// # Examples
/// 
/// ### Manual impl
/// 
/// ```rust
/// use core_extensions::{ConstVal, constval};
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
/// assert_eq!(constval!(Foo), 3);
/// assert_eq!(constval!(Bar<Foo>), 4);
/// assert_eq!(constval!(Bar<Bar<Foo>>), 6);
/// assert_eq!(constval!(Bar<Bar<Bar<Foo>>>), 9);
/// 
/// ```
/// 
pub trait ConstVal {
    /// The type of the constant this represents.
    type Ty;

    /// The constant this represents.
    const VAL: Self::Ty;

    /// Gets the constant this represents.
    fn const_val(&self) -> Self::Ty {
        Self::VAL
    }
}