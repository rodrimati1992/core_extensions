use core_extensions::{getconst, ConstVal};



#[derive(Copy, Clone)]
struct Func;

trait GetSize<T> {
    const SIZE: usize;
}

impl<T> GetSize<T> for Func {
    const SIZE: usize = std::mem::size_of::<T>();
}


core_extensions::quasiconst!{
    const FIRST_BOUND[T: 'static + GetSize<u8> + Sized + Clone]: usize = <T as GetSize<u8>>::SIZE;
    const LAST_BOUND[T: 'static + Sized + Clone + GetSize<u16>]: usize = <T as GetSize<u16>>::SIZE;
    const PAREN_BOUND[T: 'static + (Sized) + (Clone) + GetSize<u32>]: usize = <T as GetSize<u32>>::SIZE;
    
    const WITH_LIFETIME['a, T: 'static + Sized]: () = ();
}

const _: [WITH_LIFETIME<'_, u8>; 0] = [];

#[test]
fn bounds_test() {
    assert_eq!(getconst!(FIRST_BOUND<Func>), 1);
    assert_eq!(getconst!(LAST_BOUND<Func>), 2);
    assert_eq!(getconst!(PAREN_BOUND<Func>), 4);
}


mod module {
    pub mod submod {
        core_extensions::quasiconst!{
            pub const PRIVACY: usize = 3;
            pub(in super::super) const PUBSUPER: usize = 8;
        }
    }
    core_extensions::quasiconst!{
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

