/// Gets the [`ConstVal::VAL`] associated constant for a type.
/// 
/// Use this macro to avoid accidentally using inherent `VAL` associated cosntants.
/// 
/// # Examples
/// 
/// ### Implementing `ConstVal` manually
/// 
/// ```rust
/// use core_extensions::{ConstVal, getconst};
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
/// assert_eq!(getconst!(Foo), "hello");
/// assert_eq!(getconst!(Bar), "world");
/// 
/// ```
/// 
/// ### Inherent `VAL` associated constant
/// 
/// This demonstrates how inherent associated constants have priority over 
/// trait associated constants.
/// 
/// ```rust
/// use core_extensions::{ConstVal, getconst};
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
/// assert_eq!(getconst!(Foo), "hello");
/// assert_eq!(<Foo as ConstVal>::VAL, "hello");
/// assert_eq!(Foo::VAL, "world");
/// 
/// ```
/// 
/// [`ConstVal::VAL`]: trait.ConstVal.html#associatedconstant.VAL
#[macro_export]
macro_rules! getconst {
    ($ty:ty) => {<$ty as $crate::ConstVal>::VAL};
}


/// Declare a type that emulates a generic constant
/// 
/// For an example using all the syntax, you can look at the 
/// [All of the syntax section](#allthesyntax)
/// 
/// # Generated code
/// 
/// This macro geenerates:
/// 
/// -  A generic struct with the name and generic parameters of 
/// the `const` definition passed to this macro.
/// 
/// - An impl of the [`ConstVal`] trait for the struct, with the value for the constant .
/// 
/// - An inherent impl for the struct with a `VAL` associated constant,
/// to avoid requiring that [`ConstVal`] is imported to write `Foo::VAL`.
/// 
/// 
/// # Examples
/// 
/// ### Basic
/// 
/// ```rust
/// use core_extensions::{ConstVal, getconst, quasiconst};
/// 
/// quasiconst!{ const NONE[T]: Option<T> = None }
/// 
/// // `getconst` is the unambiguous way to get the constant
/// assert_eq!([getconst!(NONE<String>); 4], [None, None, None, None]);
///
/// // The `VAL` associated constant is another way to get the constant.
/// // 
/// // In other generic constants, the inherent `VAL` associated constant can be
/// // hidden by trait associated constants with fewer constraints.
/// assert_eq!([NONE::<u8>::VAL; 4], [None, None, None, None]);
/// 
/// ```
/// 
/// ### `ConstVal`
/// 
/// This example shows that you can use the generic constants with the [`ConstVal`] trait
/// 
/// ```
/// use core_extensions::{ConstDefault, ConstVal, quasiconst};
/// 
/// quasiconst!{
///     pub const PAIR[T: (ConstDefault)]: (T, T) = ConstDefault::DEFAULT;
/// }
/// 
/// fn constant<U: ConstVal>() -> U::Ty {
///     U::VAL
/// }
///
/// /// You can pass the type you want `constrained` to return as the first type argument.
/// fn constrained<T, U: ConstVal<Ty = T>>() -> T {
///     U::VAL
/// }
/// 
/// assert_eq!(constant::<PAIR<[u8; 3]>>(), ([0, 0, 0], [0, 0, 0]));
/// assert_eq!(constant::<PAIR<bool>>(), (false, false));
/// 
/// // Pair<_> is inferred to be `Pair<u8>`
/// assert_eq!(constrained::<(u8, u8), PAIR<_>>(), (0, 0));
///
/// // Pair<_> is inferred to be `Pair<String>`
/// assert_eq!(constrained::<(String, String), PAIR<_>>(), (String::new(), String::new()));
/// 
/// ```
/// 
/// <span id="allthesyntax"></span>
/// ### All of the syntax
/// 
/// Note: This macro allows const parameters(doesn't require enabling any features).
/// 
#[cfg_attr(not(feature = "const_generics"), doc = " ```rust")]
#[cfg_attr(feature = "const_generics", doc = " ```ignore")]
/// use core_extensions::{ConstDefault, getconst, quasiconst};
/// 
/// assert_eq!(getconst!(REFD<'static>), "");
/// assert_eq!(getconst!(REFD<'static, str>), "");
/// assert_eq!(getconst!(REFD<'static, [u8]>), &[]);
/// 
/// assert_eq!(getconst!(CONST_GEN<2>), [1, 3]);
/// assert_eq!(getconst!(CONST_GEN<4>), [1, 3, 6, 10]);
/// assert_eq!(getconst!(CONST_GEN<6>), [1, 3, 6, 10, 15, 21]);
/// 
/// quasiconst!{
///     /// You can document and use attributes on the generated `REFD` struct like this.
///     //
///     // Trait bounds in the generic parameter list must be enclosed in parentheses,
///     // that makes it possible for this macro to parse them,
///     // that's why `?Sized` is inside parentheses.
///     pub(crate) const REFD['a, T: 'a + (?Sized) = str]: &'a T
///     where[&'a T: ConstDefault]
///     = <&'a T>::DEFAULT;
///     
///     // The macro parses defaulted const parameters, but they're not supported by Rust yet.
///     pub const CONST_GEN[const N: usize]: [u128; N] = {
///         let mut array = [1u128; N];
///         let mut i = 1;
///         while i < array.len() {
///             array[i] += array[i - 1] + i as u128;
///             i += 1;
///         }
///         array
///     };
/// }
/// 
/// ```
/// 
/// [`ConstVal`]: ./trait.ConstVal.html
/// 
#[macro_export]
macro_rules! quasiconst {
    (
        $(
            $(#[$attr:meta])*
            $vis:vis const $ident:ident
            $( [$($generic_params:tt)*] )? 
            : $ty: ty
            $(where [$($constraints:tt)*] )?
            = $value:expr
        );*
        $(;)?
    ) => {
        $(
            $crate::__declare_const_inner!{
                ($(#[$attr])*, $vis, $ident, $ty, [$($($constraints)*)?], $value,)
                [$($($generic_params)*)? ,]
                [] [] [] []
            }
        )*
    };
}


#[doc(hidden)]
#[macro_export]
macro_rules! __declare_const_inner {
    (
        (
            $(#[$attr:meta])*,
            $vis:vis,
            $ident:ident,
            $ty: ty,
            [$($constraints:tt)*],
            $value:expr,
        )
        [$(,)*]
        [$($struct_params:tt)*  ]
        [$($impl_params:tt)*]
        [$($impl_args:tt)*]
        [$($phantoms:tt)*]
    ) => {
        $(#[$attr])*
        $vis struct $ident <$($struct_params)*> {
            _marker: $crate::__::PD<(
                $($phantoms)*
            )>
        }
        
        impl<$($impl_params)*> $crate::ConstVal for $ident<$($impl_args)*> 
        where
            $($constraints)*
        {
            type Ty = $ty;
            const VAL: <Self as $crate::ConstVal>::Ty = $value;
        }
        
        impl<$($impl_params)*> $ident<$($impl_args)*> 
        where
            $($constraints)*
        {
            $vis const VAL: <Self as $crate::ConstVal>::Ty = <Self as $crate::ConstVal>::VAL;
        }
    };
    (
        $other:tt
        [$lifetime:lifetime $(: $($bound:lifetime $(+)? )*)? , $($rem:tt)*]
        [$($struct_params:tt)*]
        [$($impl_params:tt)*]
        [$($impl_args:tt)*]
        [$($phantoms:tt)*]
    ) => {
        $crate::__declare_const_inner!{
            $other
            [$($rem)*]
            [$($struct_params)* $lifetime $(: $($bound + )*)?,]
            [$($impl_params)* $lifetime $(: $($bound + )*)?,]
            [$($impl_args)* $lifetime,]
            [$($phantoms)* &$lifetime (),]
        }
    };
    (
        $other:tt
        [
            $type:ident
            $(: $($bound:lifetime $(+)? )* $(($($tbound:tt)*) $(+)? )*  )?
            $(= $default:ty)?
            , $($rem:tt)*
        ]
        [$($struct_params:tt)*  ]
        [$($impl_params:tt)*]
        [$($impl_args:tt)*]
        [$($phantoms:tt)*]
    ) => {
        $crate::__declare_const_inner!{
            $other
            [$($rem)*]
            [$($struct_params)* $type $(: $($bound +)* $($($tbound)* +)*  )? $(= $default)? ,]
            [$($impl_params)* $type $(: $($bound +)* $($($tbound)* +)*  )? ,]
            [$($impl_args)* $type,]
            [$($phantoms)* $crate::__::PD<$type>,]
        }
    };
    (
        $other:tt
        [ const $constp:ident : $constty:ty $(= $default:expr)? , $($rem:tt)* ]
        [$($struct_params:tt)*  ]
        [$($impl_params:tt)*]
        [$($impl_args:tt)*]
        $phantoms:tt
    ) => {
        $crate::__declare_const_inner!{
            $other
            [$($rem)*]
            [$($struct_params)* const $constp: $constty $(= $default)? ,]
            [$($impl_params)* const $constp: $constty,]
            [$($impl_args)* $constp,]
            $phantoms
        }
    };
}
