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


#[cfg(feature = "bools")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "bools")))]
mod bool_extensions;

#[cfg(feature = "bools")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "bools")))]
pub use self::bool_extensions::BoolExt;


#[cfg(feature = "callable")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "callable")))]
pub mod callable;

#[cfg(feature = "callable")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "callable")))]
pub use self::callable::{CallExt, CallInto, CallMut, CallRef};


#[cfg(feature = "collections")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "collections")))]
pub mod collections;


#[cfg(feature = "const_default")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_default")))]
mod const_default_trait;

#[cfg(feature = "const_default")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_default")))]
pub use self::const_default_trait::ConstDefault;



#[cfg(feature = "integers")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "integers")))]
pub mod integers;

#[cfg(feature = "integers")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "integers")))]
pub use self::integers::{IntegerExt, ToTime};


#[cfg(feature = "iterators")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iterators")))]
pub mod iterators;

#[cfg(feature = "iterators")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iterators")))]
pub use self::iterators::{IterCloner, IterConstructor, IteratorExt, LazyOnce};


#[cfg(feature = "marker_type")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "marker_type")))]
mod marker_type;

#[cfg(feature = "marker_type")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "marker_type")))]
pub use self::marker_type::MarkerType;


#[cfg(feature = "std")]
pub mod measure_time;


#[cfg(feature = "on_drop")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "on_drop")))]
mod on_drop;

#[cfg(feature = "on_drop")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "on_drop")))]
pub use self::on_drop::RunOnDrop;


#[cfg(feature = "option_result")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "option_result")))]
pub mod option_result_ext;

#[doc(no_inline)]
#[cfg(feature = "option_result")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "option_result")))]
pub use self::option_result_ext::{OptionExt, ResultExt, ResultLike, ResultLikeExt, TransposeOption};

#[cfg(feature = "phantom")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "phantom")))]
pub mod phantom;

#[cfg(feature = "phantom")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "phantom")))]
pub use self::phantom::{
    AsPhantomData,
    AndPhantom, AndPhantomCov,
    as_phantom, as_covariant_phantom,
    ContraVariantPhantom,
    InvariantPhantom, InvariantRefPhantom, VariantDropPhantom, CovariantPhantom,
};


#[cfg(feature = "self_ops")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "self_ops")))]
mod self_ops;

#[cfg(feature = "self_ops")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "self_ops")))]
pub use self::self_ops::SelfOps;


#[cfg(feature = "slices")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "slices")))]
pub mod slices;

#[cfg(feature = "slices")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "slices")))]
pub mod strings;

#[cfg(feature = "slices")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "slices")))]
pub use self::strings::StringExt;

#[cfg(feature = "slices")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "slices")))]
#[doc(no_inline)]
pub use self::slices::{ValSliceExt,SliceExt};


#[cfg(feature = "transparent_newtype")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "transparent_newtype")))]
pub mod transparent_newtype;

#[cfg(feature = "transparent_newtype")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "transparent_newtype")))]
pub use self::transparent_newtype::{TransparentNewtype, TransparentNewtypeExt};


#[cfg(feature = "type_identity")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "type_identity")))]
mod type_identity;

#[cfg(feature = "type_identity")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "type_identity")))]
pub use self::type_identity::{TIdentity, TypeIdentity};


#[cfg(test)]
mod test_utils;


#[cfg(feature = "type_asserts")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "type_asserts")))]
pub mod type_asserts;


#[cfg(feature = "type_level_bool")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "type_level_bool")))]
pub mod type_level_bool;


pub mod utils;

mod rust_version_assert;


#[cfg(feature = "void")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "void")))]
mod void;

#[cfg(feature = "void")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "void")))]
pub use self::void::Void;
