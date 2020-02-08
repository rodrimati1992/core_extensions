//! Contains implementable alternatives of the standard Fn/FnMut/FnOnce traits.

/// implementable version of ::std::ops::Fn.
pub trait CallRef<Params>: CallMut<Params> {
    /// calls this function
    fn call_ref(&self, params: Params) -> Self::Returns;
}

/// implementable version of ::std::ops::FnMut.
pub trait CallMut<Params>: CallInto<Params> {
    /// calls this function
    fn call_mut(&mut self, params: Params) -> Self::Returns;
}

/// implementable version of ::std::ops::FnOnce.
pub trait CallInto<Params> {
    /// The return type of this function
    type Returns;
    /// calls this function
    fn call_into(self, params: Params) -> Self::Returns;
}

macro_rules! impl_call {
    ( $( [$($ty:ident),+] )* ) => {
        $(
            impl<$($ty,)* Func,Ret> CallRef<($($ty,)*)> for Func
            where Func:Fn($($ty,)*)->Ret
            {
                #[allow(non_snake_case)]
                fn call_ref(&self,($($ty,)*):($($ty,)*))->Ret{
                    self($($ty),*)
                }
            }

            impl<$($ty,)* Func,Ret> CallMut<($($ty,)*)> for Func
            where Func:FnMut($($ty,)*)->Ret
            {
                #[allow(non_snake_case)]
                fn call_mut(&mut self,($($ty,)*):($($ty,)*))->Ret{
                    self($($ty,)*)
                }
            }

            impl<$($ty,)* Func,Ret> CallInto<($($ty,)*)> for Func
            where Func:FnOnce($($ty,)*)->Ret
            {
                type Returns=Ret;

                #[allow(non_snake_case)]
                fn call_into(self,($($ty,)*):($($ty,)*))->Ret{
                    self($($ty,)*)
                }
            }

        )*
    }
}

impl<F, Ret> CallRef<()> for F
where
    F: Fn() -> Ret,
{
    fn call_ref(&self, _: ()) -> Ret {
        self()
    }
}

impl<F, Ret> CallMut<()> for F
where
    F: FnMut() -> Ret,
{
    fn call_mut(&mut self, _: ()) -> Ret {
        self()
    }
}

impl<F, Ret> CallInto<()> for F
where
    F: FnOnce() -> Ret,
{
    type Returns = Ret;
    fn call_into(self, _: ()) -> Ret {
        self()
    }
}

impl_call! {
    [A]
    [A,B]
    [A,B,C]
    [A,B,C,D]
    [A,B,C,D,E]
    [A,B,C,D,E,F]
    [A,B,C,D,E,F,G]
    [A,B,C,D,E,F,G,H]
    [A,B,C,D,E,F,G,H,I]
    [A,B,C,D,E,F,G,H,I,J]
    [A,B,C,D,E,F,G,H,I,J,K]
    [A,B,C,D,E,F,G,H,I,J,K,L]
}

/**
This macro allows more ergonomically implementing the Call(Ref|Mut|Into) traits .


# Examples

Implementing CallRef.
The lifetime is written out explicitly because this macro desugars to
impl blocks,which don't elide lifetime parameters in Rust 2015 edition.

```rust

# #[macro_use]
# extern crate core_extensions;
# use core_extensions::*;

struct Environment;

callable_impl!{
    fn call_ref['a](this:Environment => printing:&'a str ){
        println!("printing '{}'",printing);
    }
}


# fn main(){
    Environment.call_ref("what the ...");
# }

```

Implementing CallMut.
Also demonstrates a polymorphic function,
not possible in Rust closures until it gets higher ranked closures.



```rust

# #[macro_use]
# extern crate core_extensions;
# use core_extensions::*;
# use std::marker::PhantomData;

struct Environment{
    i:u16,
}

callable_impl!{
    fn call_mut[T](this:Environment => _a:VariantPhantom<T>)->T
    where [ u16:Into<T>, ]
    {
        this.i+=1;
        this.i.into()
    }
}


# fn main(){
    let mut env=Environment{i:0};
    assert_eq!(env.call_mut(u16::T),1);
    assert_eq!(env.call_mut(u32::T),2);
# }
```



Implementing CallInto.


```rust

# #[macro_use]
# extern crate core_extensions;
# use core_extensions::*;

struct Environment<T>(T);

callable_impl!{
    fn call_into[T](this:Environment<T>)->T{
        this.0
    }
}


# fn main(){
    let env=Environment("hello");
    assert_eq!(env.call_into(()),"hello");
# }
```




# Syntax


`$( ... )*` means repeated 0 or more times.

`$( ... )+` means repeated 1 or more times.

`$( ... )?` means that this is optional.

 `< ... >` is a variable,replaced with whatever it refers to.


```text

$(#[$meta:meta])*

// <fn_method_name> is one of (call_into|call_mut|call_ref),determining which trait
// is implemented.
fn <fn_method_name>

// Optionally declares the generic parameters of the function.
$( [ $( <generic_parameter> )* ] )?

// <self_ident> is the identifier used to access the closure environment.
// <self_type> is the type of the closure environment,which is implementing the Call traits.
// <function_parameter> are optional function parameters.
(   <self_ident>:<self_type>
    $( => $( <function_parameter> ),* )?
)

//<return_tyoe> optional return type,defaults to '()'.
$( -><return_type> )?

// An optional where clause,
// all tokens inside `[...]` get copied directly to the where clause of the impl.
$( where [ $( <where_predicates> )* ] )*

{
    // The definition of the function
    <function_definition>
}



```


*/
#[macro_export]
macro_rules! callable_impl{
    (
        $(#[$meta:meta])*
        fn $fn_kind:ident
        $( [ $( $fn_gen_params:tt )* ] )*
        ( $( $fn_params:tt )* )
        $( ->$ret_ty:ty )*
        $( where [ $( $where_preds:tt )* ] )*
        {
            $( $fn_contents:tt )*
        }

    )=>{
        callable_impl!{inner_fn;
            $(#[$meta])*
            fn $fn_kind
            [ $( $( $fn_gen_params )* )* ]
            ( $( $fn_params )* )
            $(-> $ret_ty )*
            where [ $( $( $where_preds )* )* ]
            {
                $( $fn_contents )*
            }

        }
    };

    (inner_param; $param:expr $(,)* )=>{
        let ()=$param;
    };

    (inner_param; $param:expr , $param0:ident : $param0_ty:ty $(,)* )=>{
        let $param0=$param;
    };

    (inner_param; $param:expr , $( $params:ident : $params_ty:ty ),+ $(,)* )=>{
        let ($($params,)+)=$param;
    };

    (inner_param_ty; $(,)* )=>{
        ()
    };

    (inner_param_ty; $param0:ident : $param0_ty:ty $(,)* )=>{
        $param0_ty
    };

    (inner_param_ty; $( $params:ident : $params_ty:ty ),+ $(,)* )=>{
        ($($params_ty,)+)
    };

    (inner_fn;
        $(#[$meta:meta])*
        fn call_into
        [ $( $fn_gen_params:tt )* ]
        ( $self_:ident:$fn_ty:ty $(=>  $($rem_param:tt)* )*  )
        $(->$ret_ty:ty)*
        where [ $( $where_preds:tt )* ]
        {
            $( $fn_contents:tt )*
        }
    )=>{
        $(#[$meta])*
        impl< $($fn_gen_params)* >
            $crate::callable::CallInto< callable_impl!{inner_param_ty; $($($rem_param)*)* } >

        for $fn_ty
        where
            $( $where_preds )*
        {
            #[allow(unused_parens)]
            type Returns=($($ret_ty)*);

            fn call_into(
                self,
                param : callable_impl!{inner_param_ty; $($($rem_param)*)* }
            )->Self::Returns{
                callable_impl!{inner_param; param, $($($rem_param)*)* }
                let $self_=self;
                $( $fn_contents )*
            }
        }
    };


    (inner_fn;
        $(#[$meta:meta])*
        fn call_mut
        [ $( $fn_gen_params:tt )* ]
        ( $self_:ident:$fn_ty:ty $(=>  $($rem_param:tt)* )*  )
        $(->$ret_ty:ty)*
        where [ $( $where_preds:tt )* ]
        {
            $( $fn_contents:tt )*
        }
    )=>{
        $(#[$meta])*
        impl< $($fn_gen_params)* >
            $crate::callable::CallInto< callable_impl!{inner_param_ty; $($($rem_param)*)* }  >
        for $fn_ty
        where $( $where_preds )*
        {
            #[allow(unused_parens)]
            type Returns=($($ret_ty)*);

            fn call_into(
                mut self,
                param : callable_impl!{inner_param_ty; $($($rem_param)*)* }
            )->Self::Returns{
                self.call_mut(param)
            }
        }

        $(#[$meta])*
        impl< $($fn_gen_params)* >
            $crate::callable::CallMut< callable_impl!{inner_param_ty; $($($rem_param)*)* }  >
        for $fn_ty
        where $( $where_preds )*
        {
            fn call_mut(
                &mut self,
                param : callable_impl!{inner_param_ty; $($($rem_param)*)* }
            )->Self::Returns{
                callable_impl!{inner_param; param, $($($rem_param)*)* }
                let $self_=self;
                $( $fn_contents )*
            }
        }
    };

    (inner_fn;
        $(#[$meta:meta])*
        fn call_ref
        [ $( $fn_gen_params:tt )* ]
        ( $self_:ident:$fn_ty:ty $(=>  $($rem_param:tt)* )*  )
        $(->$ret_ty:ty)*
        where [ $( $where_preds:tt )* ]
        {
            $( $fn_contents:tt )*
        }
    )=>{
        $(#[$meta])*
        impl< $($fn_gen_params)* >
            $crate::callable::CallInto< callable_impl!{inner_param_ty; $($($rem_param)*)* }  >
        for $fn_ty
        where $( $where_preds )*
        {
            #[allow(unused_parens)]
            type Returns=($($ret_ty)*);

            fn call_into(
                self,
                param : callable_impl!{inner_param_ty; $($($rem_param)*)* }
            )->Self::Returns{
                self.call_ref(param)
            }
        }

        $(#[$meta])*
        impl< $($fn_gen_params)* >
            $crate::callable::CallMut< callable_impl!{inner_param_ty; $($($rem_param)*)* }  >
        for $fn_ty
        where $( $where_preds )*
        {
            fn call_mut(
                &mut self,
                param : callable_impl!{inner_param_ty; $($($rem_param)*)* }
            )->Self::Returns{
                self.call_ref(param)
            }
        }

        $(#[$meta])*
        impl< $($fn_gen_params)* >
            $crate::callable::CallRef< callable_impl!{inner_param_ty; $($($rem_param)*)* }  >
        for $fn_ty
        where $( $where_preds )*
        {
            fn call_ref(
                &self,
                param : callable_impl!{inner_param_ty; $($($rem_param)*)* }
            )->Self::Returns{
                callable_impl!{inner_param; param, $($($rem_param)*)* }
                let $self_=self;
                $( $fn_contents )*
            }
        }


    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use prelude::*;

    use std_::cmp::PartialEq;

    use alloc_::string::{String,ToString};

    #[test]
    fn test_call_ref() {
        struct WhatRef<T>(T);

        callable_impl! {
            fn call_ref['a,T,U](this:WhatRef<T> => what:U )->bool
            where [ T:PartialEq<U>, ]
            {
                this.0==what
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

        callable_impl! {
            fn call_mut(this:WhatMut)->usize{
                this.state+=1;
                this.state
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

        callable_impl! {
            fn call_into[T,U](this:WhatInto<T> => _a:VariantPhantom<U>)->U
            where [ T:Into<U> ]
            {
                this.0.into()
            }
        }

        assert_eq!(WhatInto("what").call_into(String::T), "what");
        assert_eq!(WhatInto(1u8).call_into(u16::T), 1);
    }

}
