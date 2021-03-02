/// Gets the [`ConstDefault::DEFAULT`] associated constant for a type.
/// 
/// Use this macro to avoid accidentally using inherent `DEFAULT` associated cosntants.
/// 
/// # Argument
/// 
/// If a type is passed (ie: `const_default!(Foo)`),
/// this gets its [`ConstDefault::DEFAULT`] associated constant.
/// 
/// If nothing is passed (ie: `const_default!()`),
/// this is equivalent to [`ConstDefault::DEFAULT`],
/// inferring the type it gets the default value of.
/// 
/// # Examples
/// 
/// ### Basic
/// 
/// ```rust
/// use core_extensions::const_default;
/// 
/// assert_eq!(const_default!(u32), 0);
/// assert_eq!(const_default!(bool), false);
/// assert_eq!(const_default!((bool, Option<u32>)), (false, None));
/// assert_eq!(const_default!(([u32; 0], Vec<u32>)), ([], Vec::new()));
/// 
/// let list: &[u8] = const_default!();
/// assert!(list.is_empty());
/// 
/// let list: Vec<u32> = const_default!();
/// assert!(list.is_empty());
/// 
/// ```
/// 
/// ### Implementing `ConstDefault`
/// 
/// ```rust
/// use core_extensions::{ConstDefault, const_default};
/// 
/// #[derive(Debug, PartialEq)]
/// struct Foo<T> {
///     foo: u32,
///     bar: Option<T>,
/// }
/// 
/// impl<T> ConstDefault for Foo<T> {
///     const DEFAULT: Self = Self {
///         foo: const_default!(),
///         bar: const_default!(),
///     };
/// }
/// 
/// let expected = Foo{foo: 0, bar: None};
/// assert_eq!(const_default!(Foo<u32>), expected);
/// assert_eq!(Foo::<u32>::DEFAULT, expected);
/// 
/// 
/// ```
/// 
/// ### Inherent `DEFAULT` associated constant
/// 
/// This demonstrates how inherent associated constants have priority over 
/// trait associated constants.
/// 
/// ```rust
/// use core_extensions::{ConstDefault, const_default};
/// 
/// #[derive(Debug, PartialEq)]
/// struct Foo(u32);
/// 
/// impl ConstDefault for Foo {
///     const DEFAULT: Self = Foo(0);
/// }
/// 
/// impl Foo {
///     const DEFAULT: Self = Foo(3333);
/// }
/// 
/// assert_eq!(const_default!(Foo), Foo(0));
/// assert_eq!(<Foo as ConstDefault>::DEFAULT, Foo(0));
/// assert_eq!(Foo::DEFAULT, Foo(3333));
/// 
/// ```
/// 
/// [`ConstDefault::DEFAULT`]: trait.ConstDefault.html#associatedconstant.DEFAULT
#[cfg(feature = "const_default")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_default")))]
#[macro_export]
macro_rules! const_default {
    () => {
        $crate::ConstDefault::DEFAULT
    };
    ($This:ty) => {
        <$This as $crate::ConstDefault>::DEFAULT
    };
}