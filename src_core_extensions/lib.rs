//! Extension traits for many standard/core library types/traits.
//! and other miscelaneuous types / traits / functions / macros.
//!
//! # Adding as dependency
//! 
//! This crate requires cargo features for enabling items, to get all of them you can use:
//! 
//! ```toml
//! [dependencies.core_extensions]
//! version = "1.5"
//! features = [
//!     ## enables items that use anything from the standard `std` or `alloc` crates.
//!     "std",
//!     ## Requires the latest stable release, enables all the rust-version-dependent features
//!     "rust_latest_stable",
//!     ## enables all the item features 
//!     "all_items",
//! ]
//! ```
//! The `"std"` feature is required to enable impls and items that use [`std`] types,
//! otherwise only the [`core`] library is supported.
//! 
//! `"rust_latest_stable"` enables all the `"rust_1_*"` crate features
//! to use the newest stable language features,
//! [here's a list of all the `"rust_1_*"` features](#cargo-features-lang-section),
//! 
//! `"all_items"` enables all of the *item features* ([documented here](#cargo-features-section)):
//! 
//! Here is the expanded version of the above configuration:
//! ```toml
//! [dependencies.core_extensions]
//! version = "1.5"
//! features = [
//!     "std",
//!     "rust_latest_stable"
//!     ## all of the features below are what "all_items" enables
//!     "derive"
//!     "bools",
//!     "callable",
//!     "collections",
//!     "const_default",
//!     "const_val",
//!     "generics_parsing",
//!     "integers",
//!     "item_parsing",
//!     "iterators",
//!     "macro_utils",
//!     "marker_type",
//!     "on_drop",
//!     "option_result",
//!     "phantom",
//!     "self_ops",
//!     "slices",
//!     "strings",
//!     "transparent_newtype",
//!     "type_asserts",
//!     "type_identity",
//!     "type_level_bool",
//!     "void",
//! ]
//! ```
//!
//! # Examples
//!
//! Showcasing some features from this crate.
//!
//! ### `quasiconst`, generic constants.
//!
//! The [`quasiconst`] macro allows emulating generic constants by generating a 
//! zero-sized generic type that implements the [`ConstVal`] trait,
//! the preferred way to get its value is the [`getconst`] macro.
//!
//! This example demonstrates how you can use them to declare a generic VTABLE constant.
//!
#![cfg_attr(not(feature = "const_val"), doc = " ```ignore")]
#![cfg_attr(feature = "const_val", doc = " ```rust")]
//! use core_extensions::{getconst, quasiconst};
//! 
//! use std::fmt::{self, Debug};
//! 
//! 
//! quasiconst!{
//!     pub const VTABLE<T: Debug>: &'static Vtable = &Vtable {
//!         size: std::mem::size_of::<T>(),
//!         align: std::mem::align_of::<T>(),
//!         drop: drop_erased::<T>,
//!         fmt: debug_fmt_erased::<T>,
//!     };
//! }
//! 
//! # fn main() {
//! const VTABLE_U8: &'static Vtable = getconst!(VTABLE<u8>);
//! assert_eq!(VTABLE_U8.size, 1);
//! assert_eq!(VTABLE_U8.align, 1);
//! 
//! const VTABLE_USIZE: &'static Vtable = getconst!(VTABLE<usize>);
//! assert_eq!(VTABLE_USIZE.size, std::mem::size_of::<usize>());
//! assert_eq!(VTABLE_USIZE.align, std::mem::align_of::<usize>());
//! 
//! const VTABLE_STRING: &'static Vtable = getconst!(VTABLE<&str>);
//! assert_eq!(VTABLE_STRING.size, std::mem::size_of::<usize>() * 2);
//! assert_eq!(VTABLE_STRING.align, std::mem::align_of::<usize>());
//! # }
//! 
//! 
//! 
//! pub struct Vtable {
//!     pub size: usize,
//!     pub align: usize,
//!     pub drop: unsafe fn(*mut ()),
//!     pub fmt: unsafe fn(*const (), &mut fmt::Formatter<'_>) -> fmt::Result,
//! }
//! 
//! unsafe fn drop_erased<T>(ptr: *mut ()) {
//!     std::ptr::drop_in_place(ptr as *mut T)
//! }
//! 
//! unsafe fn debug_fmt_erased<T>(ptr: *const (), f: &mut fmt::Formatter<'_>) -> fmt::Result 
//! where
//!     T: Debug,
//! {
//!     let this = unsafe{ &*(ptr as *const T) };
//!     
//!     Debug::fmt(this, f)
//! }
//! ```
//! <span id = "cargo-features-section"></span>
//! # Cargo Features
//!
//! ### Item features
//!
//! Item features enables items from this crate.
//!
//! The `"all_items"` feature enables all of these features,
//! you can use it instead of the ones below if you don't mind longer compile-times.
//!
//! The `"all_items_no_derive"` feature eanbles all the features below
//! except for the `"derive"` feature, 
//! to reduce compile-times due to enabling the `syn` indirect dependency.
//!
//! - `"derive"`: Enables derive macros for traits declared in core_extensions.
//! If a trait has a derive macro it'll mention and link to it.
//!
//! - `"bools"`: Enables the [`BoolExt`] trait, extension trait for `bool`.
//!
//! - `"callable"`: Enables the [`callable`] module, 
//! with stably implementable equivalents of the `Fn*` traits.
//!
//! - `"collections"`: Enables the [`collections`] module, with traits for collection types.
//!
//! - `"const_default"`:
//! Enables the [`ConstDefault`] trait, and [`const_default`] macro,
//! for a `const` equivalent of the `Default` trait.
//!
//! - `"const_val"`:
//! Enables the [`ConstVal`] trait (for types that represent constants), 
//! [`getconst`] macro (for getting the [`ConstVal::VAL`] associated constant),
//! and [`quasiconst`] macro (for declaring types that emulate generic constants).
//! Enables the `"generics_parsing"` feature.
//!
//! - `"macro_utils`:
//! Enables the [`rewrap_macro_parameters`], [`count_tts`], [`gen_ident_range`],
//! [`tokens_method`], [`compile_error_stringify`], and [`parenthesize_args`] macro.
//! Also enables the [`macro_attr`] attribute.
//!
//! - `"generics_parsing"`: 
//! Enables the [`parse_generics`], [`parse_generics_and_where`],
//! [`split_generics_and_where`], 
//! [`parse_split_generics`], and [`parse_split_generics_and_where`] macros.
//! These allow macros to parse items with generic parameters.
//!
//! - `"item_parsing"`: 
//! Enables the `"macro_utils` and `"generics_parsing"` features.
//! Enables the [`impl_parse_generics`] and [`impl_split`] macros.
//!
//! - `"integers"`: Enables the [`integers`] module, with extension traits for integer types.
//!
//! - `"iterators"`: Enables the [`iterators`] module, 
//! with the [`IteratorExt`] extension trait for iterators, and a few iterator types.
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
//! It primarily has methods for calling free functions as methods.
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
//! - `"void"`: Enables the [`Void`] type, a type that can't be constructed, 
//! for encodign impossible situations.
//!
//! <span id = "cargo-features-lang-section"></span>
//! ### Rust Version numbers
//!
//! These features enable code that require some Rust version past the minimum supported one:
//!
//! - "rust_1_46": Makes [`TransparentNewtype`] and [`TypeIdentity`]
//! associated functions that take `Rc<Self>` or `Arc<Self>` callable as methods.
//!
//! - "rust_1_51": Enables the "rust_1_46" feature, and impls of traits for all array lengths.
//!
//! - "rust_latest_stable":
//! Enables all the "rust_1_*" features.
//! This requires the last stable release of Rust,
//! since more `"rust_1_*"` features can be added at any time.
//!
//! ### Support for other crates
//!
//! All of these are disabled by default:
//!
//! - `"std"`: Enables `std` library support. Implies the `"alloc"` feature.
//!
//! - `"alloc"`: Enables `alloc` library support.
//!
//! - `"serde_"`: Enables serde support.
//!
//! ### Miscelaneous features
//!
//! `"track_caller"`:
//! Enables the "rust_1_46" feature.
//! Changes `ResultLike` to allow getting the caller location in `ResultLike::into_result_`,
//! and makes `IsNoneError` store where it was constructed.
//!
//! `"docsrs"`: Used to document the required features in docs.rs, requires Rust nightly.
//! Doesn't enable any items itself.
//!
//!
//! # no-std support
//!
//! This crate works in `#![no_std]` contexts by default.
//!
//! # Supported Rust versions
//!
//! This crate support Rust back to 1.41.0,
//! requiring cargo features to use language features from newer versions.
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
//! [`count_tts`]: ./macro.count_tts.html
//! [`gen_ident_range`]: ./macro.gen_ident_range.html
//! [`rewrap_macro_parameters`]: ./macro.rewrap_macro_parameters.html
//! [`tokens_method`]: ./macro.tokens_method.html
//! [`compile_error_stringify`]: ./macro.compile_error_stringify.html
//! [`parenthesize_args`]: ./macro.parenthesize_args.html
//! [`macro_attr`]: ./attr.macro_attr.html
//! [`parse_generics`]: ./macro.parse_generics.html
//! [`parse_generics_and_where`]: ./macro.parse_generics_and_where.html
//! [`split_generics_and_where`]: ./macro.split_generics_and_where.html
//! [`parse_split_generics`]: ./macro.parse_split_generics.html
//! [`parse_split_generics_and_where`]: ./macro.parse_split_generics_and_where.html
//!
//! [`impl_parse_generics`]: ./macro.impl_parse_generics.html
//! [`impl_split`]: ./macro.impl_split.html
//!
//! [`BoolExt`]: ./trait.BoolExt.html
//! [`ConstDefault`]: ./trait.ConstDefault.html
//! [`ConstVal`]: ./trait.ConstVal.html
//! [`ConstVal::VAL`]: ./trait.ConstDefault.html#associatedconstant.VAL
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
//! [`core`]: https://doc.rust-lang.org/core/
//! [`std`]: https://doc.rust-lang.org/std/
//! 

#![deny(missing_docs)]
#![deny(unused_must_use)]
#![cfg_attr(not(miri), no_std)]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]

#[cfg(feature="std")]
#[macro_use]
#[doc(hidden)]
pub extern crate std as std_;

#[cfg(not(feature="std"))]
#[doc(hidden)]
pub extern crate core as std_;

#[cfg(feature="alloc")]
#[doc(hidden)]
#[macro_use]
pub extern crate alloc;


#[doc(hidden)]
#[cfg(feature = "enable_proc_macro_crate")]
pub extern crate core_extensions_proc_macros;

#[cfg(feature = "derive")]
extern crate self as core_extensions;

#[cfg(all(feature = "derive", feature = "const_default"))]
include!{"./derive/const_default_docs.rs"}

#[cfg(all(feature = "derive", feature = "transparent_newtype"))]
include!{"./derive/transparent_newtype_docs.rs"}


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


#[cfg(feature = "macro_utils")]
#[doc(inline)]
pub use crate::macros::macro_utils::*;



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
    pub use std_::{concat, compile_error, stringify};
    pub use self::foo::Usize as usize;

    mod foo {
        pub type Usize = usize;
    }
    
    #[cfg(feature = "macro_utils")]
    pub use core_extensions_proc_macros::{__priv_rewrap_macro_parameters, count_tts};

    #[cfg(feature = "enable_proc_macro_crate")]
    pub use core_extensions_proc_macros::{__priv_unwrap_bound, __priv_split_generics};

    #[cfg(feature = "item_parsing")]
    pub use core_extensions_proc_macros::__priv_split_impl;
}


