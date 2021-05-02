This changelog is a summary of the changes made in each release.

This started being written for the 0.1.19 release, it doesn't cover versions prior to 0.1.6.

# 1.0

### 1.4.0

Added the `macro_attr` attribute macro, under the `"macro_utils"` feature.

Added the `tokens_method`, `compile_error_stringify`, and `parenthesize_args` macros, under the `"macro_utils"` feature.

Changed range syntax in `gen_ident_range`, to allow `..X`/`X..`/`..=X`/`..` ranges (this is also used by newer macros).

### 1.3.3

Fixed failing release builds due to conditionally compiled code.

### 1.3.0

Added the `count_tts` and `gen_ident_range` macros, under the `"macro_utils"` feature.

### 1.2.0

Added `"macro_utils"` feature.

Added `"item_parsing"` feature, which enables both `"macro_utils"` and `"generics_parsing"`.

Added these `rewrap_macro_parameters` macros under the `"macro_utils"` feature

Added `"impl_split"`, and `"impl_parse_generics"` macros, under the `"item_parsing"` feature.

Added `"parse_split_generics"`, and `parse_split_generics_and_where`, under the `"generics_parsing"` feature.

Changed bounds output by macros to always have a trailing `+`.

Changed where predicates output by core_extensions macros to always have a trailing `,`.


### 1.1.0

Added `parse_generics`, `parse_generics_and_where`, and `split_generics_and_where` macros under the new `"generics_parsing"` feature.

Added new syntax for `quasiconst`, which uses `<....>` for generic parameters (instead of `[....]`), and does not require `[....]` for where clauses. Looking like generic constants would look in Rust.


### 1.0.0

Rewrote `TransparentNewtype` to be more resilient to soundness issues, adding methods for casting raw pointers between the newtype and the wrapped type, moving all its previous methods to `TransparentNewtypeExt`.

Added `TransparentNewtypeExt`, which now has all the methods for
casting references and std smart pointers(`Box`,`Rc`,and `Arc`) between `Self` and `Self::Inner`.

Renamed `TransparentNewtype::as_inner_ref` to `as_inner`

Added `transparent_newtype::{into_inner_vec, from_inner_vec}` functions to go between `Vec<T>` and `Vec<<T as TransparentNewtype>::Inner>`
    
Added impls of `TransparentNewtype` for:
- `core::num::Wrapping`
- `core::mem::ManuallyDrop`
- slices

Added `impl_transparent_newtype` macro to implement `TransparentNewtype`.

Added `NEW` associated constants to all `AssertEq*` types.

Renamed `ToOption` to `TransposeOption`, renamed `to_option` method to `transpose_opt`,added impl for `Result<Option<T>, E>`.

Made `measure_time` module not depend on a `std` feature.

Removed `MyDuration`, since `std::time::Duration` got feature parity with it.

Removed `OptionExt::filter_`, since it's an inherent method before 1.41.0

Removed `SelfOps::{T, assert_ty*, ty_*}` associated items.

Changed `SelfOps::{as_ref_, as_mut_, into_}` to require using the turbofish operator.

Renamed `phantom_variances` module to `phantom`.

Renamed `VariantPhantom` to `CovariantPhantom`

Added `phantom::ContraVariantPhantom` type alias

Added `phantom::AsPhantomData` trait to get the `PhantomData` of a type.

Added `phantom::{as_covariant_phantom, as_phantom}` functions to get the `PhantomData` of a value.

Added `CovariantPhantomData`newtype wrapper as a workaround to make `as_covariant_phantom` a const fn.

Added these macros in the macros::phantomdata module:
- `map_phantomdata`
- `expr_as_phantom`
- `return_type_phantom`

Added `phantom::{AndPhantom, AndPhantomCov}` structs.


Added `Debug`, `Copy`, and `Clone` derives to `LazyOnce`.

Added `IteratorExt::{sum_same, product_same}` methods.

Changed `IteratorExt` impl to be for `?Sized` types

Turned `IterConstructor` and `IterCloner` into tuple structs, removing their `new` constructors.

Added `CallExt` to be able to call the `Call*::*_call` methods with turbofish syntax,
exporting `CallExt` in the root module.

Renamed:
```rust
CallRef::call_ref as ref_call_
CallMut::call_mut as mut_call_
CallInto::call_into as into_call_
```
Rewrote and renamed `callable_impl` macro to `impl_call`.

Made the `impl_call` macro require a `self` argument, with just a comma separating it from the rest of the parameters.

Renamed `collection_traits` to `collections`

Made `collections::cloned_items` module private, exposing its items in `collections`.
    
Renamed `collections::ClonedType` to `CloneType`
    
Renamed `UsedCloneTrait` to `CloneBound` and turned it into a trait alias.

Made `bool_extensions` module private, exposing its items in the root module.

Renamed `integer_extensions` module to `integers`.

Added `ZERO`, `ONE`, `MIN`, `MAX` associated constants to `IntegerExt`

Added `std::{ops, fmt}`, `Default`, `Hash`, `Send`, `Sync`, and `'static` as supertraits of `IntegerExt`

Renamed `IntegerExt::div_safe` to `safe_div`.

Renamed `ToTime::miliseconds` to `milliseconds`

Made `IsNoneError` require calling `IsNoneError::new` to construct, adding the location it was constructed on when the `"track_caller"` feature is enabled.

Moved unwrap methods to a new `ResultLikeExt` trait, which is blanket implemented for `ResultLike`.

Renamed
```rust
ResultExt::to_result_ as into_result_
ResultExt::unwrap_safe as into_item
ResultExt::unwrap_err_safe`as into_error
ResultExt::unwrap_unchecked`as unwrap_unchecked_
ResultExt::unwrap_err_unchecked`as unwrap_err_unchecked_
```

Added `ResultLike::{from_result_, from_item, from_error}` methods.

Added `ResultLikeExt::{unwrap_or_, unwrap_err_or_, unwrap_or_else_, unwrap_err_or_else}` methods.


Turned type parameter of `SliceExt` to an `Elem` associated type, removed type parameter of `ValSliceExt`.

Made slices of zero sized types considered contained by another slice if they have the same address, and aren't empty.

Removed `KeySlice::slice` method.

Removed `StringExt::{get_offset_inside_of, offset_inside_of}` (they're already in `SliceExt`).

Fixed `StringExt::char_indices_to` to skip the last character if the end bound is in the middle of it.
    
Changed `StringExt::{min_indentation, max_indentation}` to ignore purely whitespace lines.

Removed `KeyStr::str` method

Added `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `ConstDefault`, and `Display` impls for `True` and `False`.
    
Removed `Boolean::{Not, if_false, if_true}`

Added `ConstDefault` supertrait to Boolean

Removed `BooleanOp`, `IfTrueOrFalse`, `IfElseOp`, `Nand`, `Nor`, `Xnor`, `Implies`, `NonImp`, and `IfElse`.

Made the `matches` like `std::matches`, added match guard support.

Made `Rc` and `Arc` methods in `TypeIdentity` and `TransparentNewtype` take `self` with the "rust_1_46" feature, otherwise taking a `this` parameter.

Added `ConstVal` trait for types that have a single associated constant, and `getconst` macro to get it. 

Added `quasiconst` macro to define types that emulate generic cosntants.

Implemented `From<Void> for std_::convert::Infallible` and `From<std_::convert::Infallible> for Void`

Renamed 
```rust
TypeIdentity::into_type_val as into_type
TypeIdentity::into_type_ref as as_type
TypeIdentity::into_type_mut as as_type_mut
TypeIdentity::from_type_val as from_type
```

Made only `PhantomData` or `#[repr(transparent))]` wrappers around `MarkerType` types implement MarkerType.

Removed `OptionExt::filter_`

Added `utils::transmute_vec` function.

Updated `utils::impossible` to use std::hint::unreachable_unchecked in release builds.

Removed `utils::{as_slice, as_slice_mut}`, because `std::slice::{from_ref. from_mut}` exist.

Made the `"std"` feature opt-in, only supporting the `core` crate by default.

Made virtually all items opt-in, with these features(enabling all with an `"all_items"` feature): 
- `"bools"`
- `"callable"`
- `"collections"`: renamed from "coltraits"
- `"const_default"`
- `"const_val"`
- `"integers"`
- `"iterators"`
- `"marker_type"`
- `"on_drop"`
- `"option_result"`
- `"phantom"`
- `"self_ops"`
- `"slices"`
- `"strings"`
- `"transparent_newtype"`
- `"type_asserts"`
- `"type_identity"`
- `"type_level_bool"`
- `"void"`

Added these features to manually enable functionality that requires a Rust version:
- `"rust_1_46"`
- `"rust_1_46"`

Added these miscelaneuous features
- `"track_caller"`
- `"docsrs"`

Bumped MSRV to Rust 1.41.0

Replaced the `"const_generics"` features with a `"rust_1_51"` feature.

Removed `maybe_unsafe`, and `type_panic` module.

Removed `typenum` support.

Removed conditional compilation attributes for items that require Rust versions below 1.41.0

Removed build script, which did Rust version detection.

Declared and added dependency on the `core_extensions_proc_macros` crate, used for low-level token munging. This crate has no dependencies.

# 0.1

### 0.1.19

Added the "const_generics" and "nightly_const_generics" features.

Added support for const_generics, by implementing the `Cloned`, `ConstDefault`, and `MarkerType` traits for all array lengths when the "const_generics" feature is enabled.

Added utils::RunOnDrop type, to run code at the end of a scope.

Fixed the MSRV for the `MarkerType` impl for `ManuallyDrop` to be Rust 1.22.

Fixed implementations of  SliceExt::{contains_slice, get_index_of, and get_offset_of_slice methods},
the bugs in those were discovered after writing more complete tests.

### 0.1.17

Added support for cross-compiling to powerpc architecture,
by making some impls for `Atomic*` types conditional.

### 0.1.16 

Added example of implementing ConstDefault

Fixed minimum version support, was importing AtomicU8,AtomicI8 in 1.24 instead of 1.34,
causing a compiler error because those were unstable back in 1.24.

### 0.1.15

Added ConstDefault trait, a const equivalent of the Default trait.

Added MarkerType impl for ManuallyDrop (enabled on Rust 1.29).

### 0.1.14

Added Cloned impls for `&T`,`&mut T`,`Option`,`Result`

Changed tuple and array impls of `Cloned` to have non-reference type parameters, allowing any type that implements `Cloned`.

### 0.1.13

Made collection traits visible in docs without enabling the "colltraits" feature,
while still requiring the "colltraits" feature to enable the impls.

### 0.1.12

Added alloc crate support.

Added "colltraits" cargo feature.

Added `ClonedTuple` and IntoArray traits for tuples and arrays, requiring the "colltraits" feature to use.

### 0.1.11

Added constructors for `AssertEq` types.

### 0.1.10

Added AssertEq/AssertEq3/AssertEq4 types, for doing a type level assertion that two types are the same.

### 0.1.9

Removed `serde_derive` dependency

Fixed unsound `transparent_newtype` module documentation

### 0.1.7

Added the `slices::ValSliceExt` trait, with `split_while` and `rsplit_while` methods.

### 0.1.6

Fixed use of lint that stop the crate from compiling.
