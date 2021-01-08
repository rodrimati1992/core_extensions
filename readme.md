[![Rust](https://github.com/rodrimati1992/core_extensions/workflows/Rust/badge.svg)](https://github.com/rodrimati1992/core_extensions/actions)
[![crates.io](https://img.shields.io/crates/v/core_extensions.svg)](https://crates.io/crates/core_extensions)
[![api-docs](https://docs.rs/core_extensions/badge.svg)](https://docs.rs/core_extensions/*)


This crate provides many extensions to core/std library types,
and provides some new abstractions.

This crate provides:

- Extension traits for strings/slices/Option/Result/bool.

- SelfOps: to extend all types with generic operations.

- Type-level representations of bool and unsafe  .

- Marker traits to encode invariants about types.

- etc.


# no-std support

To use this crate in no_std contexts disable the default-feature.

# Supported Rust versions

This crate support Rust back to 1.20,
using a build script to automatically enable features from newer versions.

# Cargo Features

- `"std"`:Enables standard library support.Enabled by default.

- `"serde_"`:Enables serde support.Disabled by default.

- `"typenum"`:Enables trait impls for typenum types.Disabled by default.

- `"colltraits"`:Enables trait impls for the collection traits in the `collection_traits` module.

- `"const_generics"`: Enables impls of traits for all array lengths, requires versions of Rust where const generics are stable.

- `"nightly_const_generics"`: Enables impls of traits for all array lengths in Rust nightly versions prior to the stabilization of const generics.

# License

core_extensions is licensed under either of

    Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in core_extensions by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
