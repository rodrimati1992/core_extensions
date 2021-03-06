/// Gets the [`ConstVal::VAL`] associated constant for a type.
/// 
/// Use this macro to avoid accidentally using inherent `VAL` associated cosntants.
/// 
/// # Examples
/// 
/// ### Implementing `ConstVal` manually
/// 
/// ```rust
/// use core_extensions::{ConstVal, constval};
/// 
/// struct Foo;
///
/// struct Bar;
/// 
/// impl ConstVal for Foo {
///     type Ty = &'static str;
///     const VAL: Self::Ty = "hello";
/// }
/// 
/// impl ConstVal for Bar {
///     type Ty = &'static str;
///     const VAL: Self::Ty = "world";
/// }
/// 
/// assert_eq!(constval!(Foo), "hello");
/// assert_eq!(constval!(Bar), "world");
/// 
/// ```
/// 
/// ### Inherent `VAL` associated constant
/// 
/// This demonstrates how inherent associated constants have priority over 
/// trait associated constants.
/// 
/// ```rust
/// use core_extensions::{ConstVal, constval};
/// 
/// #[derive(Debug, PartialEq)]
/// struct Foo(u32);
/// 
/// impl ConstVal for Foo {
///     type Ty = &'static str;
///     const VAL: Self::Ty = "hello";
/// }
/// 
/// impl Foo {
///     const VAL: &'static str = "world";
/// }
/// 
/// assert_eq!(constval!(Foo), "hello");
/// assert_eq!(<Foo as ConstVal>::VAL, "hello");
/// assert_eq!(Foo::VAL, "world");
/// 
/// ```
/// 
/// [`ConstVal::VAL`]: trait.ConstVal.html#associatedconstant.VAL
#[macro_export]
macro_rules! constval {
    ($ty:ty) => {<$ty as $crate::ConstVal>::VAL};
}