This changelog is a summary of the changes made in each release.

This started being written for the 0.1.19 release, it doesn't cover versions prior to 0.1.6.

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
