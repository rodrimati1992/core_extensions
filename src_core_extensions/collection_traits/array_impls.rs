use super::{
    Cloned,
    IntoArray,
};



#[cfg(feature = "const_generics")]
macro_rules! array_impls {
    ()=>{
        use std_::mem::MaybeUninit;
        use ::utils::RunOnDrop;

        /// When the "const_params" feature is disabled,
        /// the Cloned trait is implemented for arrays up to 32 elements long.
        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_params")))]
        impl<'a, T, const N: usize> Cloned for [T; N]
        where
            T: Cloned
        {
            type Cloned=[T::Cloned; N];

            fn cloned_(&self) -> [T::Cloned; N] {

                struct Written<T, const N: usize> {
                    array: [MaybeUninit<T>; N],
                    written: usize,
                }
                let mut guard = {
                    let out = Written::<T::Cloned, N>{
                        array: unsafe{ MaybeUninit::uninit().assume_init() },
                        written: 0,
                    };
                    RunOnDrop::new(out, |mut out|{
                        let start: *mut MaybeUninit<T::Cloned> = out.array.as_mut_ptr();
                        let slice = std_::ptr::slice_from_raw_parts_mut(
                            start as *mut T::Cloned,
                            out.written,
                        );
                        unsafe{
                            std_::ptr::drop_in_place(slice);
                        }
                    })                    
                };

                let out = guard.get_mut();
                for (i, elem) in self.iter().enumerate() {
                    out.array[i] = MaybeUninit::new(elem.cloned_());
                    out.written += 1;
                }

                // Can't use transmute with generic types
                unsafe{
                    ::utils::transmute_ignore_size::<[MaybeUninit<T::Cloned>; N], [T::Cloned; N]>(
                        guard.into_inner().array
                    )
                }
            }
        }

        impl<T, const N: usize> IntoArray for [T; N] {
            type Array = Self;

            fn into_array(self)->Self {
                self
            }
        }
    }
}

#[cfg(feature = "const_generics")]
array_impls!{}


/////////////////////////////////////////////////

#[cfg(not(feature = "const_generics"))]
macro_rules! array_impls {
    (
        $( ( $size:expr,[$($elem:expr,)*] ) )*
    ) => (
        $(
            impl<'a,T> Cloned for [T;$size]
            where
                T: Cloned
            {
                type Cloned=[T::Cloned;$size];

                fn cloned_(&self)->[T::Cloned;$size] {
                    [
                        $(self[$elem].cloned_(),)*
                    ]
                }
            }

            impl<T> IntoArray for [T;$size] {
                type Array=Self;

                fn into_array(self)->Self {
                    self
                }
            }
        )*
    )
}

/*

fn main() {
    let split_on=20;
    for i in 0..=32{
        print!("({0},[",i);
        let is_split=i >= split_on;
        for j in 0..i {
            if j%split_on==0 && is_split {
                print!("\n    ");
            }
            print!("{0},",j);
        }
        if is_split { println!() }
        println!("])");
    }
}

*/

#[cfg(not(feature = "const_generics"))]
array_impls! {
    (0,[])
    (1,[0,])
    (2,[0,1,])
    (3,[0,1,2,])
    (4,[0,1,2,3,])
    (5,[0,1,2,3,4,])
    (6,[0,1,2,3,4,5,])
    (7,[0,1,2,3,4,5,6,])
    (8,[0,1,2,3,4,5,6,7,])
    (9,[0,1,2,3,4,5,6,7,8,])
    (10,[0,1,2,3,4,5,6,7,8,9,])
    (11,[0,1,2,3,4,5,6,7,8,9,10,])
    (12,[0,1,2,3,4,5,6,7,8,9,10,11,])
    (13,[0,1,2,3,4,5,6,7,8,9,10,11,12,])
    (14,[0,1,2,3,4,5,6,7,8,9,10,11,12,13,])
    (15,[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,])
    (16,[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,])
    (17,[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,])
    (18,[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,])
    (19,[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,])
    (20,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
    ])
    (21,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
        20,
    ])
    (22,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
        20,21,
    ])
    (23,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
        20,21,22,
    ])
    (24,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
        20,21,22,23,
    ])
    (25,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
        20,21,22,23,24,
    ])
    (26,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
        20,21,22,23,24,25,
    ])
    (27,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
        20,21,22,23,24,25,26,
    ])
    (28,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
        20,21,22,23,24,25,26,27,
    ])
    (29,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
        20,21,22,23,24,25,26,27,28,
    ])
    (30,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
        20,21,22,23,24,25,26,27,28,29,
    ])
    (31,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
        20,21,22,23,24,25,26,27,28,29,30,
    ])
    (32,[
        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,
        20,21,22,23,24,25,26,27,28,29,30,31,
    ])

}

/////////////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;

    use ::test_utils::WithVal;

    #[test]
    fn cloned_core() {
        assert_eq!([&5].cloned_(), [5]);
        assert_eq!([&5, &8].cloned_(), [5, 8]);
        assert_eq!([&5, &8, &13].cloned_(), [5, 8, 13]);
        assert_eq!([&5, &8, &13, &21].cloned_(), [5, 8, 13, 21]);
        assert_eq!(
            [&1, &4, &9, &16, &25, &36, &49, &64, &81, &100, &121, &144].cloned_(),
            [1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144]
        );

        assert_eq!(
            [Some(&1), Some(&4), Some(&9)].cloned_(),
            [Some(1), Some(4), Some(9)]
        );

        assert_eq!(
            [Some((&3, &5)), Some((&8, &13))].cloned_(),
            [Some((3, 5)), Some((8, 13))]
        );
    }

    #[test]
    fn cloned_dest() {
        use std_::cell::Cell;
        use test_utils::DecOnDrop;

        let count = Cell::new(10);
        
        let make = |x: u32| WithVal(x, DecOnDrop::new(&count));

        {
            let arr = [make(3), make(4), make(5)];
            let refs = [&arr[0], &arr[1], &arr[2]];
            {
                let clone = refs.cloned_();

                assert_eq!(count.get(), 10);
                assert_eq!(clone, [make(3), make(4), make(5)]);
                assert_eq!(count.get(), 7);
            }
            assert_eq!(count.get(), 4);
        }

        assert_eq!(count.get(), 1);
    }


    #[test]
    #[cfg(feature = "std")]
    fn cloned_panic() {
        use std_::panic::AssertUnwindSafe;

        use test_utils::{CloneLimit, MaxClones};

        let limit = CloneLimit::new(2);
        assert_eq!(limit.clone_count(), 0);
        
        let make = |x: u32| MaxClones::new(x, &limit);

        {
            let arr = [make(3), make(4), make(5)];
            let refs = [&arr[0], &arr[1], &arr[2]];
            assert_eq!(limit.clone_count(), 0);
            assert_eq!(limit.drop_count(), 0);
            let _ = ::std_::panic::catch_unwind(AssertUnwindSafe(||{
                let _ = refs.cloned_();
            })).unwrap_err();
            assert_eq!(limit.clone_count(), 2);
            assert_eq!(limit.drop_count(), 2);
        }
        assert_eq!(limit.clone_count(), 2);
        assert_eq!(limit.drop_count(), 5);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn cloned_alloc() {
        use alloc_::string::ToString;

        assert_eq!(["5"].cloned_(), ["5".to_string()]);
        assert_eq!(["5", "8"].cloned_(), ["5".to_string(), "8".to_string()]);
        assert_eq!(
            ["5", "8", "13"].cloned_(),
            ["5".to_string(), "8".to_string(), "13".to_string()]
        );
        assert_eq!(
            ["5", "8", "13", "21"].cloned_(),
            [
                "5".to_string(),
                "8".to_string(),
                "13".to_string(),
                "21".to_string()
            ]
        );

        #[cfg(feature = "const_generics")]
        {
            use alloc_::string::String;
            use alloc_::vec::Vec;

            use std_::convert::TryInto;

            const LEN: usize = 65;
            
            let owned: Vec<String> = (0..LEN).map(|x| x.to_string()).collect();
            let owned: [String; LEN] = owned.clone().try_into().unwrap();
            
            let borrowed: Vec<&str> = owned.iter().map(|x|x.as_str()).collect();
            let borrowed: [&str; LEN] = borrowed.try_into().unwrap();

            assert_eq!(borrowed.cloned_(), owned);
        }
    }

    #[test]
    fn into_array() {
        macro_rules! into_array_tests {
            ( $($array:expr,)* ) => (
                $({
                    let array=$array;
                    assert_eq!( array.clone().into_array(), array );
                })*
            )
        }
        into_array_tests! {
            [0],
            [0,1],
            [0;2],
            [0;3],
            [0;16],
            [0;32],
        }

        #[cfg(feature = "const_generics")]
        into_array_tests! {
            [0;33],
            [0;65],
        }
    }
}
