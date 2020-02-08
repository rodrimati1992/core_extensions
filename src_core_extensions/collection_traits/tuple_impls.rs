#![allow(non_snake_case)]

use super::{
    Cloned,
    IntoArray,
};

macro_rules! impl_tuple {
    (l; $($anything:tt)* )=>{ 1 };
    (a; )=>{};
    (a; $($anything:tt)* )=>{ C0 };
    ( ( $($tup:ident,)* ) ) => (
        impl_tuple!{cloned; all($($tup,)*) }

        impl_tuple!{into_array; all($($tup,)*) }
    );
    (cloned; all($($tup:ident,)*) ) => (
        impl<'a,$($tup),*> Cloned for ($($tup,)*)
        where
            $($tup: Cloned,)*
        {
            type Cloned=($($tup::Cloned,)*);

            fn cloned_(&self)->Self::Cloned {
                let ($($tup,)*)=self;
                (
                    $($tup.cloned_(),)*
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

impl_tuple! {
    ()
}
impl_tuple! {
    (C0,)
}
impl_tuple! {
    (C0,C1,)
}
impl_tuple! {
    (C0,C1,C2,)
}
impl_tuple! {
    (C0,C1,C2,C3,)
}
impl_tuple! {
    (C0,C1,C2,C3,C4,)
}
impl_tuple! {
    (C0,C1,C2,C3,C4,C5,)
}
impl_tuple! {
    (C0,C1,C2,C3,C4,C5,C6,)
}
impl_tuple! {
    (C0,C1,C2,C3,C4,C5,C6,C7,)
}
impl_tuple! {
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,)
}
impl_tuple! {
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,)
}
impl_tuple! {
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,)
}
impl_tuple! {
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cloned_core() {
        assert_eq!((&5,).cloned_(), (5,));
        assert_eq!((&5, &8).cloned_(), (5, 8));
        assert_eq!((&5, &8, &13).cloned_(), (5, 8, 13));
        assert_eq!((&5, &8, &13, &21).cloned_(), (5, 8, 13, 21));
        assert_eq!(
            (&1, &4, &9, &16, &25, &36, &49, &64, &81, &100, &121, &144).cloned_(),
            (1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144)
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn cloned_alloc() {
        use alloc_::string::ToString;

        assert_eq!(("5",).cloned_(), ("5".to_string(),));
        assert_eq!(("5", "8").cloned_(), ("5".to_string(), "8".to_string()));
        assert_eq!(
            ("5", "8", "13").cloned_(),
            ("5".to_string(), "8".to_string(), "13".to_string())
        );
        assert_eq!(
            ("5", "8", "13", "21").cloned_(),
            (
                "5".to_string(),
                "8".to_string(),
                "13".to_string(),
                "21".to_string()
            )
        );
    }

    #[test]
    fn into_array() {
        macro_rules! into_array_tests {
            ( $([$($array:tt)*],)* ) => (
                $({
                    assert_eq!( ($($array)*).into_array(), [$($array)*] );
                })*
            )
        }
        into_array_tests! {
            [5,],
            [5,8],
            [5,8,13],
            [5,8,13,21],
            [5,8,13,21,34],
            [1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144],
        }
    }
}
