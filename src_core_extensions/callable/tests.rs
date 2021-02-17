use crate::{
    impl_call,
    AsPhantomData,
    CallExt, CallRef, CallMut, CallInto,
};

use std_::{
    cmp::PartialEq,
    marker::PhantomData,
};

use alloc_::string::{String,ToString};

#[test]
fn test_call_ref() {
    struct WhatRef<T>(T);

    impl_call! {
        fn call_ref['a,T,U](self:WhatRef<T>, what:U )->bool
        where [ T:PartialEq<U>, ]
        {
            self.0==what
        }
    }

    let env = WhatRef("hello".to_string());
    assert_eq!(env.call_ref("hello"), true);
    assert_eq!(env.call_ref("hello".to_string()), true);
    assert_eq!(env.call_ref("lo"), false);
}

#[test]
fn test_call_mut() {
    struct WhatMut {
        state: usize,
    }

    impl_call! {
        fn call_mut(self:WhatMut)->usize{
            self.state+=1;
            self.state
        }
    }

    let mut env = WhatMut { state: 0 };
    assert_eq!(env.call_mut(()), 1);
    assert_eq!(env.call_mut(()), 2);
    assert_eq!(env.call_mut(()), 3);
}

#[test]
fn test_call_into() {
    struct WhatInto<T>(T);

    impl_call! {
        fn call_into[T,U](self: WhatInto<T>, _a: PhantomData<U>)->U
        where [ T:Into<U> ]
        {
            self.0.into()
        }
    }

    assert_eq!(WhatInto("what").call_into(String::PHANTOM), "what");
    assert_eq!(WhatInto(1u8).call_into(u16::PHANTOM), 1);
}

#[test]
fn parameter_counts() {
    struct ZeroParam;

    impl_call! {
        fn call_ref(self: ZeroParam) -> u64 {
            3
        }
    }

    struct SingleParam;

    impl_call! {
        fn call_ref[T](self: SingleParam, single: T) -> u64
        where [ T:Into<u64> ]
        {
            single.into()
        }
    }

    struct AddTwo;

    impl_call! {
        fn call_ref[T](self: AddTwo, l: T, r: T) -> T
        where [ T: std_::ops::Add<Output = T> ]
        {
            l + r
        }
    }

    struct AddThree;

    impl_call! {
        fn call_ref[T](self: AddThree, f0: T, f1: T, f2: T) -> T
        where [ T: std_::ops::Add<Output = T> ]
        {
            f0 + f1 + f2
        }
    }

    assert_eq!(ZeroParam.call_ref(()), 3);

    assert_eq!(SingleParam.call_ref(5u8), 5);
    
    assert_eq!(AddTwo.call_ref((5, 3)), 8);
    
    assert_eq!(AddThree.call_ref((5, 8, 21)), 34);
}

#[test]
fn return_optionality() {
    struct ImplicitReturn;

    impl_call! {
        fn call_ref(self: ImplicitReturn, input: &mut u32) {
            *input = 27;
        }
    }
    
    struct ExplicitReturn;

    impl_call! {
        fn call_ref(self: ExplicitReturn) -> u32 {
            27
        }
    }

    let mut num = 0;
    let _: () = ImplicitReturn.call_ref(&mut num);
    assert_eq!(num, 27);

    assert_eq!(ExplicitReturn.call_ref(()), 27);
}


#[test]
fn which_impls() {
    struct DerivesCallRef;

    impl_call! {
        fn call_ref(self: DerivesCallRef) -> u32 {
            27
        }
    }

    struct DerivesCallMut;

    impl_call! {
        fn call_mut(self: DerivesCallMut) -> u32 {
            self.call_ref_(())
        }
    }
    impl CallRef<()> for DerivesCallMut {
        fn call_ref_(&self, _: ()) -> u32 {
            81
        }
    }


    struct DerivesCallInto;

    impl_call! {
        fn call_into(self: DerivesCallInto) -> u32 {
            self.call_ref_(())
        }
    }
    impl CallMut<()> for DerivesCallInto {
        fn call_mut_(&mut self, _: ()) -> u32 {
            self.call_ref_(())
        }
    }
    impl CallRef<()> for DerivesCallInto {
        fn call_ref_(&self, _: ()) -> u32 {
            243
        }
    }

    assert_eq!(DerivesCallRef.call_ref_(()), 27);
    assert_eq!(DerivesCallRef.call_mut_(()), 27);
    assert_eq!(DerivesCallRef.call_into_(()), 27);

    assert_eq!(DerivesCallMut.call_ref_(()), 81);
    assert_eq!(DerivesCallMut.call_mut_(()), 81);
    assert_eq!(DerivesCallMut.call_into_(()), 81);

    assert_eq!(DerivesCallInto.call_ref_(()), 243);
    assert_eq!(DerivesCallInto.call_mut_(()), 243);
    assert_eq!(DerivesCallInto.call_into_(()), 243);
}




