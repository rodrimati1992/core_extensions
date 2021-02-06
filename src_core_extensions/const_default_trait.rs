//! Contains marker traits representing a variety of guarantees provided by the implementors.
//!
//!
//!

use std_::cell::{Cell,RefCell,UnsafeCell};

use std_::cmp::Reverse;

use std_::marker::PhantomData;

use std_::mem::ManuallyDrop;

use std_::num::Wrapping;

use std_::sync::atomic::{AtomicUsize,AtomicIsize,AtomicBool};

#[allow(unused_imports)]
use std_::sync::atomic;

/// A const equivalent of the `Default` trait.
///
/// # Example
///
/// Implementing `ConstDefault` for a struct
///
/// ```rust
/// // use core_extensions::const_default; // in newer versions of Rust.
/// #[macro_use(const_default)]
/// extern crate core_extensions;
///
/// use core_extensions::ConstDefault;
/// 
/// #[derive(Debug,PartialEq)]
/// struct Point<T>{
///     x:T,
///     y:T,
/// }
///
/// // `+ Copy` here is required for Rust 1.20 and 1.21,
/// // and can be removed from Rust 1.22 onwards.
/// impl<T> ConstDefault for Point<T>
/// where
///     T: ConstDefault + Copy
/// {
///     const DEFAULT: Self= Point{ x: T::DEFAULT,  y: T::DEFAULT };
/// }
///
/// # fn main(){
/// assert_eq!( const_default!(Point<u8>), Point{x:0, y:0} );
/// assert_eq!( const_default!(Point<f32>), Point{x:0.0, y:0.0} );
/// assert_eq!( const_default!(Point<Option<()>>), Point{x:None, y:None} );
/// # }
/// ```
pub trait ConstDefault: Sized {
    /// The default value for `Self`.
    const DEFAULT: Self;
}

/// Gets the ConstDefault::DEFAULT associated constant for This.
/// 
/// Use this macro to avoid using the wrong `DEFAULT` associated cosntant,
/// eg: a `DEFAULT` associated constant in an inherent impl block with a
/// subset of the  constraints that the `ConstDefault` impl has.
#[macro_export]
macro_rules! const_default {
    ($This:ty) => {
        <$This as $crate::ConstDefault>::DEFAULT
    };
}

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


impl<T> ConstDefault for [T;0]{
    const DEFAULT: Self=[];
}

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
            for[$($for:tt)*]
            $ty:ty = 
            $def:expr
        ),*
        $(,)*
    )=>{
        $(
            impl<$($for)*> ConstDefault for $ty {
                const DEFAULT: Self= $def;
            }
        )*
    }
}

impl_const_default!{
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
    for['a,T:'a] &'a [T] = &[],
}

impl_const_default!{
    for[T: ConstDefault] Wrapping<T> = Wrapping(T::DEFAULT),
    for[T: ConstDefault] Reverse<T> = Reverse(T::DEFAULT),
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
impl_const_default!{
    for[T] ::alloc_::vec::Vec<T> = Self::new(),
    for[] ::alloc_::string::String = Self::new(),
}


#[cfg(test)]
mod tests{
    use super::*;

    #[derive(Debug,PartialEq)]
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
        assert_eq!(const_def_assert!([NoDefault;0]), []);
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
        use alloc_::vec::Vec;
        use alloc_::string::String;

        assert_eq!(const_def_assert!(Vec<u8>), Vec::new());
        assert_eq!(const_def_assert!(Vec<NoDefault>), Vec::new());
        assert_eq!(const_def_assert!(String), String::new());
    }

}
