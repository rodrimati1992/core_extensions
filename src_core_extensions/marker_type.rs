//! Contains marker traits representing a variety of guarantees provided by the implementors.
//!
//!
//!

use std_::marker::PhantomData;

use std_::mem::ManuallyDrop;

#[allow(dead_code)]
union __Uninit<T: Copy> {
    uninit: (),
    value: T,
}

/**
Represents a zero-sized marker type .

Types implementing this trait are zero-sized and can safely be stored in any
`#[repr(C)]` type without changing their layout.

# Safety

Implementors of this trait must ensure:

- That the type is zero-sized,

- That it has an alignment of 1.

- That the type is trivially constructible, eg: by implementing [`Default`].

The easiest way to enforce the requirements of being zero-sized and
having an alignment of 1 is to have structs composed entirely of MarkerType fields
(ie: `PhantomData`).

# Built-in impls

This trait is not implemented for arrays because it's not yet clear
what the behavior of `#[repr(C)]` types will be with zero-sized arrays.
The ["repr(C) is unsound on MSVC targets" issue](https://github.com/rust-lang/rust/issues/81996)
could possibly require zero-length arrays(or `#[repr(C)]` structs)
in `#[repr(C)]` types not being zero sized on MSVC.

[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
*/
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "marker_type")))]
pub unsafe trait MarkerType: Copy + Sized {
    /// The value of Self.
    #[allow(const_err)]
    const MTVAL: Self = {
        // this is safe since implementing MarkerType guarantees that
        // this type is a Zero Sized Type ,which can't be uninitialized.
        unsafe { (__Uninit::<Self> { uninit: () }).value }
    };

    #[inline(always)]
    #[allow(const_err)]
    /// Constructs a reference to Self.
    fn markertype_ref<'a>() -> &'a Self
    where
        Self: 'a,
    {
        unsafe {
            &*std_::ptr::NonNull::<Self>::dangling().as_ptr()
        }
    }

    /// Constructs Self,this is possible because Self implements MarkerType.
    #[inline(always)]
    fn markertype_val() -> Self {
        Self::MTVAL
    }
}

#[doc(hidden)]
#[inline(always)]
pub fn assert_markertype<T: MarkerType>(){}

unsafe impl<T: ?Sized> MarkerType for PhantomData<T> {}

unsafe impl<T> MarkerType for ManuallyDrop<T> 
where
    T: MarkerType
{}

/*
// Uncomment once the rules around zero sized types in `#[repr(C)]` types are figured out,
// and it treats these types as zero sized.
// Then uncomment all `Uncomment` comments
// 
// https://github.com/rust-lang/rust/issues/81996

unsafe impl MarkerType for () {}

////////////////////////////////

#[cfg(feature = "rust_1_51")]
macro_rules! impl_zero_sized_array {
    ()=>{
        /// When the "const_params" feature is disabled,
        /// the MarkerType trait is implemented for arrays up to 32 elements long.
        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_params")))]
        unsafe impl<T, const N: usize> MarkerType for [T; N]
        where T: MarkerType
        {}
    }
}

#[cfg(feature = "rust_1_51")]
impl_zero_sized_array!{}

///////////////////////////////////

#[cfg(not(feature = "rust_1_51"))]
macro_rules! impl_zero_sized_array {
    ($($size:expr),*)=>{
        $(
            unsafe impl<T> MarkerType for [T;$size]
            where T:MarkerType
            {}
        )*
    }
}

#[cfg(not(feature = "rust_1_51"))]
impl_zero_sized_array! {
    00,01,02,03,04,05,06,07,08,09,
    10,11,12,13,14,15,16,17,18,19,
    20,21,22,23,24,25,26,27,28,29,
    30,31,32
}

////////////////////////////////

macro_rules! impl_zero_sized_tuple {
    ($($ty:ident),+) => (
        unsafe impl<$($ty),*> MarkerType for ($($ty,)*)
        where $($ty:MarkerType,)*
        {}
    )
}

impl_zero_sized_tuple! {A}
impl_zero_sized_tuple! {A,B}
impl_zero_sized_tuple! {A,B,C}
impl_zero_sized_tuple! {A,B,C,D}
impl_zero_sized_tuple! {A,B,C,D,E}
impl_zero_sized_tuple! {A,B,C,D,E,F}
impl_zero_sized_tuple! {A,B,C,D,E,F,G}
impl_zero_sized_tuple! {A,B,C,D,E,F,G,H}
impl_zero_sized_tuple! {A,B,C,D,E,F,G,H,I}
impl_zero_sized_tuple! {A,B,C,D,E,F,G,H,I,J}
impl_zero_sized_tuple! {A,B,C,D,E,F,G,H,I,J,K}
impl_zero_sized_tuple! {A,B,C,D,E,F,G,H,I,J,K,L}
impl_zero_sized_tuple! {A,B,C,D,E,F,G,H,I,J,K,L,M}
impl_zero_sized_tuple! {A,B,C,D,E,F,G,H,I,J,K,L,M,N}
impl_zero_sized_tuple! {A,B,C,D,E,F,G,H,I,J,K,L,M,N,O}
impl_zero_sized_tuple! {A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P}
*/

#[cfg(test)]
mod tests {
    use std_::marker::PhantomData;
    use std_::mem::{ManuallyDrop, align_of, size_of};
    macro_rules! assert_size_align {
        ($ty:ty) => {
            assert_eq!(size_of::<$ty>(), 0);
            assert_eq!(align_of::<$ty>(), 1);

            assert_eq!(size_of::<ManuallyDrop<$ty>>(), 0);
            assert_eq!(align_of::<ManuallyDrop<$ty>>(), 1);
        };
    }
    
    type PD = PhantomData<u64>;

    #[test]
    fn test_manuallydrop(){
        assert_size_align!(ManuallyDrop<PD>);
        // Uncomment at the same time as the impls
        // assert_size_align!(ManuallyDrop<(PD,PD)>);
    }

    #[test]
    fn test_alignment_size() {

        assert_size_align!(());
        assert_size_align!(PhantomData<()>);
        assert_size_align!(PhantomData<u64>);

        /*
        // Uncomment at the same time as the impls
        assert_size_align!((PD,));
        assert_size_align!((PD, PD,));
        assert_size_align!((PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD, PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,));
        assert_size_align!((PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,));
        assert_size_align!((
            PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,
        ));
        assert_size_align!((
            PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,
        ));
        assert_size_align!((
            PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,
        ));
        assert_size_align!((
            PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,
        ));
        assert_size_align!((
            PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,
        ));
        assert_size_align!((
            PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,
            PD,
        ));
        assert_size_align!((
            PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD, PD,
            PD, PD,
        ));

        assert_size_align!([PD; 0]);
        assert_size_align!([PD; 1]);
        assert_size_align!([PD; 2]);
        assert_size_align!([PD; 3]);
        assert_size_align!([PD; 4]);
        assert_size_align!([PD; 5]);
        assert_size_align!([PD; 6]);
        assert_size_align!([PD; 7]);
        assert_size_align!([PD; 8]);
        assert_size_align!([PD; 9]);
        assert_size_align!([PD; 10]);
        assert_size_align!([PD; 11]);
        assert_size_align!([PD; 12]);
        assert_size_align!([PD; 13]);
        assert_size_align!([PD; 14]);
        assert_size_align!([PD; 15]);
        assert_size_align!([PD; 16]);
        assert_size_align!([PD; 17]);
        assert_size_align!([PD; 18]);
        assert_size_align!([PD; 19]);
        assert_size_align!([PD; 20]);
        assert_size_align!([PD; 21]);
        assert_size_align!([PD; 22]);
        assert_size_align!([PD; 23]);
        assert_size_align!([PD; 24]);
        assert_size_align!([PD; 25]);
        assert_size_align!([PD; 26]);
        assert_size_align!([PD; 27]);
        assert_size_align!([PD; 28]);
        assert_size_align!([PD; 29]);
        assert_size_align!([PD; 30]);
        assert_size_align!([PD; 31]);
        assert_size_align!([PD; 32]);

        #[cfg(feature = "rust_1_51")]
        assert_size_align!([PD; 63]);

        */
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////
