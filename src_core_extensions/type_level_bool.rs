//! Type level booleans
//!
//! # Example
//!
//! Access of privileges based on renamed Boolean types.
//!
#![cfg_attr(feature = "phantom", doc = " ```rust")]
#![cfg_attr(not(feature = "phantom"), doc = " ```ignore")]
//! use std::mem;
//! use std::marker::PhantomData;
//!
//! use core_extensions::CovariantPhantom;
//!
//! pub use core_extensions::type_level_bool::{
//!     Boolean as PrivilegeLevel,
//!     False as Unprivileged,
//!     True as Privileged,
//! };
//!
//!
//! #[repr(C)]
//! #[derive(Debug)]
//! struct User<P>{
//!     name: String,
//!     privilege_level: CovariantPhantom<P>,
//! }
//!
//! impl<P: PrivilegeLevel> User<P>{
//!     fn new(name: String, privilege_level: P) -> Self {
//!         Self{name, privilege_level: PhantomData}
//!     }
//!     fn name(&self)->&str{
//!         &self.name
//!     }
//!     fn into_unprivileged(self) -> User<Unprivileged> {
//!         User{name: self.name, privilege_level: PhantomData}
//!     }
//!     fn as_unprivileged(self: &mut User<P>) -> &mut User<Unprivileged> {
//!         // Only the type parameter P changes here
//!         unsafe{ mem::transmute(self) }
//!     }
//! }
//!
//! impl User<Privileged>{
//!     fn set_name(&mut self, name: String){
//!         self.name = name;
//!     }
//! }
//!
//! let mut user: User<Privileged> = User::new("bob".into(), Privileged);
//! assert_eq!(user.name(), "bob");
//!
//! user.set_name("paul".into());
//! assert_eq!(user.name(), "paul");
//!
//! {
//!     let user: &mut User<Unprivileged> = user.as_unprivileged();
//!
//!     // Unprivileged Users can't change their name.
//!     // user.set_name("james".into());
//!
//!     assert_eq!(user.name(), "paul");
//! }
//!
//! user.set_name("john".into());
//! assert_eq!(user.name(), "john");
//!
//! ```
//!
//!

#[cfg(feature = "const_default")]
use crate::ConstDefault;

#[cfg(not(feature = "const_default"))]
use std_::marker::Sized as ConstDefault;



use std_::fmt::{self, Debug, Display};
use std_::ops;

#[cfg(feature = "marker_type")]
use crate::MarkerType;

#[cfg(not(feature = "marker_type"))]
use std_::marker::Sized as MarkerType;


/// Represents a type-level `true`
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct True;

/// Represents a type-level `false`
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct False;

impl Display for True {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("True")
    }
}

impl Display for False {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("False")
    }
}

mod sealed {
    use super::{False, True};
    pub trait Sealed {}
    impl Sealed for True {}
    impl Sealed for False {}
}
use self::sealed::Sealed;

#[cfg(feature = "marker_type")]
unsafe impl MarkerType for True {}

#[cfg(feature = "marker_type")]
unsafe impl MarkerType for False {}

const _: &[[(); 0]] = &[
    [(); std_::mem::size_of::<True>()],
    [(); std_::mem::size_of::<False>()],
    [(); std_::mem::align_of::<True>() - 1],
    [(); std_::mem::align_of::<False>() - 1],
];

#[cfg(feature = "const_default")]
impl ConstDefault for True {
    const DEFAULT: Self = True;
}

#[cfg(feature = "const_default")]
impl ConstDefault for False {
    const DEFAULT: Self = False;
}

/// Represents a type-level `bool`
///
/// Only implemented on [`True`] and [`False`].
///
/// For examples look at [the module-level documentation](./index.html).
///
/// This trait is sealed and cannot be implemented for types outside this crate.
///
/// [`True`]: ./struct.True.html
/// [`False`]: ./struct.False.html
pub trait Boolean:
    Sealed
    + MarkerType
    + ConstDefault
    + Default
    + Sized
    + Debug
    + Copy
    + Clone
    + ops::Not
    + ops::BitAnd<True, Output = Self>
    + ops::BitAnd<False, Output = False>
    + ops::BitAnd<Self, Output = Self>
    + ops::BitOr<True, Output = True>
    + ops::BitOr<False, Output = Self>
    + ops::BitOr<Self, Output = Self>
    + ops::BitXor<True, Output = <Self as ops::Not>::Output>
    + ops::BitXor<False, Output = Self>
    + ops::BitXor<Self, Output = False>
{
    /// The `bool` value of this type
    const VALUE: bool;
}

impl Boolean for True {
    const VALUE: bool = true;
}
impl Boolean for False {
    const VALUE: bool = false;
}

mod internals {
    #[cfg(feature = "const_default")]
    use crate::ConstDefault;

    use super::{Boolean, False, True, Not};

    use std_::ops;

    impl ops::Not for True {
        type Output = False;
        fn not(self) -> Self::Output {
            False
        }
    }
    impl ops::Not for False {
        type Output = True;
        fn not(self) -> Self::Output {
            True
        }
    }

    impl<B> ops::BitAnd<B> for False {
        type Output = False;
        fn bitand(self, _: B) -> Self::Output {
            False
        }
    }
    impl<B> ops::BitAnd<B> for True {
        type Output = B;
        fn bitand(self, v: B) -> Self::Output {
            v
        }
    }

    impl<B> ops::BitOr<B> for True {
        type Output = True;
        fn bitor(self, _: B) -> Self::Output {
            True
        }
    }
    impl<B> ops::BitOr<B> for False {
        type Output = B;
        fn bitor(self, v: B) -> Self::Output {
            v
        }
    }

    impl<B: Boolean> ops::BitXor<B> for True 
    where
        Not<B>: Boolean
    {
        type Output = Not<B>;

        fn bitxor(self, _: B) -> Self::Output {
            #[cfg(feature = "const_default")]
            {
                Not::<B>::DEFAULT
            }
            #[cfg(not(feature = "const_default"))]
            {
                Not::<B>::default()
            }
        }
    }
    impl<B> ops::BitXor<B> for False {
        type Output = B;
        fn bitxor(self, v: B) -> Self::Output {
            v
        }
    }
}

/// Negates a [Boolean](./trait.Boolean.html).
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(Not::<True >::VALUE, false);
///     assert_eq!(Not::<False>::VALUE, true);
///
pub type Not<T> = <T as ops::Not>::Output;

/// `And`s two [Boolean](./trait.Boolean.html)s
///
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(And::<True , True >::VALUE, true);
///     assert_eq!(And::<False, True >::VALUE, false);
///     assert_eq!(And::<True , False>::VALUE, false);
///     assert_eq!(And::<False, False>::VALUE, false);
///
pub type And<L, R> = <L as ops::BitAnd<R>>::Output;

/// `Or`s two [Boolean](./trait.Boolean.html)s
///
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(Or::<True , True >::VALUE, true);
///     assert_eq!(Or::<False, True >::VALUE, true);
///     assert_eq!(Or::<True , False>::VALUE, true);
///     assert_eq!(Or::<False, False>::VALUE, false);
///
pub type Or<L, R> = <L as ops::BitOr<R>>::Output;

/// `Xor`s two [Boolean](./trait.Boolean.html)s
///
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(Xor::<True , True >::VALUE, false);
///     assert_eq!(Xor::<False, True >::VALUE, true);
///     assert_eq!(Xor::<True , False>::VALUE, true);
///     assert_eq!(Xor::<False, False>::VALUE, false);
///
pub type Xor<L, R> = <L as ops::BitXor<R>>::Output;
