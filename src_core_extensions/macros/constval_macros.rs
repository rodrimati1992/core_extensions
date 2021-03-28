/// Gets the [`ConstVal::VAL`] associated constant for a type.
/// 
/// Use this macro to unambiguously use the [`ConstVal::VAL`] associated constant,
/// as opposed to an inherent `VAL` associated constant,
/// or a `VAL` associated constant from another trait.
/// 
/// # Examples
/// 
/// ### Quasiconstants
/// 
/// Using the [`quasiconst`] macro to declare (generic) constants.
/// 
/// ```rust
/// use core_extensions::{getconst, quasiconst, IntegerExt};
/// 
/// #[derive(Debug, PartialEq)]
/// pub struct Single<T>(pub T);
/// 
/// quasiconst!{const Foo: &'static str = "hello"}
/// quasiconst!{const Bar: &'static str = "world"}
/// quasiconst!{const SINGLE_INT[T: IntegerExt = u8]: Single<T> = Single(T::ONE) }
/// 
/// assert_eq!(getconst!(Foo), "hello");
/// assert_eq!(getconst!(Bar), "world");
/// 
/// // `SINGLE_INT` == `SINGLE_INT<u8>`, because of the defaulted type parameter
/// assert_eq!(getconst!(SINGLE_INT), Single(1_u8)); 
/// 
/// assert_eq!(getconst!(SINGLE_INT<u16>), Single(1_u16));
/// 
/// assert_eq!(getconst!(SINGLE_INT<_>), Single(1_i8));
/// 
/// // `Type<..>` is special syntax from `getconst`, to infer all generic parameters.
/// assert_eq!(getconst!(SINGLE_INT<..>), Single(1u128));
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
/// [`quasiconst`]: ./macro.quasiconst.html
#[macro_export]
macro_rules! getconst {
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* <..>
    ) => ({
        use $crate::ConstVal;
        $(:: $(@$leading@)? )? $first $(:: $trailing)* ::__CORE_EXTENSIONS__05FFE5XDEJHD07CTUSQMW
    });
    ($ty:ty) => {<$ty as $crate::ConstVal>::VAL};
}


/// Declare types that emulate generic constants.
/// 
/// # Syntax
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
///     pub const PAIR[T: ConstDefault]: (T, T) = ConstDefault::DEFAULT;
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
/// Note: This macro allows const parameters
/// (and doesn't require enabling the "const_generics" feature to use them).
/// 
#[cfg_attr(not(feature = "const_generics"), doc = " ```ignore")]
#[cfg_attr(feature = "const_generics", doc = " ```rust")]
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
///     /// You can document and use attributes on the generated `REFD` struct.
///     pub(crate) const REFD['a: 'a, T: 'a + ?Sized = str]: &'a T
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
                (
                    $(#[$attr])*,
                    $vis,
                    $ident,
                    $ty,
                    [$($($constraints)*)?],
                    $value,
                    concat!("Cosntructs a `", stringify!($ident), "` (the type)"),
                )
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
            $new_doc:expr,
        )
        [$(,)*]
        [$($struct_params:tt)*  ]
        [$($impl_params:tt)*]
        [$($impl_args:tt)*]
        [$($phantoms:tt)*]
    ) => {
        $(#[$attr])*
        #[allow(non_camel_case_types)]
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
            #[doc = $new_doc]
            $vis const NEW: Self = Self{_marker: $crate::__::PD};

            /// The constant that this type represents.
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
            [$($struct_params)* $type $(= $default)? ,]
            [$($impl_params)* $type ,]
            [$($impl_args)* $type,]
            [$($phantoms)* $crate::__::PD<$type>,]
        }
    };
    (
        $other:tt
        [
            $type:ident
            : $($rem:tt)*
        ]
        $struct_params:tt
        $impl_params:tt
        $impl_args:tt
        $phantoms:tt
    ) => {
        $crate::__declare_const_type_param_bounds!{
            (
                $other
                $type
                $struct_params
                $impl_params
                $impl_args
                $phantoms
            )
            []
            [ + $($rem)*]
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
    (
        $other:tt
        [
            $($rem:tt)*
        ]
        $struct_params:tt
        $impl_params:tt
        $impl_args:tt
        $phantoms:tt
    ) => {
        compile_error!{concat!(
            "Cannot parse these generics:\n\t",
            $(stringify!($rem),)*
        )}
    };
}


#[doc(hidden)]
#[macro_export]
macro_rules! __declare_const_type_param_bounds {
    (
        (
            $other:tt
            $type:ident
            [$($struct_params:tt)*]
            [$($impl_params:tt)*]
            [$($impl_args:tt)*]
            [$($phantoms:tt)*]
        )
        [$($bounds:tt)*]
        [ $(= $default:ty)? $(, $($rem:tt)*)? ]
    ) => {
        $crate::__declare_const_inner!{
            $other
            [$($($rem)*)?]
            [$($struct_params)* $type : $($bounds)* $(= $default)? ,]
            [$($impl_params)* $type : $($bounds)*,]
            [$($impl_args)* $type,]
            [$($phantoms)* $crate::__::PD<$type>,]
        }
    };
    (
        $fixed:tt
        [$($boundts:tt)*]
        [ + $lt:lifetime $($rem:tt)* ]
    ) => {
        $crate::__declare_const_type_param_bounds!{
            $fixed
            [$($boundts)* $lt + ]
            [$($rem)*]
        }
    };
    (
        $fixed:tt
        [$($boundts:tt)*]
        [ + ($($parenthesized:tt)*) $($rem:tt)* ]
    ) => {
        $crate::__declare_const_type_param_bounds!{
            $fixed
            [$($boundts)* ($($parenthesized)*) + ]
            [$($rem)*]
        }
    };
    (
        $fixed:tt
        $prev_bounds:tt
        [ + $rem_bounds:ty $(= $default:ty)? $(, $($rem:tt)*)? ]
    ) => {
        $crate::__::__priv_remove_non_delimiter!{
            $rem_bounds

            $crate::__declare_const_type_param_finish!{
                $fixed
                $prev_bounds
                [ ($($default)?) $(, $($rem)*)? ]
            }
        }
    };
    (
        $fixed:tt
        [$($boundts:tt)*]
        [ $($rem:tt)* ]
    ) => {
        compile_error!{concat!(
            "Cannot parse bounds at the start of these tokens,\n\
             you need to wrap them in parentheses:\n\t",
            $(stringify!($rem),)*
        )}
    };
}




#[doc(hidden)]
#[macro_export]
macro_rules! __declare_const_type_param_finish {
    (
        (
            $other:tt
            $type:ident
            [$($struct_params:tt)*]
            [$($impl_params:tt)*]
            [$($impl_args:tt)*]
            [$($phantoms:tt)*]
        )
        [$($bounds:tt)*]
        [ ($($($default:tt)+)?) $(, $($rem:tt)*)? ]
        ($($rem_bounds:tt)*)
    ) => {
        $crate::__declare_const_inner!{
            $other
            [$($($rem)*)?]
            [$($struct_params)* $type : $($bounds)* $($rem_bounds)* $(= $($default)+ )? ,]
            [$($impl_params)* $type : $($bounds)* $($rem_bounds)*,]
            [$($impl_args)* $type,]
            [$($phantoms)* $crate::__::PD<$type>,]
        }
    };
}
