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