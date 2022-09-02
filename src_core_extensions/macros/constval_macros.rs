/// Gets the [`ConstVal::VAL`](trait.ConstVal.html#associatedconstant.VAL)
/// associated constant for a type.
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
#[cfg_attr(not(feature = "integers"), doc = " ```ignore")]
#[cfg_attr(feature = "integers", doc = " ```rust")]
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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_val")))]
#[macro_export]
macro_rules! getconst {
    (
        $(:: $(@$leading:tt@)? )? $($path:ident)::* <..>
    ) => ({
        use $crate::ConstVal;
        $(:: $(@$leading@)? )? $($path)::* ::__CORE_EXTENSIONS__05FFE5XDEJHD07CTUSQMW
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
/// This macro generates:
/// 
/// -  A generic zero-sized struct with the name and generic parameters of 
/// the `const` definition passed to this macro.
/// 
/// - An impl of the [`ConstVal`] trait for the struct, with the value for the constant .
/// 
/// - An inherent `VAL` associated constant for the struct,
/// to avoid requiring that [`ConstVal`] is imported to write `Foo::VAL`.
/// 
/// - An inherent `NEW` associated constant that constructs the struct.
/// 
/// # Examples
/// 
/// ### Basic
/// 
/// ```rust
/// use core_extensions::{getconst, quasiconst};
/// 
/// quasiconst!{ const NONE<T>: Option<T> = None }
/// 
/// // `getconst` is the unambiguous way to get the constant
/// assert_eq!([getconst!(NONE<String>); 4], [None, None, None, None]);
///
/// // The `VAL` associated constant is another way to get the constant.
/// //
/// // I get worse compiler errors with `::VAL` than with `getconst`
/// // when the bounds of the generic constant aren't satisfied.
/// assert_eq!([NONE::<u8>::VAL; 4], [None, None, None, None]);
/// 
/// ```
/// 
/// ### `ConstVal`
/// 
/// This example shows that you can use the generic constants with the [`ConstVal`] trait
/// 
#[cfg_attr(not(all(feature = "const_default", feature = "alloc")), doc = " ```ignore")]
#[cfg_attr(all(feature = "const_default", feature = "alloc"), doc = " ```rust")]
/// use core_extensions::{ConstDefault, ConstVal, quasiconst};
/// 
/// quasiconst!{
///     pub const PAIR<T: ConstDefault>: (T, T) = ConstDefault::DEFAULT;
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
/// ### Newer syntax
/// 
/// This is the newer syntax that looks closest to what generic constants would look like.
/// 
/// Note: This macro allows const parameters
/// (and doesn't require enabling the "rust_1_51" feature to use them).
/// 
#[cfg_attr(not(all(feature = "const_default", feature = "rust_1_51")), doc = " ```ignore")]
#[cfg_attr(all(feature = "const_default", feature = "rust_1_51"), doc = " ```rust")]
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
///     pub(crate) const REFD<'a: 'a, T: 'a + ?Sized = str>: &'a T
///     where
///         &'a T: ConstDefault
///     = <&'a T>::DEFAULT;
/// }
/// quasiconst!{
///     // Defaulted const parameters require Rust 1.59.0
///     pub const CONST_GEN<const N: usize>: [u128; N] = {
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
/// ### Older syntax
/// 
/// This is the older (but equally supported) syntax for generic parameters and
/// where clauses, using `[]` for both of them.
/// 
#[cfg_attr(not(all(feature = "const_default", feature = "rust_1_51")), doc = " ```ignore")]
#[cfg_attr(all(feature = "const_default", feature = "rust_1_51"), doc = " ```rust")]
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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_val")))]
#[macro_export]
macro_rules! quasiconst {
    (
        $(#[$attr:meta])*
        $vis:vis const $ident:ident
        $( [$($generic_params:tt)*] )? 
        : $ty: ty
        $(where [$($constraints:tt)*] )?
        = $value:expr
        $(; $($rem:tt)* )?
    ) => {
            $crate::parse_generics!{
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
                }

                ($($($generic_params)*)?)
            }

        $($crate::quasiconst!{ $($rem)* })?
    };
    (
        $(#[$attr:meta])*
        $vis:vis const $ident:ident
        $($rem:tt)*
    ) => {
        $crate::parse_generics_and_where!{
            $crate::__declare_const_angle_inner!{
                (
                    $(#[$attr])*,
                    $vis,
                    $ident,
                    concat!("Cosntructs a `", stringify!($ident), "` (the type)"),
                )
            }
            
            ($($rem)*)
        }
    };
    ($(;)?)=>{};
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
        ($($struct_params:tt)*)
        ($($impl_params:tt)*)
        ($($impl_args:tt)*)
        ($($phantoms:tt)*)
    ) => {
        $(#[$attr])*
        #[allow(non_camel_case_types)]
        $vis struct $ident <$($struct_params)*> {
            _marker: $($phantoms)*
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
}


#[doc(hidden)]
#[macro_export]
macro_rules! __declare_const_angle_inner {
    (
        (
            $(#[$attr:meta])*,
            $vis:vis,
            $ident:ident,
            $new_doc:expr,
        )
        $struct_params:tt
        $impl_params:tt
        $impl_args:tt
        $phantoms:tt
        (: $ty: ty)
        ($($where:tt)*)
        (= $value:expr $(; $($($more:tt)+)? )? )
    ) => {

        $crate::__declare_const_inner!{
            (
                $(#[$attr])*,
                $vis,
                $ident,
                $ty,
                [$($where)*],
                $value,
                $new_doc,
            )
            $struct_params
            $impl_params
            $impl_args
            $phantoms
        }

        $($(
            $crate::quasiconst!{
                $($more)*
            }
        )?)?
    }
}


