//! Allows defining traits whose functions/methods can be safe/unsafe 
//! to call depending on the implementor.
//!
//! # Example 
//! An alternative implementation of Default whose safety is determined by the implementor.
//! 
//! ```
//! # #[macro_use]
//! # extern crate core_extensions;
//! use core_extensions::maybe_unsafe::{IsSafe,IsUnsafe,MaybeUnsafe};
//! use std::{fmt,mem};
//!
//! # fn main(){
//!
//! /// Alternative definition of Default.
//! trait AlternativeDefault{
//!     /// Whether AlternativeDefault::default is safe to call.
//!     ///
//!     /// If Safety=IsUnsafe one must consult the documentation of the implementor
//!     /// to check what must be done to maintain safety.
//!     type Safety:MaybeUnsafe;
//!
//!     fn default(safety:&Self::Safety)->Self;
//! } 
//!
//! #[derive(Debug,PartialEq)]
//! pub struct ZeroInit<T>(pub T);
//!
//! #[derive(Debug,PartialEq)]
//! pub struct WithDefault<T>(pub T);
//!
//! /// # Safety
//! ///
//! /// Make sure to use this only on types on which 0 is a valid bit pattern.
//! impl<T> AlternativeDefault for ZeroInit<T> {
//!     type Safety=IsUnsafe;
//!     fn default(safety:&IsUnsafe)->Self{
//!         unsafe_!{safety=>
//!             ZeroInit(mem::zeroed())
//!         }
//!     }
//! }
//!
//! impl<T> AlternativeDefault for WithDefault<T>
//! where T:Default
//! {
//!     type Safety=IsSafe;
//!     fn default(_:&IsSafe)->Self{
//!         WithDefault(Default::default())
//!     }
//! }
//!
//! fn no_unsafe<T,U,F>(v:&Option<T>,f:F)->U
//! where T:fmt::Debug+AlternativeDefault<Safety=IsSafe>,
//!       F:FnOnce(&T)->U,
//! {
//!     match v.as_ref() {
//!         Some(v)=>f(v),
//!         None=>f(&AlternativeDefault::default(&())),
//!     }
//! }
//!
//! fn uses_unsafe(v:Option<ZeroInit<usize>>)->usize{
//!     v.unwrap_or_else(||unsafe{
//!         //zeroing a usize is fine.
//!         IsUnsafe::with(AlternativeDefault::default)
//!     }).0
//! }
//!
//! no_unsafe(&None::<WithDefault<bool>> ,|v| assert_eq!(v.0,false) );
//! no_unsafe(&Some(WithDefault(true))   ,|v| assert_eq!(v.0,true) );
//! no_unsafe(&None::<WithDefault<usize>>,|v| assert_eq!(v.0,0)  );
//! no_unsafe(&Some(WithDefault(10))     ,|v| assert_eq!(v.0,10) );
//!
//! assert_eq!(uses_unsafe(Some(ZeroInit(10))),10);
//! assert_eq!(uses_unsafe(None              ),0);
//!
//!
//! # }
//!
//! ```
//!

use std_::borrow::Borrow;


/// The trait used to choose whether traits' function/method 
/// is safe(using IsSafe) or unsafe(using IsUnsafe) to call.
///
/// When passing a MaybeUnsafe as a parameter it is recommended to use an
/// `impl AsRef<IsSafe>`/`impl AsRef<IsUnsafe>` parameter for convenience.
///
/// This trait has a Sealed super trait to prevent users of this library from implementing it.
///
/// For examples of how to use this [look at the module-level documentation](index.html).
pub trait MaybeUnsafe:Sealed{
    /// Constructs a MaybeUnsafe,and passes it by reference to prevent 
    /// it from escaping this function call.
    ///
    /// This is unsafe because it applies to both IsSafe and IsUnsafe.
    unsafe fn with<F,U>(f:F)->U
    where F:FnOnce(&Self)->U;
}


/// Represents the `safe` effect.
///
/// Functions taking this as a parameter should not to be unsafe.
///
/// For examples of how to use this [look at the module-level documentation](index.html).
pub type IsSafe=();

impl MaybeUnsafe for IsSafe{
    unsafe fn with<F,U>(f:F)->U
    where F:FnOnce(&Self)->U
    { 
        f(&())
    }
}

/// Represents the `unsafe` effect.
///
/// Functions taking this as a parameter are equivalent to `unsafe fn`.
///
/// For examples of how to use this [look at the module-level documentation](index.html).
#[derive(Debug,PartialEq,Eq,Ord,PartialOrd,Hash)]
pub struct IsUnsafe(());


impl MaybeUnsafe for IsUnsafe{
    unsafe fn with<F,U>(f:F)->U
    where F:FnOnce(&Self)->U
    { 
        f(&IsUnsafe(()))
    }
}

impl Borrow<IsSafe> for IsUnsafe{
    fn borrow(&self)->&IsSafe{
        static UNIT:&()=&();
        UNIT
    }
}


/// Macro for correctly using unsafe{} blocks inside functions that take IsUnsafe references.
///
/// This macro ensures that an IsUnsafe reference was provided to use an unsafe block.
///
/// For more information about IsUnsafe/IsSafe/MaybeUnsafe look at the 
/// [maybe_unsafe module](./maybe_unsafe/index.html)
///
/// # Example 
/// ``` 
/// # #[macro_use]
/// # extern crate core_extensions;
/// use core_extensions::maybe_unsafe::IsUnsafe;
/// use std::mem;
///
/// # fn main(){
/// /// Returns a zero initialized Copy value.
/// /// 
/// /// # Safety
/// /// 
/// /// 0 must be a valid bitpattern for the returned value.
/// fn zeroed_copy<T>(safety:&IsUnsafe)->T
/// where 
///     T:Copy,
/// {
///     unsafe_!{safety=>
///         mem::zeroed()
///     }
/// }
///
/// # }
/// ``` 
#[macro_export]
macro_rules! unsafe_ {
    ($is_unsafe:expr=> $($tt:tt)* ) => {{
        let _:&$crate::maybe_unsafe::IsUnsafe=
            $crate::std_::borrow::Borrow::borrow(&$is_unsafe);
        unsafe{
            $($tt)*
        }
    }}
}


/////////////////////////////////////////////////////////////////////
///////                         SEALED              /////////////////
/////////////////////////////////////////////////////////////////////

mod sealed{
    use super::{IsSafe,IsUnsafe};
    pub trait Sealed{}
    impl Sealed for IsSafe{}
    impl Sealed for IsUnsafe{}
}
use self::sealed::Sealed;

