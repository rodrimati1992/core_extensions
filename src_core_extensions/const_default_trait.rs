//! Contains marker traits representing a variety of guarantees provided by the implementors.
//!
//!
//!

use crate::const_default;

use std_::{
    cell::{Cell,RefCell,UnsafeCell},
    cmp::Reverse,
    marker::PhantomData,
    mem::ManuallyDrop,
    num::Wrapping,
    sync::atomic::{AtomicUsize,AtomicIsize,AtomicBool},
};

#[allow(unused_imports)]
use std_::sync::atomic;

/// A const equivalent of the `Default` trait.
///
/// This trait can be derived with the [`ConstDefault`] derive macro
/// (requires the "derive" feature).
///
/// # Features
///
/// Enabling the "rust_1_51" feature allows arrays of all lengths to implement this trait,
/// otherwise it's only implemented for arrays up to 32 elements long.
///
/// # Example
///
/// Manually implementing `ConstDefault` for a struct
///
/// ```rust
/// use core_extensions::{ConstDefault, const_default};
/// 
/// #[derive(Debug,PartialEq)]
/// struct Point<T>{
///     x: T,
///     y: T,
/// }
///
/// impl<T> ConstDefault for Point<T>
/// where
///     T: ConstDefault
/// {
///     const DEFAULT: Self = Point {
///         // `const_default!()` is equivalent to `ConstDefault::DEFAULT`
///         x: const_default!(),
///         y: const_default!(),
///     };
/// }
///
/// # fn main(){
/// assert_eq!(const_default!(Point<u8>), Point{x: 0, y: 0});
/// assert_eq!(const_default!(Point<f32>), Point{x: 0.0, y: 0.0});
/// assert_eq!(const_default!(Point<Option<()>>), Point{x: None, y: None});
/// # }
/// ```
/// 
/// [`ConstDefault`]: ./derive.ConstDefault.html    
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_default")))]
pub trait ConstDefault: Sized {
    /// The default value for `Self`.
    const DEFAULT: Self;
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "rust_1_51")]
macro_rules! impl_array_const_default {
    ()=>{
        /// When the "const_params" feature is disabled,
        /// the ConstDefault trait is implemented for arrays up to 32 elements long.
        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_params")))]
        impl<T: ConstDefault, const N: usize> ConstDefault for [T; N] {
            const DEFAULT: Self = [T::DEFAULT; N];
        }
    }
}

#[cfg(feature = "rust_1_51")]
impl_array_const_default!{}

////////////////////////////////////////////////////////////////////////////////

#[cfg(not(feature = "rust_1_51"))]
macro_rules! impl_array_const_default_inner {
    ([ $extra_bounds:ident ] 
        $(($size:expr)=[ $($t:ident,)* ]),*
        $(,)*
    )=>{
        $(
            impl<T> ConstDefault for [T;$size]
            where T:ConstDefault + $extra_bounds
            {
                const DEFAULT: Self=[ $($t::DEFAULT),* ];
            }
        )*
    };
}

#[cfg(not(feature = "rust_1_51"))]
macro_rules! impl_array_const_default {
    (@inner [ $extra_bounds:ident ] 
        $(($size:expr)=[ $($t:ident,)* ]),*
        $(,)*
    )=>{
        $(
            impl<T> ConstDefault for [T;$size]
            where T:ConstDefault + $extra_bounds
            {
                const DEFAULT: Self=[ $($t::DEFAULT),* ];
            }
        )*
    };
    ($($args:tt)*) => (
        impl_array_const_default_inner!{[Sized] $($args)* }
    );
}

/*
fn main(){
    for i in 0..=32{
        print!("{}=[",i);
        for j in 0..i {
            print!("T,");
        }
        println!("],");
    }
}
*/

#[cfg(not(feature = "rust_1_51"))]
impl<T> ConstDefault for [T;0]{
    const DEFAULT: Self=[];
}

#[cfg(not(feature = "rust_1_51"))]
impl_array_const_default! {
    (1)=[T,],
    (2)=[T,T,],
    (3)=[T,T,T,],
    (4)=[T,T,T,T,],
    (5)=[T,T,T,T,T,],
    (6)=[T,T,T,T,T,T,],
    (7)=[T,T,T,T,T,T,T,],
    (8)=[T,T,T,T,T,T,T,T,],
    (9)=[T,T,T,T,T,T,T,T,T,],
    (10)=[T,T,T,T,T,T,T,T,T,T,],
    (11)=[T,T,T,T,T,T,T,T,T,T,T,],
    (12)=[T,T,T,T,T,T,T,T,T,T,T,T,],
    (13)=[T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (14)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (15)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (16)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (17)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (18)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (19)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (20)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (21)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (22)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (23)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (24)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (25)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (26)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (27)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (28)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (29)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (30)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (31)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
    (32)=[T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,],
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_tuple_const_default {
    ($($ty:ident),*) => (
        impl_tuple_const_default!{@inner [Sized] $($ty),* }
    );
    (@inner [ $extra_bounds:ident ] $($ty:ident),*)=>{
        impl<$($ty),*> ConstDefault for ($($ty,)*)
        where $($ty:ConstDefault + $extra_bounds ,)*
        {
            const DEFAULT: Self=($($ty::DEFAULT,)*);
        }
    };
}

impl_tuple_const_default! {}
impl_tuple_const_default! {A}
impl_tuple_const_default! {A,B}
impl_tuple_const_default! {A,B,C}
impl_tuple_const_default! {A,B,C,D}
impl_tuple_const_default! {A,B,C,D,E}
impl_tuple_const_default! {A,B,C,D,E,F}
impl_tuple_const_default! {A,B,C,D,E,F,G}
impl_tuple_const_default! {A,B,C,D,E,F,G,H}
impl_tuple_const_default! {A,B,C,D,E,F,G,H,I}
impl_tuple_const_default! {A,B,C,D,E,F,G,H,I,J}
impl_tuple_const_default! {A,B,C,D,E,F,G,H,I,J,K}
impl_tuple_const_default! {A,B,C,D,E,F,G,H,I,J,K,L}
impl_tuple_const_default! {A,B,C,D,E,F,G,H,I,J,K,L,M}
impl_tuple_const_default! {A,B,C,D,E,F,G,H,I,J,K,L,M,N}
impl_tuple_const_default! {A,B,C,D,E,F,G,H,I,J,K,L,M,N,O}
impl_tuple_const_default! {A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P}


macro_rules! impl_const_default{
    (
        $( 
            $(#[$attr:meta])*

            for[$($for:tt)*]
            $ty:ty = 
            $def:expr
        ),*
        $(,)*
    )=>{
        $(
            $(#[$attr])*
            impl<$($for)*> ConstDefault for $ty {
                const DEFAULT: Self= $def;
            }
        )*
    }
}

impl_const_default!{
    for[] isize=0,
    for[] usize=0,
    for[] i8=0,
    for[] u8=0,
    for[] i16=0,
    for[] u16=0,
    for[] i32=0,
    for[] u32=0,
    for[] i64=0,
    for[] u64=0,
    for[] f32=0.0,
    for[] f64=0.0,
    for[] bool=false,
    for[] char='\x00',
    for[T: ?Sized] PhantomData<T> = PhantomData,
    for[T] Option<T> = None,
    for['a] &'a str = "",
    for['a, T:'a] &'a [T] = &[],

    for[T: ConstDefault] Wrapping<T> = Wrapping(T::DEFAULT),
    for[T: ConstDefault] Reverse<T> = Reverse(T::DEFAULT),
    for[T] std_::iter::Empty<T> = std_::iter::empty(),
}

impl_const_default!{
    for[] AtomicUsize = AtomicUsize::new(0),
    for[] AtomicIsize = AtomicIsize::new(0),
    for[] AtomicBool = AtomicBool::new(false),
    for[T: ConstDefault] Cell<T> = Cell::new(T::DEFAULT),
    for[T: ConstDefault] RefCell<T> = RefCell::new(T::DEFAULT),
    for[T: ConstDefault] UnsafeCell<T> = UnsafeCell::new(T::DEFAULT),
}

impl_const_default!{
    for[] ::std_::time::Duration = ::std_::time::Duration::from_secs(0),
}

impl_const_default!{
    for[] i128=0,
    for[] u128=0,
}

impl_const_default!{
    for[T: ConstDefault] ManuallyDrop<T> = ManuallyDrop::new(T::DEFAULT),
}

#[cfg(not(target_arch = "powerpc"))]
impl_const_default!{
    for[] atomic::AtomicI8 = atomic::AtomicI8::new(0),
    for[] atomic::AtomicU8 = atomic::AtomicU8::new(0),
    for[] atomic::AtomicI16 = atomic::AtomicI16::new(0),
    for[] atomic::AtomicU16 = atomic::AtomicU16::new(0),
    for[] atomic::AtomicI32 = atomic::AtomicI32::new(0),
    for[] atomic::AtomicU32 = atomic::AtomicU32::new(0),
    for[] atomic::AtomicI64 = atomic::AtomicI64::new(0),
    for[] atomic::AtomicU64 = atomic::AtomicU64::new(0),
}

#[cfg(feature = "alloc")]
use alloc::{
    borrow::{Cow, ToOwned},
    collections::LinkedList,
    string::String,
    vec::Vec,
};

#[cfg(feature = "alloc")]
impl_const_default!{
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
    for[T] Vec<T> = Self::new(),

    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
    for[] String = Self::new(),

    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
    for[T] LinkedList<T> = Self::new(),
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
impl<'a, B: ?Sized + ToOwned + 'a> ConstDefault for Cow<'a, B>
where
    B::Owned: ConstDefault + 'a,
{
    const DEFAULT: Self = Cow::Owned(<B::Owned as ConstDefault>::DEFAULT);
}


#[cfg(test)]
mod tests{
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct NoDefault;

    #[derive(Debug,PartialEq)]
    struct NonCopy;

    impl ConstDefault for NonCopy{
        const DEFAULT:Self=NonCopy;
    }

    impl Drop for NonCopy{
        fn drop(&mut self){}
    }

    // Make sure that `const_default` returns a constant
    macro_rules! const_def_assert {
        ($This:ty) => {{
            const T:$This=const_default!($This);
            T
        }};
    }

    #[test]
    fn always_available(){
        assert_eq!(const_def_assert!([u8;6]), [0;6]);
        assert_eq!(const_def_assert!([u8;16]), [0;16]);
        assert_eq!(const_def_assert!([u8;32]), [0;32]);
        
        assert_eq!(const_def_assert!(()), ());
        assert_eq!(const_def_assert!((u8,)), (0,));
        assert_eq!(const_def_assert!((u8,u16)), (0,0));
        assert_eq!(const_def_assert!((u8,u16,u32)), (0,0,0));
        
        assert_eq!(const_def_assert!(u8), 0);
        assert_eq!(const_def_assert!(f64), 0.0);
        assert_eq!(const_def_assert!(&str), "");
        assert_eq!(const_def_assert!(&[u8]), &[]);
        assert_eq!(const_def_assert!(Option<NoDefault>), None);
        
        assert_eq!(const_def_assert!(AtomicUsize).into_inner(), 0);
        assert_eq!(const_def_assert!(AtomicIsize).into_inner(), 0);
        assert_eq!(const_def_assert!(AtomicBool).into_inner(), false);
        
        assert_eq!(const_def_assert!(Wrapping<u8>).0, 0);
        assert_eq!(const_def_assert!(Wrapping<bool>).0, false);

        assert_eq!(const_def_assert!(Reverse<u8>).0, 0);
        assert_eq!(const_def_assert!(Reverse<bool>).0, false);

        assert_eq!(const_def_assert!(std_::iter::Empty<u8>).next(), None);
    }

    #[test]
    fn for_rust_1_22(){
        assert_eq!(const_def_assert!([NonCopy;2]), [NonCopy,NonCopy]);
        
        assert_eq!(const_def_assert!(()), ());
        assert_eq!(const_def_assert!((NonCopy,)), (NonCopy,));
        assert_eq!(const_def_assert!((NonCopy,NonCopy)), (NonCopy,NonCopy));
        assert_eq!(const_def_assert!((NonCopy,NonCopy,NonCopy)), (NonCopy,NonCopy,NonCopy));

        assert_eq!(const_def_assert!(Wrapping<NonCopy>).0, NonCopy);
        assert_eq!(const_def_assert!(Reverse<NonCopy>).0, NonCopy);
    }
    #[test]
    fn for_rust_1_24(){
        assert_eq!(const_def_assert!(Cell<Option<()>>).into_inner(), None);
        assert_eq!(const_def_assert!(RefCell<Option<()>>).into_inner(), None);
        assert_eq!(const_def_assert!(UnsafeCell<Option<()>>).into_inner(), None);
    }

    #[test]
    fn for_rust_1_34(){
        assert_eq!(const_def_assert!(atomic::AtomicU8).into_inner(), 0);
        assert_eq!(const_def_assert!(atomic::AtomicI8).into_inner(), 0);
        assert_eq!(const_def_assert!(atomic::AtomicU16).into_inner(), 0);
        assert_eq!(const_def_assert!(atomic::AtomicI16).into_inner(), 0);
    }

    #[test]
    fn for_rust_1_25(){
        use std_::time::Duration;

        assert_eq!(const_def_assert!(Duration).as_secs(), 0);
    }

    #[test]
    fn for_rust_1_26(){
        assert_eq!(const_def_assert!(i128), 0);
        assert_eq!(const_def_assert!(u128), 0);
    }

    #[test]
    fn for_rust_1_32(){
        assert_eq!(const_def_assert!(ManuallyDrop<u8>), ManuallyDrop::new(0));
        assert_eq!(const_def_assert!(ManuallyDrop<bool>), ManuallyDrop::new(false));

    }

    #[test]
    #[cfg(feature = "alloc")]
    fn for_rust_1_39(){
        assert_eq!(const_def_assert!(Vec<u8>), Vec::new());
        assert_eq!(const_def_assert!(Vec<NoDefault>), Vec::new());
        assert_eq!(const_def_assert!(String), String::new());

        assert_eq!(const_def_assert!(LinkedList<u8>), LinkedList::new());
        assert_eq!(const_def_assert!(LinkedList<NoDefault>), LinkedList::new());

        assert_eq!(const_def_assert!(Cow<'_, u8>), Cow::Owned(0u8));
        assert_eq!(const_def_assert!(Cow<'_, String>), Cow::<str>::Owned(String::new()));
        assert_eq!(const_def_assert!(Cow<'_, str>), Cow::<str>::Owned(String::new()));
        assert_eq!(const_def_assert!(Cow<'_, [u8]>), Cow::<[u8]>::Owned(Vec::new()));
        assert_eq!(const_def_assert!(Cow<'_, [NoDefault]>), Cow::<[NoDefault]>::Owned(Vec::new()));
    }

    #[test]
    #[cfg(feature = "rust_1_51")]
    fn for_rust_1_51(){
        // This type must not implement Copy
        #[derive(Debug, PartialEq, Eq)]
        struct F(u32);

        impl ConstDefault for F {
            const DEFAULT: F = F(34);
        }

        let mf = ||F(34);
        let arr33 = [
            mf(), mf(), mf(), mf(), mf(), mf(), mf(), mf(), 
            mf(), mf(), mf(), mf(), mf(), mf(), mf(), mf(),
            mf(), mf(), mf(), mf(), mf(), mf(), mf(), mf(),
            mf(), mf(), mf(), mf(), mf(), mf(), mf(), mf(),
            mf(),
        ];
        let arr65 = [
            mf(), mf(), mf(), mf(), mf(), mf(), mf(), mf(), 
            mf(), mf(), mf(), mf(), mf(), mf(), mf(), mf(),
            mf(), mf(), mf(), mf(), mf(), mf(), mf(), mf(),
            mf(), mf(), mf(), mf(), mf(), mf(), mf(), mf(),
            mf(), mf(), mf(), mf(), mf(), mf(), mf(), mf(), 
            mf(), mf(), mf(), mf(), mf(), mf(), mf(), mf(),
            mf(), mf(), mf(), mf(), mf(), mf(), mf(), mf(),
            mf(), mf(), mf(), mf(), mf(), mf(), mf(), mf(),
            mf(),
        ];

        assert_eq!(const_def_assert!([F; 33]), arr33);
        assert_eq!(const_def_assert!([F; 65]), arr65);
        assert_eq!(const_def_assert!([u32; 33]), [0; 33]);
        assert_eq!(const_def_assert!([u32; 63]), [0; 63]);
    }

}
