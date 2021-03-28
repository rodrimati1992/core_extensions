//! Extension traits for many standard/core library types/traits.
//! and other miscelaneuous types / traits / functions / macros.
//!
//! # no-std support
//!
//! To use this crate in no_std contexts disable the default-feature.
//!
//! # Supported Rust versions
//!
//! This crate support Rust back to 1.41.0,
//! requiring cargo features to enable features for newer versions.
//!
//! # Cargo Features
//!
//! ### 1.0 crate features
//!
//! The `"in_1_0"` feature enables all of these features,
//! you can use it instead of the ones below if you don't mind longer compile-times:
//!
//! - "collections": Enables the [`collections`] module, with traits for collection types.
//!
//! - `"bools"`: Enables the [`BoolExt`] trait, extension trait for `bool`.
//!
//! - `"callable"`: Enables the [`callable`] module, 
//! with stably implementable equivalents of the `Fn*` traits.
//!
//! - `"const_default"`:
//! Enables the [`ConstDefault`] trait, and [`const_default`] macro.
//!
//! - `"const_val"`:
//! Enables the [`ConstVal`] trait, [`getconst`] macro, and [`quasiconst`] macro.
//!
//! - `"integers"`: Enables the [`integers`] module, with extension traits for integer types.
//!
//! - `"iterators"`: Enables the [`iterators`] module, 
//! with the [`IteratorExt`] extension trait for iterators, and iterator types.
//!
//! - `"marker_type"`: Enables the [`MarkerType`] trait,
//! for trivially constructible, zero-sized, and aligned-to-1 types.
//!
//! - `"on_drop"`: Enables the [`RunOnDrop`] type,
//! a wrapper type that runs a closure at the end of the scope.
//!
//! - `"option_result"`: Enables the [`option_result_ext`] module,
//! with traits for `Option` and `Result`-like types.
//!
//! - `"phantom"`: Enables the [`phantom`] module(with `PhantomData`-related items),
//! [`expr_as_phantom`] macro,[`map_phantomdata`] macro, and [`return_type_phantom`] macro.
//!
//! - `"self_ops"`: Enables the [`SelfOps`] trait, an extension trait for all types.
//!
//! - `"slices"`:
//! Enables the [`slices`] module, with extension traits for `[T]` and `str` slices.
//!
//! - `"strings"`:
//! Enables the [`strings`] module, with the [`StringExt`] extension trait for strings.
//!
//! - `"transparent_newtype"`: Enables the [`transparent_newtype`] module,
//! with extension traits and functions for `#[repr(transparent)]` newtypes with public fields.
//!
//! - `"type_asserts"`: Enables the [`type_asserts`] module, with type-level assertiosn,
//! most useful in tests.
//!
//! - `"type_identity"`: Enables the [`TypeIdentity`] trait,
//! for proving that two types are equal, and converting between them in a generic context.
//!
//! - `"type_level_bool"`: Enables the [`type_level_bool`] module,
//! which encodes `bool`s on the type-level.
//!
//! - `"void"`: Enables the [`Void`] type, for impossible situations.
//!
//!
//! ### Version numbers
//!
//! These features enable code that require some Rust version past the minimum supported one:
//!
//! - "rust_1_46": Makes [`TransparentNewtype`] and [`TypeIdentity`]
//! associated functions that take `Rc<Self>` or `Arc<Self>` callable as methods.
//!
//! ### Support for other crates
//!
//! `"std"`: Enables `std` library support. Enabled by default. Implies the `"alloc"` feature.
//!
//! `"alloc"`: Enables `alloc` library support. Enabled by default.
//!
//! `"serde_"`: Enables serde support. Disabled by default.
//!
//! ### Language features
//!
//! `"const_generics"`:
//! Enables impls of traits for all array lengths, 
//! requires versions of Rust where const generics are stable.
//!
//! `"nightly_const_generics"`: 
//! Enables impls of traits for all array lengths in Rust nightly versions prior to 
//! the stabilization of const generics.
//!
//!
//! [`collections`]: ./collections/index.html
//! [`callable`]: ./callable/index.html
//! [`integers`]: ./integers/index.html
//! [`iterators`]: ./iterators/index.html
//! [`option_result_ext`]: ./option_result_ext/index.html
//! [`phantom`]: ./phantom/index.html
//! [`slices`]: ./slices/index.html
//! [`strings`]: ./strings/index.html
//! [`transparent_newtype`]: ./transparent_newtype/index.html
//! [`type_asserts`]: ./type_asserts/index.html
//! [`type_level_bool`]: ./type_level_bool/index.html
//!
//! [`BoolExt`]: ./trait.BoolExt.html
//! [`ConstDefault`]: ./trait.ConstDefault.html
//! [`ConstVal`]: ./trait.ConstVal.html
//! [`MarkerType`]: ./trait.MarkerType.html
//! [`SelfOps`]: ./trait.SelfOps.html
//! [`TypeIdentity`]: ./trait.TypeIdentity.html
//! [`TransparentNewtype`]: ./transparent_newtype/trait.TransparentNewtype.html
//!
//! [`RunOnDrop`]: ./struct.RunOnDrop.html
//! [`Void`]: ./enum.Void.html
//! 
//! [`const_default`]: ./macro.const_default.html
//! [`getconst`]: ./macro.getconst.html
//! [`quasiconst`]: ./macro.quasiconst.html
//! [`expr_as_phantom`]: ./macro.expr_as_phantom.html
//! [`map_phantomdata`]: ./macro.map_phantomdata.html
//! [`return_type_phantom`]: ./macro.return_type_phantom.html
//! 
//! [`IteratorExt`]: ./iterators/trait.IteratorExt.html
//! [`StringExt`]: ./strings/trait.StringExt.html
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

#[cfg(feature = "const_val")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_val")))]
mod const_val;


#[cfg(feature = "const_val")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_val")))]
pub use self::const_val::ConstVal;


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




#[doc(hidden)]
pub mod __ {
    pub use std_::marker::PhantomData as PD;
}


