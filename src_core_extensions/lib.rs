//! This crate provides:
//!
//! - Extension traits for many standard/core library types/traits.
//!
//! - SelfOps:to extend all types with generic operations.
//!
//! - Type-level representations of bool and unsafe  .
//!
//! - Marker traits to encode invariants about types.
//!
//! - etc.
//!
//!
//! # no-std support
//!
//! To use this crate in no_std contexts disable the default-feature.
//!
//! # Supported Rust versions
//!
//! This crate support Rust back to 1.20,
//! using a build script to automatically enable features from newer versions.
//!
//! # Cargo Features
//!
//! `"std"`: Enables standard library support.Enabled by default.
//!
//! `"serde_"`: Enables serde support.Disabled by default.
//!
//! `"typenum"`: Enables trait impls for typenum types.Disabled by default.
//!
//! "collections":Enables trait impls for the collection traits in the collections module.
//!
//! `"const_generics"`:
//! Enables impls of traits for all array lengths, 
//! requires versions of Rust where const generics are stable.
//!
//! `"nightly_const_generics"`: 
//! Enables impls of traits for all array lengths in Rust nightly versions prior to 
//! the stabilization of const generics.
//!
//! # **Contents**
//!
//!
//! ## Extension trait:[SelfOps](./trait.SelfOps.html)
//!
//! This is implemented for all types.
//!
//! The most importand methods in this are:
//!
//! - [piped](./trait.SelfOps.html#method.piped):
//!      Allows emulating the pipeline operator.
//!
//! - [mutated](./trait.SelfOps.html#method.mutated):
//!      Allows mutating `self` with a closure passing it along the method chain
//!
//! - [observe](./trait.SelfOps.html#method.observe):
//!     Observes the value of `self` with a closure passing
//!     it along the method chain unmodified.
//!
//! - [into_](./trait.SelfOps.html#method.into_),
//!   [as_ref_](./trait.SelfOps.html#method.as_ref_),
//!   [as_mut_](./trait.SelfOps.html#method.as_mut_):
//!      Alternative syntax for the standard conversion methods.
//!
//! ## Other extension traits
//!
//!
//! [ResultExt](./option_result_ext/trait.ResultExt.html)::Extension trait for [Result].
//!
//! [OptionExt](./option_result_ext/trait.OptionExt.html)::Extension trait for [Option].
//!
//! [BoolExt](./bool_extensions/trait.BoolExt.html):Extension trait for bool.
//!
//! [IntegerExt](./integers/trait.IntegerExt.html):Extension trait for integers.
//!
//! [ToTime](./integers/trait.ToTime.html):Extension trait for integers, to create
//! [Duration](::std::time::Duration)s of a certain unit.
//!
//! [StringExt](./strings/trait.StringExt.html)Extension trait for `str`.
//!
//!
//! ## Construction traits
//!
//! [MarkerType](./marker_traits/trait.MarkerType.html):
//! Represents a zero-sized marker type.
//!
//! [ConstDefault](./trait.ConstDefault.html):
//! A const-equivalent of the Default trait.
//!
//! ## Other traits
//!
//! [ResultLike](./option_result_ext/trait.ResultLike.html):
//! Trait for types with item/error values,like Option and Result.
//!
//! [TransparentNewtype](./transparent_newtype/trait.TransparentNewtype.html)
//! Represents a newtype that is safe to transmute to and/from its only non-zero-sized field.
//!
//!
//!
//! ## Iteration
//!
//! [IteratorExt](./iterators/trait.IteratorExt.html)
//! Extension trait for [Iterator] implementors.
//!
//! ### Factories
//!
//! [IterCloner](./iterators/struct.IterCloner.html):
//!     Constructs [Iterator]s by cloning the one it references,only possible if it is Clone.
//!
//! [IterConstructor](./iterators/struct.IterConstructor.html):
//! Constructs [Iterator]s using a closure,this can be done multiple times if the closure it Copy.
//!
//! ### Iterators
//!
//! [Loop](./iterators/struct.Loop.html):
//! Iterator that infinitely produces a value by calling an impl FnMut()->T
//!
//! [LazyOnce](./iterators/struct.LazyOnce.html):
//! Lazy version of [::std::iter::Once],only evaluating the item when
//! [Iterator::next] is called.
//!
//!
//!
//! ## Type-level stuff
//!
//! ### Type-Level bool
//!
//! This crate contains
//! [types and operations using type-level `bool`s](./type_level_bool/index.html).
//!    
//! [Boolean](./type_level_bool/trait.Boolean.html):
//! Trait representing `bool`.
//!
//! [True](./type_level_bool/struct.True.html)/
//! [False](./type_level_bool/struct.False.html):
//! Types representing `true`/`false`.
//!
//! ### Type equality
//!
//! [TypeIdentity](./trait.TypeIdentity.html)
//! Allows converting a type back to itself.Useful in generic contexts.
//!
//! ### Empty type
//!
//! [Void](./void/enum.Void.html):
//! Uninstantiable Type for statically impossible situations.
//! Useful as a type parameter/associated type.
//!
//!
//! ### (Un)safety represented as type
//!
//! [maybe_unsafe](./maybe_unsafe/index.html) emulates a safe/unsafe effect using types.
//!
//! Allows having traits whose implementations can choose whether their methods/functions
//! are safe to call or not.
//!
//!
//!

#![deny(missing_docs)]
#![deny(unused_must_use)]
#![cfg_attr(not(miri), no_std)]
#![cfg_attr(feature = "nightly_const_generics", feature(min_const_generics))]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]

#[cfg(feature="std")]
#[macro_use]
#[doc(hidden)]
pub extern crate std as std_;

#[cfg(not(feature="alloc"))]
#[doc(hidden)]
pub extern crate core as std_;

#[cfg(feature="alloc")]
#[doc(hidden)]
#[macro_use]
pub extern crate alloc;

#[cfg(all(not(feature = "std"),feature = "alloc"))]
#[doc(hidden)]
pub use alloc as alloc_;

#[cfg(feature = "std")]
#[doc(hidden)]
pub use std_ as alloc_;

#[doc(hidden)]
#[macro_use]
pub mod macros;

#[cfg(feature = "serde_")]
extern crate serde;

#[cfg(feature = "typenum")]
extern crate typenum;

#[cfg(test)]
extern crate rand;

mod bool_extensions;
pub mod callable;

#[cfg(feature = "collections")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "collections")))]
pub mod collections;

mod const_default_trait;
pub mod integers;
pub mod iterators;

mod marker_traits;
pub mod measure_time;

pub mod option_result_ext;
pub mod phantom;
mod self_ops;
pub mod slices;
pub mod strings;
pub mod transparent_newtype;
mod type_identity;

#[cfg(test)]
mod test_utils;

pub mod type_asserts;
pub mod type_level_bool;
pub mod utils;
pub mod void;

mod rust_version_assert;

/// The items from this crate which are almost always used.
pub mod prelude {
    #[doc(inline)]
    pub use self_ops::SelfOps;

    #[doc(inline)]
    pub use strings::StringExt;

    #[doc(inline)]
    pub use marker_traits::MarkerType;

    #[doc(inline)]
    pub use type_identity::{TIdentity, TypeIdentity};

    #[doc(inline)]
    pub use phantom::CovariantPhantom;
}

pub use self::const_default_trait::ConstDefault;
pub use self::callable::{CallExt, CallInto, CallMut, CallRef};
#[doc(inline)]
pub use self::self_ops::SelfOps;
pub use self::strings::StringExt;

pub use self::bool_extensions::BoolExt;
pub use self::integers::{IntegerExt, ToTime};
pub use self::iterators::{IterCloner, IterConstructor, IteratorExt, LazyOnce};
#[doc(inline)]
pub use self::marker_traits::MarkerType;
pub use self::option_result_ext::{OptionExt, ResultExt, ResultLike};
pub use self::phantom::{
    AsPhantomData,
    AndPhantom, AndPhantomCov,
    as_phantom, as_covariant_phantom,
    ContraVariantPhantom,
    InvariantPhantom, InvariantRefPhantom, VariantDropPhantom, CovariantPhantom,
};
pub use self::slices::{ValSliceExt,SliceExt};
pub use self::transparent_newtype::{TransparentNewtype, TransparentNewtypeExt};

#[doc(inline)]
pub use self::type_identity::{TIdentity, TypeIdentity};

pub use self::void::Void;
