//! Contains implementable alternatives of the standard Fn/FnMut/FnOnce traits.



#[cfg(test)]
mod tests;



/// Extension trait for calling `Call*` closures.
pub trait CallExt {
    /// For calling [`CallRef::call_ref_`] with optional explicit generic arguments.
    ///
    /// # Example
    ///
    /// ```rust
    /// use core_extensions::{impl_call, CallExt};
    ///
    /// struct PushTwice;
    ///
    /// impl_call! { 
    ///     fn call_ref[T](self: PushTwice, vector: &mut Vec<T>, value: T )
    ///     where[ T: Clone ]
    ///     {
    ///         vector.push(value.clone());
    ///         vector.push(value);
    ///     }
    /// }
    ///
    /// let mut vector = Vec::new();
    ///
    /// // `Call*` style closures encode multiple parameters as tuples
    /// PushTwice.call_ref((&mut vector, 3));
    /// assert_eq!(vector, [3, 3]);
    ///
    /// PushTwice.call_ref((&mut vector, 5));
    /// assert_eq!(vector, [3, 3, 5, 5]);
    ///
    /// PushTwice.call_ref((&mut vector, 8));
    /// assert_eq!(vector, [3, 3, 5, 5, 8, 8]);
    ///
    /// ```
    /// 
    /// [`CallRef::call_ref_`]: ./trait.CallRef.html#tymethod.call_ref_
    #[inline(always)]
    fn call_ref<P>(&self, params: P) -> Self::Returns
    where
        Self: CallRef<P>
    {
        self.call_ref_(params)
    }

    /// For calling [`CallMut::call_mut_`] with optional explicit generic arguments.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use core_extensions::{impl_call, CallExt};
    ///
    /// struct ComputeFib {
    ///     nums: [u128; 2],
    /// }
    ///
    /// impl_call! { 
    ///     fn call_mut(self: ComputeFib, numbers: &mut Vec<u128>) {
    ///         let [l, r] = self.nums;
    ///         let next = l + r;
    ///         self.nums = [r, next];
    ///         numbers.push(r);
    ///     }
    /// }
    ///
    /// let mut fibs = ComputeFib {nums: [0, 1]};
    /// 
    /// let mut numbers = Vec::new();
    ///
    /// fibs.call_mut(&mut numbers);
    /// assert_eq!(numbers, [1]);
    ///
    /// fibs.call_mut(&mut numbers);
    /// assert_eq!(numbers, [1, 1]);
    ///
    /// fibs.call_mut(&mut numbers);
    /// assert_eq!(numbers, [1, 1, 2]);
    ///
    /// fibs.call_mut(&mut numbers);
    /// assert_eq!(numbers, [1, 1, 2, 3]);
    ///
    /// fibs.call_mut(&mut numbers);
    /// assert_eq!(numbers, [1, 1, 2, 3, 5]);
    ///
    /// fibs.call_mut(&mut numbers);
    /// assert_eq!(numbers, [1, 1, 2, 3, 5, 8]);
    ///
    ///
    /// ```
    /// 
    /// [`CallMut::call_mut_`]: ./trait.CallMut.html#tymethod.call_mut_
    #[inline(always)]
    fn call_mut<P>(&mut self, params: P) -> Self::Returns
    where
        Self: CallMut<P>
    {
        self.call_mut_(params)
    }

    /// For calling [`CallInto::call_into_`] with optional explicit generic arguments.
    ///
    /// # Example
    ///
    /// ```rust
    /// use core_extensions::{impl_call, CallExt};
    ///
    /// use std::iter::FromIterator;
    ///
    /// struct IntoElem<T>(T);
    ///
    /// impl_call! { 
    ///     fn call_into[T](self: IntoElem<T>, nth: usize) -> Option<T::Item>
    ///     where[ T: IntoIterator ]
    ///     {
    ///         self.0.into_iter().nth(nth)
    ///     }
    /// }
    /// 
    /// let list = vec![3, 5, 8, 13, 21, 34, 55, 89];
    /// 
    /// assert_eq!(IntoElem(list.clone()).call_into(0), Some(3));
    /// 
    /// assert_eq!(IntoElem(list.clone()).call_into(1), Some(5));
    /// 
    /// assert_eq!(IntoElem(list.clone()).call_into(2), Some(8));
    /// 
    /// assert_eq!(IntoElem(list.clone()).call_into(3), Some(13));
    /// 
    /// assert_eq!(IntoElem(list.clone()).call_into(7), Some(89));
    ///
    /// ```
    /// 
    /// [`CallInto::call_into_`]: ./trait.CallOnce.html#tymethod.call_into_
    #[inline(always)]
    fn call_into<P>(self, params: P) -> Self::Returns
    where
        Self: Sized,
        Self: CallInto<P>
    {
        self.call_into_(params)
    }
}

impl<T: ?Sized> CallExt for T {}


/// Implementable alternative to [`std::ops::Fn`].
///
/// # Parameters
///
/// The `Call*` traits encode multiple parameters like this:
/// 
/// - 0 parameters: by taking a `()` parameter.
/// 
/// - 1 parameters: by taking the single parameter.
/// 
/// - 2 or more parameters: by taking a tuple of the parameters.
/// 
/// # Example
///
/// ```rust
/// use core_extensions::{impl_call, CallExt};
///
/// struct MulBy<T>(T);
///
/// impl_call! { 
///     fn call_ref[T](self: MulBy<T>, lhs: T ) -> T
///     where[ T: Clone + std::ops::Mul<Output = T> ]
///     {
///         lhs * self.0.clone()
///     }
/// }
/// 
/// let two = MulBy(2);
/// let seven = MulBy(7);
/// 
/// assert_eq!(two.call_ref(3), 6);
/// assert_eq!(two.call_ref(5), 10);
/// 
/// assert_eq!(seven.call_ref(3), 21);
/// assert_eq!(seven.call_ref(5), 35);
/// 
/// ```
///
/// [`std::ops::Fn`]: https://doc.rust-lang.org/core/ops/trait.Fn.html
pub trait CallRef<Params>: CallMut<Params> {
    /// calls this function
    fn call_ref_(&self, params: Params) -> Self::Returns;
}

/// Implementable alternative to [`std::ops::FnMut`].
///
/// # Parameters
///
/// The `Call*` traits encode multiple parameters like this:
/// 
/// - 0 parameters: by taking a `()` parameter.
/// 
/// - 1 parameters: by taking the single parameter.
/// 
/// - 2 or more parameters: by taking a tuple of the parameters.
/// 
/// # Example
/// 
/// ```rust
/// use core_extensions::{impl_call, CallExt};
///
/// struct Reporter{
///     line: u32,
/// }
///
/// impl_call! { 
///     fn call_mut(self: Reporter, buffer: &mut String, person: &str, score: u32) {
///         use std::fmt::Write;
///
///         writeln!(buffer, "{}- {}: {}", self.line, person, score).unwrap();
///         
///         self.line += 1;
///     }
/// }
///
/// let mut reporter = Reporter{line: 0};
/// 
/// let mut buffer = String::new();
///
/// reporter.call_mut((&mut buffer, "foo", 10));
/// reporter.call_mut((&mut buffer, "bar", 7));
/// reporter.call_mut((&mut buffer, "baz", 1000));
///
/// assert_eq!(
///     buffer,
///     "\
///         0- foo: 10\n\
///         1- bar: 7\n\
///         2- baz: 1000\n\
///     "
/// );
///
/// ```
/// 
/// [`std::ops::FnMut`]: https://doc.rust-lang.org/core/ops/trait.FnMut.html
pub trait CallMut<Params>: CallInto<Params> {
    /// calls this function
    fn call_mut_(&mut self, params: Params) -> Self::Returns;
}

/// Implementable alternative to [`std::ops::FnOnce`].
///
/// # Parameters
///
/// The `Call*` traits encode multiple parameters like this:
/// 
/// - 0 parameters: by taking a `()` parameter.
/// 
/// - 1 parameters: by taking the single parameter.
/// 
/// - 2 or more parameters: by taking a tuple of the parameters.
/// 
/// # Example
///
/// ```rust
/// use core_extensions::{impl_call, CallExt};
///
/// use std::iter::FromIterator;
///
/// struct Duplicator<T>(T);
///
/// impl_call! { 
///     fn call_into[T](self: Duplicator<T>) -> T
///     where[
///         T: IntoIterator + Default,
///         T: FromIterator<<T as IntoIterator>::Item>,
///         T::Item: Clone,
///     ] {
///         self.0
///             .into_iter()
///             .flat_map(|elem| vec![elem; 2] )
///             .collect()
///     }
/// }
/// 
/// assert_eq!(Duplicator(vec![3, 5]).call_into(()), vec![3, 3, 5, 5]);
///
/// assert_eq!(Duplicator(vec!["hi", "ho"]).call_into(()), vec!["hi", "hi", "ho", "ho"]);
/// 
/// ```
///
/// [`std::ops::FnOnce`]: https://doc.rust-lang.org/core/ops/trait.FnOnce.html
pub trait CallInto<Params> {
    /// The return type of this function
    type Returns;
    /// calls this function
    fn call_into_(self, params: Params) -> Self::Returns;
}

macro_rules! impl_call {
    ( $( [$($ty:ident),+] )* ) => {
        $(
            impl<$($ty,)* Func,Ret> CallRef<($($ty,)*)> for Func
            where Func:Fn($($ty,)*)->Ret
            {
                #[allow(non_snake_case)]
                fn call_ref_(&self,($($ty,)*):($($ty,)*))->Ret{
                    self($($ty),*)
                }
            }

            impl<$($ty,)* Func,Ret> CallMut<($($ty,)*)> for Func
            where Func:FnMut($($ty,)*)->Ret
            {
                #[allow(non_snake_case)]
                fn call_mut_(&mut self,($($ty,)*):($($ty,)*))->Ret{
                    self($($ty,)*)
                }
            }

            impl<$($ty,)* Func,Ret> CallInto<($($ty,)*)> for Func
            where Func:FnOnce($($ty,)*)->Ret
            {
                type Returns = Ret;

                #[allow(non_snake_case)]
                fn call_into_(self,($($ty,)*):($($ty,)*))->Ret{
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
    fn call_ref_(&self, _: ()) -> Ret {
        self()
    }
}

impl<F, Ret> CallMut<()> for F
where
    F: FnMut() -> Ret,
{
    fn call_mut_(&mut self, _: ()) -> Ret {
        self()
    }
}

impl<F, Ret> CallInto<()> for F
where
    F: FnOnce() -> Ret,
{
    type Returns = Ret;
    fn call_into_(self, _: ()) -> Ret {
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
This macro allows more ergonomically implementing the 
[`CallRef`], [`CallMut`], |`CallInto`] traits .

# Examples

### Implementing `CallRef`.

```rust

use core_extensions::{impl_call, CallExt};

struct Environment;

impl_call!{
    fn call_ref(self: Environment, printing: &str ) {
        println!("printing '{}'",printing);
    }
}

Environment.call_ref("what the ...");

```

### Implementing `CallMut`.

Also demonstrates a polymorphic function, not possible in Rust closures yet.

```rust
use core_extensions::{impl_call, AsPhantomData, CallExt};

use std::marker::PhantomData;

struct Environment{
    i: u16,
}

impl_call!{
    // The PhantomData parameter is necessary because closures can't return a generic type
    // that doesn't appear in the parameter.
    fn call_mut[T](self: Environment, _a: PhantomData<T>) -> T
    where [ u16: Into<T>, ]
    {
        self.i += 1;
        self.i.into()
    }
}

let mut env = Environment{i:0};
assert_eq!(env.call_mut(u16::PHANTOM), 1);
assert_eq!(env.call_mut(u32::PHANTOM), 2);
```


### Implementing `CallInto`.

```rust
use core_extensions::{impl_call, CallExt};

struct Environment<T>(T);

impl_call!{
    fn call_into[T](self: Environment<T>)->T{
        self.0
    }
}


let env = Environment("hello");
assert_eq!(env.call_into(()), "hello");
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
(   
    self: <self_type>
    $(, <function_parameter> )*
    $(,)?
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



[`CallRef`]: ./callable/trait.CallRef.html

[`CallMut`]: ./callable/trait.CallMut.html

|`CallInto`]: ./callable/trait.CallInto.html

*/
#[macro_export]
macro_rules! impl_call{
    (
        $(#[$meta:meta])*
        fn $fn_kind:ident
        $( [ $( $fn_gen_params:tt )* ] )*
        ( $( $fn_params:tt )* )
        $( -> $ret_ty:ty )?
        $( where [ $( $where_preds:tt )* ] )*
        $body:block

    )=>{
        $crate::__priv_impl_call!{
            outer_step_a;
            (
                $(#[$meta])*
                fn $fn_kind
                [ $( $( $fn_gen_params )* )* ]
            )
            ( $( $fn_params )* )
            ($($ret_ty)?)
            (
                where [ $( $( $where_preds )* )* ]
                $body
            )
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_impl_call {
    (outer_step_a; $prefn:tt $fn_params:tt () $postfn:tt ) => {
        $crate::__priv_impl_call!{outer_step_b; $prefn $fn_params (()) $postfn}
    };
    (outer_step_a; $prefn:tt $fn_params:tt ($ret_ty:ty) $postfn:tt ) => {
        $crate::__priv_impl_call!{outer_step_b; $prefn $fn_params ($ret_ty) $postfn}
    };
    
   (outer_step_b;
        ($($prefn:tt)*)
        ( $self:ident: $fn_ty:ty, $params:ident : $params_ty:ty $(,)? )
        ($ret_ty:ty)
        ($($postfn:tt)*)
    ) => {
        $crate::__priv_impl_call!{
            inner_fn;
            $($prefn)*
            ( $self: $fn_ty, $params, $params_ty)
            -> $ret_ty
            $($postfn)*
        }
    };
   (outer_step_b;
        ($($prefn:tt)*)
        ( $self:ident: $fn_ty:ty  $(, $params:ident : $params_ty:ty )* $(,)? )
        ($ret_ty:ty)
        ($($postfn:tt)*)
    ) => {
        $crate::__priv_impl_call!{
            inner_fn;
            $($prefn)*
            ( $self: $fn_ty, ($($params),*), ($($params_ty),*))
            -> $ret_ty
            $($postfn)*
        }
    };

    (inner_fn;
        $(#[$meta:meta])*
        fn call_into
        [ $( $fn_gen_params:tt )* ]
        ( $self:ident: $fn_ty:ty, $params_pati:pat, $params_ty:ty)
        -> $ret_ty:ty
        where [ $( $where_preds:tt )* ]
        $body:block
    )=>{
        $(#[$meta])*
        impl< $($fn_gen_params)* > $crate::CallInto<$params_ty> for $fn_ty
        where
            $( $where_preds )*
        {
            type Returns = $ret_ty;

            fn call_into_($self, $params_pati: $params_ty) -> $ret_ty 
            $body
        }
    };
    (inner_fn;
        $(#[$meta:meta])*
        fn call_mut
        [ $( $fn_gen_params:tt )* ]
        ( $self:ident: $fn_ty:ty, $params_pati:pat, $params_ty:ty)
        -> $ret_ty:ty
        where [ $( $where_preds:tt )* ]
        $body:block
    )=>{
        $(#[$meta])*
        impl< $($fn_gen_params)* > $crate::CallInto<$params_ty> for $fn_ty
        where $( $where_preds )*
        {
            type Returns = $ret_ty;

            #[inline(always)]
            fn call_into_(mut $self, param : $params_ty) -> $ret_ty {
                $crate::CallMut::call_mut_(&mut $self, param)
            }
        }

        $(#[$meta])*
        impl< $($fn_gen_params)* > $crate::CallMut<$params_ty> for $fn_ty
        where $( $where_preds )*
        {
            fn call_mut_(&mut $self, $params_pati: $params_ty) -> $ret_ty
            $body
        }
    };

    (inner_fn;
        $(#[$meta:meta])*
        fn call_ref
        [ $( $fn_gen_params:tt )* ]
        ( $self:ident: $fn_ty:ty, $params_pati:pat, $params_ty:ty)
        -> $ret_ty:ty
        where [ $( $where_preds:tt )* ]
        $body:block
    )=>{
        $(#[$meta])*
        impl< $($fn_gen_params)* > $crate::CallInto<$params_ty> for $fn_ty
        where $( $where_preds )*
        {
            type Returns = $ret_ty;

            #[inline(always)]
            fn call_into_($self, param : $params_ty) -> $ret_ty {
                $crate::CallRef::call_ref_(&$self, param)
            }
        }

        $(#[$meta])*
        impl< $($fn_gen_params)* > $crate::CallMut<$params_ty> for $fn_ty
        where $( $where_preds )*
        {
            #[inline(always)]
            fn call_mut_(&mut $self, param : $params_ty) -> $ret_ty {
                $crate::CallRef::call_ref_($self, param)
            }
        }

        $(#[$meta])*
        impl< $($fn_gen_params)* > $crate::CallRef<$params_ty> for $fn_ty
        where $( $where_preds )*
        {
            fn call_ref_(&$self, $params_pati: $params_ty) -> $ret_ty
            $body
        }
    };
}