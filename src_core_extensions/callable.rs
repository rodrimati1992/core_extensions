//! Alternatives of the standard [`Fn`]/[`FnMut`]/[`FnOnce`] traits,
//! implementable in stable Rust.
//!
//! [`Fn`]: https://doc.rust-lang.org/std/ops/trait.Fn.html
//!
//! [`FnMut`]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
//!
//! [`FnOnce`]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html



#[cfg(test)]
mod tests;



/// Extension trait for calling `Call*` closures.
pub trait CallExt {
    /// For calling [`CallRef::ref_call_`],
    /// with the ability to specify the types of the arguments..
    ///
    /// # Example
    ///
    /// ```rust
    /// use core_extensions::{impl_call, CallExt};
    ///
    /// struct PushTwice;
    ///
    /// impl_call! { 
    ///     fn ref_call[T](self: PushTwice, vector: &mut Vec<T>, value: T )
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
    /// PushTwice.ref_call((&mut vector, 3));
    /// assert_eq!(vector, [3, 3]);
    ///
    /// // The turbofish operator can be used to specify the types of the arguments.
    /// PushTwice.ref_call::<(_, u128)>((&mut vector, 5));
    /// assert_eq!(vector, [3, 3, 5, 5]);
    ///
    /// PushTwice.ref_call((&mut vector, 8));
    /// assert_eq!(vector, [3, 3, 5, 5, 8, 8]);
    ///
    /// ```
    /// 
    /// [`CallRef::ref_call_`]: ./trait.CallRef.html#tymethod.ref_call_
    #[inline(always)]
    fn ref_call<P>(&self, params: P) -> Self::Returns
    where
        Self: CallRef<P>
    {
        self.ref_call_(params)
    }

    /// For calling [`CallMut::mut_call_`],
    /// with the ability to specify the types of the arguments..
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
    ///     fn mut_call(self: ComputeFib, numbers: &mut Vec<u128>) {
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
    /// // The turbofish operator can be used to specify the types of the arguments.
    /// fibs.mut_call::<&mut Vec<u128>>(&mut numbers);
    /// assert_eq!(numbers, [1]);
    ///
    /// fibs.mut_call(&mut numbers);
    /// assert_eq!(numbers, [1, 1]);
    ///
    /// fibs.mut_call(&mut numbers);
    /// assert_eq!(numbers, [1, 1, 2]);
    ///
    /// fibs.mut_call(&mut numbers);
    /// assert_eq!(numbers, [1, 1, 2, 3]);
    ///
    /// fibs.mut_call(&mut numbers);
    /// assert_eq!(numbers, [1, 1, 2, 3, 5]);
    ///
    /// fibs.mut_call(&mut numbers);
    /// assert_eq!(numbers, [1, 1, 2, 3, 5, 8]);
    ///
    ///
    /// ```
    /// 
    /// [`CallMut::mut_call_`]: ./trait.CallMut.html#tymethod.mut_call_
    #[inline(always)]
    fn mut_call<P>(&mut self, params: P) -> Self::Returns
    where
        Self: CallMut<P>
    {
        self.mut_call_(params)
    }

    /// For calling [`CallInto::into_call_`],
    /// with the ability to specify the types of the arguments..
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
    ///     fn into_call[T](self: IntoElem<T>, nth: usize) -> Option<T::Item>
    ///     where[ T: IntoIterator ]
    ///     {
    ///         self.0.into_iter().nth(nth)
    ///     }
    /// }
    /// 
    /// let list = vec![3, 5, 8, 13, 21, 34, 55, 89];
    /// 
    /// // The turbofish operator can be used to specify the types of the arguments.
    /// assert_eq!(IntoElem(list.clone()).into_call::<usize>(0), Some(3));
    /// 
    /// assert_eq!(IntoElem(list.clone()).into_call(1), Some(5));
    /// 
    /// assert_eq!(IntoElem(list.clone()).into_call(2), Some(8));
    /// 
    /// assert_eq!(IntoElem(list.clone()).into_call(3), Some(13));
    /// 
    /// assert_eq!(IntoElem(list.clone()).into_call(7), Some(89));
    ///
    /// ```
    /// 
    /// [`CallInto::into_call_`]: ./trait.CallInto.html#tymethod.into_call_
    #[inline(always)]
    fn into_call<P>(self, params: P) -> Self::Returns
    where
        Self: Sized,
        Self: CallInto<P>
    {
        self.into_call_(params)
    }
}

impl<T: ?Sized> CallExt for T {}


/// Implementable alternative to [`std::ops::Fn`].
///
/// # Parameters
///
/// The `Call*` traits encode multiple parameters like this:
/// 
/// - 0 parameters: by taking a `()` parameter, eg: `foo.ref_call(())`.
/// 
/// - 1 parameters: by taking the single parameter, eg: `foo.ref_call(10)`.
/// 
/// - 2 or more parameters: by taking a tuple of the parameters, eg: `foo.ref_call((10, 20))`.
/// 
/// # Example
///
/// ```rust
/// use core_extensions::{impl_call, CallExt};
///
/// struct MulBy<T>(T);
///
/// impl_call! { 
///     fn ref_call[T](self: MulBy<T>, lhs: T ) -> T
///     where[ T: Clone + std::ops::Mul<Output = T> ]
///     {
///         lhs * self.0.clone()
///     }
/// }
/// 
/// let two = MulBy(2);
/// let seven = MulBy(7);
/// 
/// assert_eq!(two.ref_call(3), 6);
/// assert_eq!(two.ref_call(5), 10);
/// 
/// assert_eq!(seven.ref_call(3), 21);
/// assert_eq!(seven.ref_call(5), 35);
/// 
/// ```
///
/// # Closure impls
///
/// Closures implement the `Call*` traits,
/// and they always require a tuple of the parameters to be passed in.
///
/// ```rust
/// use core_extensions::CallExt;
///
/// let fn_0 = || 10;
/// assert_eq!(fn_0.ref_call(()), 10);
///
/// let fn_1 = |a: i32| a + 10;
/// assert_eq!(fn_1.ref_call((5,)), 15);
///
/// let fn_2 = |a: i32, b: i32| a + b;
/// assert_eq!(fn_2.ref_call((8, 13)), 21);
/// ```
///
/// [`std::ops::Fn`]: https://doc.rust-lang.org/core/ops/trait.Fn.html
pub trait CallRef<Params>: CallMut<Params> {
    /// calls this function
    fn ref_call_(&self, params: Params) -> Self::Returns;
}

/// Implementable alternative to [`std::ops::FnMut`].
///
/// # Parameters
///
/// The `Call*` traits encode multiple parameters like this:
/// 
/// - 0 parameters: by taking a `()` parameter, eg: `foo.ref_call(())`.
/// 
/// - 1 parameters: by taking the single parameter, eg: `foo.ref_call(10)`.
/// 
/// - 2 or more parameters: by taking a tuple of the parameters, eg: `foo.ref_call((10, 20))`.
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
///     fn mut_call(self: Reporter, buffer: &mut String, person: &str, score: u32) {
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
/// reporter.mut_call((&mut buffer, "foo", 10));
/// reporter.mut_call((&mut buffer, "bar", 7));
/// reporter.mut_call((&mut buffer, "baz", 1000));
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
///
/// # Closure impls
///
/// Closures implement the `Call*` traits,
/// and they always require a tuple of the parameters to be passed in.
///
/// ```rust
/// use core_extensions::CallExt;
///
/// let mut i = 0;
///
/// let mut fn_0 = ||{i += 1; i};
/// assert_eq!(fn_0.mut_call(()), 1);
///
/// let mut fn_1 = |a: i32|{i += 1; a + i};
/// assert_eq!(fn_1.mut_call((5,)), 7);
///
/// let mut fn_2 = |a: i32, b: i32|{i += 1; a + b + i};
/// assert_eq!(fn_2.mut_call((8, 13)), 24);
/// ```
/// 
/// [`std::ops::FnMut`]: https://doc.rust-lang.org/core/ops/trait.FnMut.html
pub trait CallMut<Params>: CallInto<Params> {
    /// calls this function
    fn mut_call_(&mut self, params: Params) -> Self::Returns;
}

/// Implementable alternative to [`std::ops::FnOnce`].
///
/// # Parameters
///
/// The `Call*` traits encode multiple parameters like this:
/// 
/// - 0 parameters: by taking a `()` parameter, eg: `foo.ref_call(())`.
/// 
/// - 1 parameters: by taking the single parameter, eg: `foo.ref_call(10)`.
/// 
/// - 2 or more parameters: by taking a tuple of the parameters, eg: `foo.ref_call((10, 20))`.
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
///     fn into_call[T](self: Duplicator<T>) -> T
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
/// assert_eq!(Duplicator(vec![3, 5]).into_call(()), vec![3, 3, 5, 5]);
///
/// assert_eq!(Duplicator(vec!["hi", "ho"]).into_call(()), vec!["hi", "hi", "ho", "ho"]);
/// 
/// ```
///
/// # Closure impls
///
/// Closures implement the `Call*` traits,
/// and they always require a tuple of the parameters to be passed in.
///
/// ```rust
/// use core_extensions::CallExt;
///
/// let orig = vec![3, 5, 8, 13, 21, 34];
///
/// let list = orig.clone();
/// let fn_0 = || list.into_iter().next();
/// assert_eq!(fn_0.into_call(()), Some(3));
///
/// let list = orig.clone();
/// let fn_1 = |i: usize| list.into_iter().nth(i);
/// assert_eq!(fn_1.into_call((3,)), Some(13));
///
/// let list = orig.clone();
/// let fn_2 = |s: usize, i: usize| list.into_iter().skip(s).nth(i);
/// assert_eq!(fn_2.into_call((3, 1)), Some(21));
/// ```
///
/// [`std::ops::FnOnce`]: https://doc.rust-lang.org/core/ops/trait.FnOnce.html
pub trait CallInto<Params> {
    /// The return type of this function
    type Returns;
    /// calls this function
    fn into_call_(self, params: Params) -> Self::Returns;
}

macro_rules! impl_call {
    ( $( [$($ty:ident),+] )* ) => {
        $(
            impl<$($ty,)* Func,Ret> CallRef<($($ty,)*)> for Func
            where Func:Fn($($ty,)*)->Ret
            {
                #[allow(non_snake_case)]
                fn ref_call_(&self,($($ty,)*):($($ty,)*))->Ret{
                    self($($ty),*)
                }
            }

            impl<$($ty,)* Func,Ret> CallMut<($($ty,)*)> for Func
            where Func:FnMut($($ty,)*)->Ret
            {
                #[allow(non_snake_case)]
                fn mut_call_(&mut self,($($ty,)*):($($ty,)*))->Ret{
                    self($($ty,)*)
                }
            }

            impl<$($ty,)* Func,Ret> CallInto<($($ty,)*)> for Func
            where Func:FnOnce($($ty,)*)->Ret
            {
                type Returns = Ret;

                #[allow(non_snake_case)]
                fn into_call_(self,($($ty,)*):($($ty,)*))->Ret{
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
    fn ref_call_(&self, _: ()) -> Ret {
        self()
    }
}

impl<F, Ret> CallMut<()> for F
where
    F: FnMut() -> Ret,
{
    fn mut_call_(&mut self, _: ()) -> Ret {
        self()
    }
}

impl<F, Ret> CallInto<()> for F
where
    F: FnOnce() -> Ret,
{
    type Returns = Ret;
    fn into_call_(self, _: ()) -> Ret {
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
[`CallRef`](./callable/trait.CallRef.html) /
[`CallMut`](./callable/trait.CallMut.html) /
[`CallInto`](./callable/trait.CallInto.html)
traits .

# Examples

### Implementing `CallRef`.

```rust

use core_extensions::{impl_call, CallExt};

struct Environment;

impl_call!{
    fn ref_call(self: Environment, printing: &str ) {
        println!("printing '{}'",printing);
    }
}

Environment.ref_call("what the ...");

```

### Implementing `CallMut`.

Also demonstrates a polymorphic function, not possible in Rust closures yet.

*/
#[cfg_attr(feature = "phantom", doc = " ```rust")]
#[cfg_attr(not(feature = "phantom"), doc = " ```ignore")]
/**
use core_extensions::{impl_call, AsPhantomData, CallExt};

use std::marker::PhantomData;

struct Environment{
    i: u16,
}

impl_call!{
    // The PhantomData parameter is necessary because closures can't return a generic type
    // that doesn't appear in the parameter.
    fn mut_call[T](self: Environment, _a: PhantomData<T>) -> T
    where [ u16: Into<T>, ]
    {
        self.i += 1;
        self.i.into()
    }
}

let mut env = Environment{i:0};
assert_eq!(env.mut_call(u16::PHANTOM), 1);
assert_eq!(env.mut_call(u32::PHANTOM), 2);
```


### Implementing `CallInto`.

```rust
use core_extensions::{impl_call, CallExt};

struct Environment<T>(T);

impl_call!{
    fn into_call[T](self: Environment<T>)->T{
        self.0
    }
}


let env = Environment("hello");
assert_eq!(env.into_call(()), "hello");
```




# Syntax


`$( ... )*` means repeated 0 or more times.

`$( ... )+` means repeated 1 or more times.

`$( ... )?` means that this is optional.

 `< ... >` is a variable,replaced with whatever it refers to.


```text

$(#[$meta:meta])*

// <fn_method_name> is one of (into_call|mut_call|ref_call),determining which trait
// is implemented.
fn <fn_method_name>

// Optionally declares the generic parameters of the function.
$( [ $( <generic_parameter> )* ] )?

// <self_type> is the type of the closure environment,which is implementing the Call traits.
// <function_parameter> are optional function parameters.
(   
    self: <self_type>
    $(, <function_parameter> )*
    $(,)?
)

//<return_tyoe> optional return type,defaults to '()'.
$( -> <return_type> )?

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

[`CallInto`]: ./callable/trait.CallInto.html

*/
#[cfg(feature = "callable")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "callable")))]
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
        fn into_call
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

            fn into_call_($self, $params_pati: $params_ty) -> $ret_ty 
            $body
        }
    };
    (inner_fn;
        $(#[$meta:meta])*
        fn mut_call
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
            fn into_call_(mut $self, param : $params_ty) -> $ret_ty {
                $crate::CallMut::mut_call_(&mut $self, param)
            }
        }

        $(#[$meta])*
        impl< $($fn_gen_params)* > $crate::CallMut<$params_ty> for $fn_ty
        where $( $where_preds )*
        {
            fn mut_call_(&mut $self, $params_pati: $params_ty) -> $ret_ty
            $body
        }
    };

    (inner_fn;
        $(#[$meta:meta])*
        fn ref_call
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
            fn into_call_($self, param : $params_ty) -> $ret_ty {
                $crate::CallRef::ref_call_(&$self, param)
            }
        }

        $(#[$meta])*
        impl< $($fn_gen_params)* > $crate::CallMut<$params_ty> for $fn_ty
        where $( $where_preds )*
        {
            #[inline(always)]
            fn mut_call_(&mut $self, param : $params_ty) -> $ret_ty {
                $crate::CallRef::ref_call_($self, param)
            }
        }

        $(#[$meta])*
        impl< $($fn_gen_params)* > $crate::CallRef<$params_ty> for $fn_ty
        where $( $where_preds )*
        {
            fn ref_call_(&$self, $params_pati: $params_ty) -> $ret_ty
            $body
        }
    };
}