//! Contains marker traits representing a variety of guarantees provided by the implementors.
//!
//!
//!

use std_::marker::PhantomData;

#[cfg(rust_1_29)]
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

-that the type is zero-sized,

-that it has an alignment of 1.

-that the type is trivially constructible,eg:by implementing ::std::default::Default.

The easiest way to enforce the requirements of being zero-sized and
having an alignment of 1 is to have structs composed entirely of MarkerType fields (ie:VariantPhantom , PhantomData , ()  ).

*/
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
    /// Constructs a reference to Self,
    /// this is possible because all references to zero sized types are valid.
    fn markertype_ref<'a>() -> &'a Self
    where
        Self: 'a,
    {
        unsafe {
            const SOME_ADDRESS: usize = 1_000_000;
            // this is safe since implementing MarkerType guarantees that
            // this type is a 1-aligned Zero Sized Type ,in which all pointers are valid.
            &*(SOME_ADDRESS as *const Self)
        }
    }

    /// Constructs Self,this is possible because Self implements MarkerType.
    #[inline(always)]
    fn markertype_val() -> Self {
        Self::MTVAL
    }
}

unsafe impl<T: ?Sized> MarkerType for PhantomData<T> {}

#[cfg(rust_1_29)]
unsafe impl<T> MarkerType for ManuallyDrop<T> 
where
    T: ?Sized + MarkerType
{}

unsafe impl MarkerType for () {}

////////////////////////////////

#[cfg(feature = "const_generics")]
macro_rules! impl_zero_sized_array {
    ()=>{
        unsafe impl<T, const N: usize> MarkerType for [T; N]
        where T: MarkerType
        {}
    }
}

#[cfg(feature = "const_generics")]
impl_zero_sized_array!{}

///////////////////////////////////

#[cfg(not(feature = "const_generics"))]
macro_rules! impl_zero_sized_array {
    ($($size:expr),*)=>{
        $(
            unsafe impl<T> MarkerType for [T;$size]
            where T:MarkerType
            {}
        )*
    }
}

#[cfg(not(feature = "const_generics"))]
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

#[cfg(feature = "typenum")]
mod typenum {
    use super::MarkerType;

    use typenum::marker_traits::{Bit, NonZero, Unsigned};
    use typenum::{NInt, PInt, UInt, UTerm, Z0};

    unsafe impl<U: Unsigned + Default + NonZero + Copy> MarkerType for PInt<U> {}
    unsafe impl<U: Unsigned + Default + NonZero + Copy> MarkerType for NInt<U> {}
    unsafe impl MarkerType for Z0 {}
    unsafe impl<U: Unsigned + Default + Copy, B: Bit + Copy + Default> MarkerType for UInt<U, B> {}
    unsafe impl MarkerType for UTerm {}
}

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
    #[cfg(rust_1_29)]
    fn test_manuallydrop(){
        assert_size_align!(ManuallyDrop<PD>);
        assert_size_align!(ManuallyDrop<(PD,PD)>);
    }

    #[test]
    fn test_alignment_size() {

        assert_size_align!(());
        assert_size_align!(PhantomData<()>);
        assert_size_align!(PhantomData<u64>);
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

        #[cfg(feature = "const_generics")]
        assert_size_align!([PD; 63]);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////
