#![allow(non_snake_case)]

use super::{Cloned,IntoArray};
use super::cloned_items::*;

macro_rules! impl_tuple {
    (l; $($anything:tt)* )=>{ 1 };
    (a; )=>{};
    (a; $($anything:tt)* )=>{ C0 };
    ( ( $($tup:ident,)* ) ) => (
        impl_tuple!{cloned; all($($tup,)*) }

        impl_tuple!{into_array; all($($tup,)*) }
    );
    (cloned; all($($tup:ident,)*) ) => (
        impl<'a,$($tup),*> Cloned for ($(&'a $tup,)*) 
        where
            $($tup: ?Sized + UsedCloneTrait,)*
        {
            type Cloned=($(ClonedType<$tup>,)*);

            fn cloned_(&self)->Self::Cloned {
                let ($($tup,)*)=*self;
                (
                    $(clone_this($tup),)*
                )
            }
        }
    );
    (into_array; all() ) => ();
    (into_array; all($($tup:ident,)+) ) => (
        impl<C0> IntoArray for ($( impl_tuple!(a;$tup) ,)*) {
            type Array=[C0; $( impl_tuple!(l;$tup)+ )* 0];

            #[inline(always)]
            fn into_array(self)->Self::Array{
                let ($($tup,)*)=self;

                [$($tup,)*]
            }
        }
    );
}

impl_tuple!{
    ()
}
impl_tuple!{
    (C0,)
}
impl_tuple!{
    (C0,C1,)
}
impl_tuple!{
    (C0,C1,C2,)
}
impl_tuple!{
    (C0,C1,C2,C3,)
}
impl_tuple!{
    (C0,C1,C2,C3,C4,)
}
impl_tuple!{
    (C0,C1,C2,C3,C4,C5,)
}
impl_tuple!{
    (C0,C1,C2,C3,C4,C5,C6,)
}
impl_tuple!{
    (C0,C1,C2,C3,C4,C5,C6,C7,)
}
impl_tuple!{
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,)
}
impl_tuple!{
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,)
}
impl_tuple!{
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,)
}
impl_tuple!{
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,)
}
