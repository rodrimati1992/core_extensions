use krate::ConstDefault;

use core::marker::PhantomData as PD;

use static_assertions::{
    assert_not_impl_all as assert_not_impl,
    assert_impl_all as assert_impl,
};


#[derive(Debug, PartialEq, ConstDefault)]
#[cdef(crate = krate)]
struct Regular<T>(T);

#[derive(Debug, PartialEq)]
struct NoDef;

#[derive(Debug, PartialEq, ConstDefault)]
#[cdef(crate = krate)]
struct Clonab<T>(T);

#[derive(Debug, Clone, PartialEq, ConstDefault)]
#[cdef(crate = krate)]
struct Ztring(&'static str);

#[derive(Debug, PartialEq)]
pub struct CherryPick<T>(T);


macro_rules! cherrypicked_impls {
    ($($ty:ty),*) => {
        $(
            impl ConstDefault for CherryPick<$ty> {
                const DEFAULT: Self = Self(<$ty>::DEFAULT);
            }
        )*
    };
}
cherrypicked_impls!{u32, Ztring, u64}

mod bounds {
    use super::*;

    #[derive(Debug, PartialEq, ConstDefault)]
    #[cdef(bound(T: Copy))]
    #[cdef(crate = krate)]
    pub struct With<T>(pub PD<T>);

    #[derive(Debug, PartialEq, ConstDefault)]
    #[cdef(bound(T: ))]
    #[cdef(crate = krate)]
    pub struct Without<T>(pub PD<T>);
}
    

#[test]
fn test_bound_attr(){
    assert_not_impl!(bounds::With<Ztring>: ConstDefault);

    assert_impl!(bounds::With<u32>: ConstDefault);
    assert_impl!(Regular<Ztring>: ConstDefault);

    assert_eq!(bounds::With::<u32>::DEFAULT, bounds::With(PD));
    assert_eq!(Regular::<Ztring>::DEFAULT, Regular(Ztring("")));
}

mod nb {
    use super::*;

    #[derive(Debug, PartialEq, ConstDefault)]
    #[cdef(crate = krate)]
    #[cdef(no_bounds)]
    pub struct None<T, U>(pub PD<T>, pub PD<U>);
    
    #[derive(Debug, PartialEq, ConstDefault)]
    #[cdef(crate = krate)]
    #[cdef(no_bounds)]
    pub struct FB0<T, U>(
        #[cdef(field_bound)]
        pub CherryPick<T>,
        pub PD<U>,
    );

    #[derive(Debug, PartialEq, ConstDefault)]
    #[cdef(crate = krate)]
    #[cdef(no_bounds)]
    pub struct FB1<T, U>(
        pub PD<T>,
        #[cdef(field_bound)] 
        pub CherryPick<U>,
    );
}


#[test]
fn test_nobounds_attr(){
    assert_not_impl!(nb::FB0<NoDef, NoDef>: ConstDefault);
    assert_not_impl!(nb::FB1<NoDef, NoDef>: ConstDefault);

    assert_impl!(nb::None<u32, u32>: ConstDefault);
    assert_impl!(nb::None<NoDef, NoDef>: ConstDefault);
    assert_impl!(nb::FB0<u32, NoDef>: ConstDefault);
    assert_impl!(nb::FB1<NoDef, u32>: ConstDefault);
    
    assert_eq!(nb::None::<u32, u32>::DEFAULT, nb::None(PD, PD));
    assert_eq!(nb::None::<NoDef, NoDef>::DEFAULT, nb::None(PD, PD));
    assert_eq!(nb::FB0::<u32, NoDef>::DEFAULT, nb::FB0(CherryPick(0), PD));
    assert_eq!(nb::FB1::<NoDef, u32>::DEFAULT, nb::FB1(PD, CherryPick(0)));
}


mod fb {
    use super::*;

    #[derive(Debug, PartialEq, ConstDefault)]
    #[cdef(crate = krate)]
    #[cdef(field_bound)]
    pub struct FB<T, U>(
        pub PD<T>,
        pub CherryPick<U>,
    );
}

#[test]
fn test_container_field_bound_attr(){
    assert_not_impl!(fb::FB<NoDef, NoDef>: ConstDefault);

    assert_impl!(fb::FB<u32, u32>: ConstDefault);
    assert_impl!(fb::FB<NoDef, u32>: ConstDefault);
    assert_impl!(fb::FB<NoDef, Ztring>: ConstDefault);

    assert_eq!(fb::FB::<u32, u32>::DEFAULT, fb::FB(PD, CherryPick(0)));
    assert_eq!(fb::FB::<NoDef, u32>::DEFAULT, fb::FB(PD, CherryPick(0)));
    assert_eq!(fb::FB::<NoDef, Ztring>::DEFAULT, fb::FB(PD, CherryPick(Ztring(""))));
}


struct WCA(u8);
mod wc {
    use super::*;

    #[derive(Debug, PartialEq, ConstDefault)]
    #[cdef(crate = krate)]
    #[cdef(
        where
            T: Copy,
            [(); {
                impl ConstDefault for WCA {
                    const DEFAULT: Self = Self(3);
                }
                0
            }]:
    )]
    pub struct WC<T>(pub T);
}

#[test]
fn test_where_clause_attr(){
    assert_not_impl!(wc::WC<Clonab<u32>>: ConstDefault);
    assert_not_impl!(wc::WC<Ztring>: ConstDefault);

    assert_impl!(wc::WC<u32>: ConstDefault);
    assert_impl!(wc::WC<()>: ConstDefault);

    assert_eq!(wc::WC::<u32>::DEFAULT, wc::WC(0));
    assert_eq!(wc::WC::<()>::DEFAULT, wc::WC(()));
    assert_eq!(WCA::DEFAULT.0, 3);
}


mod defs {
    use super::*;

    #[derive(Debug, PartialEq, ConstDefault)]
    #[cdef(crate = krate)]
    pub enum Enum {
        #[cdef(default)]
        Foo {
            bar: u8,
            #[cdef(default = 2u32.pow(3))]
            baz: u32,
        },
        Bar,
    }

    #[derive(Debug, PartialEq, ConstDefault)]
    #[cdef(crate = krate)]
    pub struct Struc {
        #[cdef(default = {
            let x = 5u8;
            (x.pow(2) + x) / 2
        })]
        pub bar: u8,
        pub baz: u32,
    }
}

#[test]
fn test_default_attr(){
    assert_eq!(defs::Enum::DEFAULT, defs::Enum::Foo{bar: 0, baz: 8});
    assert_eq!(defs::Struc::DEFAULT, defs::Struc{bar: 15, baz: 0});
}



