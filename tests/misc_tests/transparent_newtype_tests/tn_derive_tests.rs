use krate::{TransparentNewtype, TransparentNewtypeExt};

use core::marker::PhantomData as PD;

use static_assertions::{
    assert_not_impl_all as assert_not_impl,
    assert_impl_all as assert_impl,
};


#[derive(Debug, PartialEq)]
pub struct Wring<T: ?Sized>(T);


macro_rules! cherrypicked_impls {
    ($($ty:ty),*) => {
        $(
            unsafe impl TransparentNewtype for Wring<$ty> {
                type Inner = $ty;

                krate::impl_transparent_newtype!{Self}
            }
        )*
    };
}
cherrypicked_impls!{isize, usize, i8, u8, i16, u16, i32, u32, i64, u64}


mod single {
    use super::*;

    #[derive(Debug, PartialEq, TransparentNewtype)]
    #[twrap(crate = krate)]
    #[repr(transparent)]
    pub(super) struct W<T: ?Sized>(pub(super) T);
}

#[test]
fn test_single() {
    use self::single::W;

    {
        assert_eq!(W::from_inner(3u8), W(3u8));
        assert_eq!(W::from_inner_ref(&5u16), &W(5u16));
        assert_eq!(W::from_inner_mut(&mut 8u32), &mut W(8u32));

        assert_eq!(W(3u8).into_inner(), 3u8);
        assert_eq!(W(5u8).as_inner(), &5u8);
        assert_eq!(W(8u8).as_inner_mut(), &mut 8u8);
    }
    {
        type WS = W<[u8]>;
        assert_eq!(WS::from_inner_ref(&[5u8]), &W([5u8]) as &WS);
        assert_eq!(WS::from_inner_mut(&mut [8u8]), &mut W([8u8]) as &mut WS);

        assert_eq!((&W([5u8]) as &WS).as_inner(), &[5u8][..]);
        assert_eq!((&mut W([8u8]) as &mut WS).as_inner_mut(), &mut [8u8][..]);
    }
}

mod delegated {
    use super::*;
    #[derive(Debug, PartialEq, TransparentNewtype)]
    #[twrap(crate = krate)]
    #[repr(transparent)]
    pub(super) struct W<T: ?Sized>(
        #[twrap(delegate)]
        pub(super) T,
    );
}

#[test]
fn test_single_delegated() {
    use self::delegated::W;
    assert_eq!(W::<Wring<u8>>::from_inner(3u8), W(Wring(3u8)));
    assert_eq!(W::<Wring<u8>>::from_inner_ref(&5u8), &W(Wring(5u8)));
    assert_eq!(W::<Wring<u8>>::from_inner_mut(&mut 8u8), &mut W(Wring(8u8)));

    assert_eq!(W(Wring(3u8)).into_inner(), 3u8);
    assert_eq!(W(Wring(5u8)).as_inner(), &5u8);
    assert_eq!(W(Wring(8u8)).as_inner_mut(), &mut 8u8);
}

mod two_deleg_ty_param {
    use super::*;
    #[derive(Debug, PartialEq, TransparentNewtype)]
    #[twrap(crate = krate)]
    #[repr(transparent)]
    pub(super) struct W<T: ?Sized>(
        pub PD<T>,
        #[twrap(delegate)]
        pub T,
    );
}

#[test]
fn test_two_deleg_ty_param() {
    use self::two_deleg_ty_param::W;

    assert_not_impl!(W<u8>: TransparentNewtype);

    assert_eq!(W::<Wring<u8>>::from_inner(3u8), W(PD, Wring(3u8)));
    assert_eq!(W::<Wring<u8>>::from_inner_ref(&5u8), &W(PD, Wring(5u8)));
    assert_eq!(W::<Wring<u8>>::from_inner_mut(&mut 8u8), &mut W(PD, Wring(8u8)));

    assert_eq!(W(PD, Wring(3u8)).into_inner(), 3u8);
    assert_eq!(W(PD, Wring(5u8)).as_inner(), &5u8);
    assert_eq!(W(PD, Wring(8u8)).as_inner_mut(), &mut 8u8);
}


mod two_deleg_generic {
    use super::*;
    #[derive(Debug, PartialEq, TransparentNewtype)]
    #[twrap(crate = krate)]
    #[repr(transparent)]
    pub(super) struct W<T: ?Sized>(
        pub(super) PD<T>,
        #[twrap(delegate)]
        pub(super) Wring<T>,
    );
}

#[test]
fn test_two_deleg_generic() {
    use self::two_deleg_generic::W;

    assert_not_impl!(W<Wring<String>>: TransparentNewtype);
    assert_not_impl!(W<String>: TransparentNewtype);
    assert_not_impl!(W<&str>: TransparentNewtype);
    assert_not_impl!(W<bool>: TransparentNewtype);
    
    assert_impl!(W<i64>: TransparentNewtype);

    assert_eq!(W::<u8>::from_inner(3u8), W(PD, Wring(3u8)));
    assert_eq!(W::<u16>::from_inner_ref(&5u16), &W(PD, Wring(5u16)));
    assert_eq!(W::<u32>::from_inner_mut(&mut 8u32), &mut W(PD, Wring(8u32)));

    assert_eq!(W(PD, Wring(3u64)).into_inner(), 3u64);
    assert_eq!(W(PD, Wring(5i8)).as_inner(), &5i8);
    assert_eq!(W(PD, Wring(8i16)).as_inner_mut(), &mut 8i16);
}


mod two_deleg_concrete {
    use super::*;
 
    #[derive(Debug, PartialEq, TransparentNewtype)]
    #[twrap(crate = krate)]
    #[repr(transparent)]
    pub(super) struct W<T: ?Sized>(
        pub(super) PD<T>,
        #[twrap(delegate)]
        pub(super) Wring<u8>,
    );
}

#[test]
fn test_two_deleg_concrete() {
    use self::two_deleg_concrete::W;

    assert_eq!(W::<u16>::from_inner(3u8), W(PD, Wring(3u8)));
    assert_eq!(W::<u32>::from_inner_ref(&5u8), &W(PD, Wring(5u8)));
    assert_eq!(W::<i8>::from_inner_mut(&mut 8u8), &mut W(PD, Wring(8u8)));

    assert_eq!(W(PD::<i16>, Wring(3u8)).into_inner(), 3u8);
    assert_eq!(W(PD::<i32>, Wring(5u8)).as_inner(), &5u8);
    assert_eq!(W(PD::<String>, Wring(8u8)).as_inner_mut(), &mut 8u8);
}


mod constrained {
    use super::*;
 
    #[derive(Debug, PartialEq, TransparentNewtype)]
    #[twrap(crate = krate)]
    #[twrap(where T: Copy)]
    #[repr(transparent)]
    pub(super) struct W<T>(
        pub(super) T,
    );
}

#[test]
fn test_constrained() {
    use self::constrained::W;

    assert_not_impl!(W<String>: TransparentNewtype);

    assert_impl!(W<&str>: TransparentNewtype<Inner = &'static str>);

    assert_eq!(W::from_inner(3u8), W(3u8));
    assert_eq!(W::from_inner_ref(&5u16), &W(5u16));
    assert_eq!(W::from_inner_mut(&mut 8u32), &mut W(8u32));

    assert_eq!(W(3i8).into_inner(), 3i8);
    assert_eq!(W(5i16).as_inner(), &5i16);
    assert_eq!(W(8i32).as_inner_mut(), &mut 8i32);
}


