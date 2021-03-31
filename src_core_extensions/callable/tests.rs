use crate::{
    impl_call,
    CallExt, CallRef, CallMut, CallInto,
};

use std_::{
    cmp::PartialEq,
    marker::PhantomData,
};

#[cfg(feature = "alloc")]
use alloc_::string::{String,ToString};

#[test]
#[cfg(feature = "alloc")]
fn test_ref_call() {
    struct WhatRef<T>(T);

    impl_call! {
        fn ref_call['a,T,U](self:WhatRef<T>, what:U )->bool
        where [ T:PartialEq<U>, ]
        {
            self.0==what
        }
    }

    let env = WhatRef("hello".to_string());
    assert_eq!(env.ref_call("hello"), true);
    assert_eq!(env.ref_call("hello".to_string()), true);
    assert_eq!(env.ref_call("lo"), false);
}

#[test]
fn test_mut_call() {
    struct WhatMut {
        state: usize,
    }

    impl_call! {
        fn mut_call(self:WhatMut)->usize{
            self.state+=1;
            self.state
        }
    }

    let mut env = WhatMut { state: 0 };
    assert_eq!(env.mut_call(()), 1);
    assert_eq!(env.mut_call(()), 2);
    assert_eq!(env.mut_call(()), 3);
}

#[test]
fn test_into_call() {
    struct WhatInto<T>(T);

    impl_call! {
        fn into_call[T,U](self: WhatInto<T>, _a: PhantomData<U>)->U
        where [ T:Into<U> ]
        {
            self.0.into()
        }
    }

    #[cfg(feature = "alloc")]
    assert_eq!(WhatInto("what").into_call(PhantomData::<String>), "what");

    assert_eq!(WhatInto(1u8).into_call(PhantomData::<u16>), 1);
}

#[test]
fn parameter_counts() {
    struct ZeroParam;

    impl_call! {
        fn ref_call(self: ZeroParam) -> u64 {
            3
        }
    }

    struct SingleParam;

    impl_call! {
        fn ref_call[T](self: SingleParam, single: T) -> u64
        where [ T:Into<u64> ]
        {
            single.into()
        }
    }

    struct AddTwo;

    impl_call! {
        fn ref_call[T](self: AddTwo, l: T, r: T) -> T
        where [ T: std_::ops::Add<Output = T> ]
        {
            l + r
        }
    }

    struct AddThree;

    impl_call! {
        fn ref_call[T](self: AddThree, f0: T, f1: T, f2: T) -> T
        where [ T: std_::ops::Add<Output = T> ]
        {
            f0 + f1 + f2
        }
    }

    assert_eq!(ZeroParam.ref_call(()), 3);

    assert_eq!(SingleParam.ref_call(5u8), 5);
    
    assert_eq!(AddTwo.ref_call((5, 3)), 8);
    
    assert_eq!(AddThree.ref_call((5, 8, 21)), 34);
}

#[test]
fn return_optionality() {
    struct ImplicitReturn;

    impl_call! {
        fn ref_call(self: ImplicitReturn, input: &mut u32) {
            *input = 27;
        }
    }
    
    struct ExplicitReturn;

    impl_call! {
        fn ref_call(self: ExplicitReturn) -> u32 {
            27
        }
    }

    let mut num = 0;
    let _: () = ImplicitReturn.ref_call(&mut num);
    assert_eq!(num, 27);

    assert_eq!(ExplicitReturn.ref_call(()), 27);
}


#[test]
fn which_impls() {
    struct DerivesCallRef;

    impl_call! {
        fn ref_call(self: DerivesCallRef) -> u32 {
            27
        }
    }

    struct DerivesCallMut;

    impl_call! {
        fn mut_call(self: DerivesCallMut) -> u32 {
            self.ref_call_(())
        }
    }
    impl CallRef<()> for DerivesCallMut {
        fn ref_call_(&self, _: ()) -> u32 {
            81
        }
    }


    struct DerivesCallInto;

    impl_call! {
        fn into_call(self: DerivesCallInto) -> u32 {
            self.ref_call_(())
        }
    }
    impl CallMut<()> for DerivesCallInto {
        fn mut_call_(&mut self, _: ()) -> u32 {
            self.ref_call_(())
        }
    }
    impl CallRef<()> for DerivesCallInto {
        fn ref_call_(&self, _: ()) -> u32 {
            243
        }
    }

    assert_eq!(DerivesCallRef.ref_call_(()), 27);
    assert_eq!(DerivesCallRef.mut_call_(()), 27);
    assert_eq!(DerivesCallRef.into_call_(()), 27);

    assert_eq!(DerivesCallMut.ref_call_(()), 81);
    assert_eq!(DerivesCallMut.mut_call_(()), 81);
    assert_eq!(DerivesCallMut.into_call_(()), 81);

    assert_eq!(DerivesCallInto.ref_call_(()), 243);
    assert_eq!(DerivesCallInto.mut_call_(()), 243);
    assert_eq!(DerivesCallInto.into_call_(()), 243);
}


#[test]
fn test_closures() {
    let mut ref_fn = || 10;
    
    let mut n = 0;
    let mut mut_fn = || {
        n += 1;
        n
    };
    
    let list = [0, 1, 2];
    let into_fn = || list;

    assert_eq!(ref_fn.ref_call(()), 10);
    assert_eq!(ref_fn.mut_call(()), 10);
    assert_eq!(ref_fn.into_call(()), 10);
    
    assert_eq!(mut_fn.mut_call(()), 1);
    assert_eq!(mut_fn.mut_call(()), 2);
    assert_eq!(mut_fn.mut_call(()), 3);
    assert_eq!(mut_fn.into_call(()), 4);

    assert_eq!(into_fn.into_call(()), [0, 1, 2]);
}
