use krate::getconst;

use std::marker::PhantomData;


#[derive(Copy, Clone)]
struct Func;

trait GetSize<T> {
    const SIZE: usize;
}

impl<T> GetSize<T> for Func {
    const SIZE: usize = std::mem::size_of::<T>();
}


krate::quasiconst!{
    const LT<'a>: &'a u16 = &1337;
    const LT_COMMA['a,]: &'a u16 = &8337;
    
    const LT_BOUND_BOUND['a: ]: &'a u16 = &8000;
    const LT_TWO_BOUNDS<'a, 'b: 'a>: &'a u16 = {
        let x: &'b u16 = &8007;
        x
    };
}

#[test]
fn test_lifetime() {
    fn get_u16s<'a>(_: &'a u8) -> [&'a u16; 2] {
        [
            getconst!(LT<'a>),
            getconst!(LT_COMMA<'a>),
        ]
    }

    // Putting LT_COMMA_BOUND in the return type so that its implied bounds make `'a == 'static`
    fn get_u16s_static<'a>(_: &'a u8) -> ([&'a u16; 2], LT_BOUND_BOUND<'a>) {
        (
            [
                getconst!(LT_BOUND_BOUND<'a>),
                getconst!(LT_TWO_BOUNDS<'a, '_>),
            ],
            LT_BOUND_BOUND::NEW,
        )
    }

    let local = 0;
    assert_eq!(get_u16s(&local), [&1337, &8337]);
    {
        let arr: [&'static u16; 2] = get_u16s(&0);
        assert_eq!(arr, [&1337, &8337]);
    }
    {
        let arr: [&u16; 2] = get_u16s_static(&0).0;
        assert_eq!(arr, [&8000, &8007]);
    }
}



krate::quasiconst!{
    const FIRST_BOUND<T: 'static + GetSize<u8> + Sized + Clone>: usize =
        <T as GetSize<u8>>::SIZE;

    const LAST_BOUND[T: 'static + Sized + Clone + GetSize<u16>]: usize =
        <T as GetSize<u16>>::SIZE;

    const PAREN_BOUND[T: 'static + (Sized) + (Clone) + GetSize<u32>]: usize =
        <T as GetSize<u32>>::SIZE;
    
    const WITH_LIFETIME<'a, T: 'static + Sized, U = u16, V = u32>: () = ();

}

const _: WITH_LIFETIME<'_, u8> = <WITH_LIFETIME<'_, _>>::NEW;
const _: WITH_LIFETIME<'_, (), u16, u32> = <WITH_LIFETIME<'_, _>>::NEW;

#[test]
fn bounds_test() {
    assert_eq!(getconst!(FIRST_BOUND<Func>), 1);
    assert_eq!(getconst!(LAST_BOUND<Func>), 2);
    assert_eq!(getconst!(PAREN_BOUND<Func>), 4);
}


mod module {
    pub mod submod {
        krate::quasiconst!{
            pub const PRIVACY: usize = 3;
            pub(in super::super) const PUBSUPER: usize = 8;
        }
    }
    krate::quasiconst!{
        const PRIVACY: usize = 5;
        pub(self) const PUBSUPER: usize = 13;
    }
}

use self::module::*;
use self::module::submod::*;

// Making sure that module::PRIVACY is actually private
#[test]
fn privacy_test() {
    assert_eq!(getconst!(PRIVACY), 3);
    assert_eq!(getconst!(PUBSUPER), 8);
}


   
krate::quasiconst!{
    const NO_BOUND_DEF<T = u8>: (PhantomData<T>, usize) =
        (PhantomData, std::mem::size_of::<T>());
    
    const BOUND_DEF[T: Copy + std::fmt::Debug = u8]: (PhantomData<T>, usize) = {
        fn bounded<T: Copy + std::fmt::Debug>(){}
        bounded::<T>;

        (PhantomData, std::mem::size_of::<T>() * 2)
    };

    const BOUND_DEF_COMMA[T: Copy + std::fmt::Debug = u8,]: (PhantomData<T>, usize) = {
        getconst!(BOUND_DEF<..>)
    };
}

const _: NO_BOUND_DEF<u16> = <NO_BOUND_DEF<u16>>::NEW;
const _: NO_BOUND_DEF<u8> = <NO_BOUND_DEF>::NEW;
const _: NO_BOUND_DEF = <NO_BOUND_DEF>::NEW;

#[test]
fn defaulted_test() {
    assert_eq!(getconst!(NO_BOUND_DEF), (PhantomData, 1));
    assert_eq!(getconst!(NO_BOUND_DEF<u16>), (PhantomData, 2));

    assert_eq!(getconst!(NO_BOUND_DEF<..>), (PhantomData::<u8>, 1));
    assert_eq!(getconst!(NO_BOUND_DEF<..>), (PhantomData::<u16>, 2));


    assert_eq!(getconst!(BOUND_DEF), (PhantomData, 2));
    assert_eq!(getconst!(BOUND_DEF<u16>), (PhantomData, 4));

    assert_eq!(getconst!(BOUND_DEF<..>), (PhantomData::<u8>, 2));
    assert_eq!(getconst!(BOUND_DEF<..>), (PhantomData::<u16>, 4));


    assert_eq!(getconst!(BOUND_DEF_COMMA), (PhantomData, 2));
    assert_eq!(getconst!(BOUND_DEF_COMMA<u16>), (PhantomData, 4));

    assert_eq!(getconst!(BOUND_DEF_COMMA<..>), (PhantomData::<u8>, 2));
    assert_eq!(getconst!(BOUND_DEF_COMMA<..>), (PhantomData::<u16>, 4));


}


krate::quasiconst!{
    const WITH_WHERE<T>: usize 
    where
        T: GetSize<u32>, T: GetSize<u64>
    = <T as GetSize<u32>>::SIZE;

    const WITH_TRIVIAL_WHERE: u32 
    where [
        u32: Copy,
        u64: Clone,
    ] = 100;
}


#[test]
fn with_where_clause() {
    assert_eq!(getconst!(WITH_WHERE<Func>), 4);
    assert_eq!(getconst!(WITH_TRIVIAL_WHERE), 100);
}


krate::quasiconst!{
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]    
    const WITH_DERIVE: u32 = 2000;
}

#[test]
fn with_attributes() {
    assert_eq!(WITH_DERIVE::NEW, WITH_DERIVE::NEW);
}




#[test]
fn test_nested_getconst() {
    let (PhantomData::<u8> , a) = getconst!(::misc_tests::quasiconst_tests::nested::NESTED<..>);
    assert_eq!(a, 1);

    let (PhantomData::<u16>, b) = getconst!(self::nested::NESTED<..>);
    assert_eq!(b, 2);

    let (PhantomData::<u32>, c) = getconst!(nested::NESTED<..>);
    assert_eq!(c, 4);
}

pub mod nested {
    use std::marker::PhantomData;

    krate::quasiconst!{
        pub const NESTED<T>: (PhantomData<T>, usize) = (PhantomData, std::mem::size_of::<T>());
    }    
}


#[cfg(feature = "rust_1_59")]
mod rust_1_59 {
    use core::marker::PhantomData;

    use krate::{getconst, quasiconst};

    quasiconst!{
        const WITH_DEF<const F: u32 = 10>: u32 = F;
        const GENS_ORDER<const F: usize, T>: (usize, PhantomData<T>) = (F, PhantomData::<T>);
    }

    #[test]
    fn test_defaulted_const() {
        assert_eq!(getconst!(WITH_DEF), 10);
        assert_eq!(getconst!(WITH_DEF<20>), 20);
    }

    #[test]
    fn test_const_before_type_param() {
        assert_eq!(getconst!(GENS_ORDER<15, Vec<u32>>), (15, PhantomData::<Vec<u32>>));
    }
}





