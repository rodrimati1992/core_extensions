[![Rust](https://github.com/rodrimati1992/core_extensions/workflows/Rust/badge.svg)](https://github.com/rodrimati1992/core_extensions/actions)
[![crates.io](https://img.shields.io/crates/v/core_extensions.svg)](https://crates.io/crates/core_extensions)
[![api-docs](https://docs.rs/core_extensions/badge.svg)](https://docs.rs/core_extensions/*)


Extension traits for many standard/core library types/traits.
and other miscelaneuous types / traits / functions / macros.

# Adding as dependency

This crate requires cargo features for enabling items, to get all of them you can use:

```toml
[dependencies.core_extensions]
version = "1.0"
features = ["std", "all_items"]
```
The "std" feature is required to enable impls and items that use [`std`] types,
otherwise only the [`core`] library is supported.

For enabling features individually, [look here](#cargo-features-section).

This crate currently [requires cargo features](#cargo-features-lang-section)
to use newer language features,

# Examples

Showcasing some features from this crate.

### `quasiconst`, generic constants.

The [`quasiconst`] macro allows emulating generic constants by generating a 
zero-sized generic type that implements the [`ConstVal`] trait,
the preferred way to get its value is the [`getconst`] macro.

This example demonstrates how you can use them to declare a generic VTABLE constant.

```rust
use core_extensions::{getconst, quasiconst};

use std::fmt::{self, Debug};


quasiconst!{
    pub const VTABLE[T: Debug]: &'static Vtable = &Vtable {
        size: std::mem::size_of::<T>(),
        align: std::mem::align_of::<T>(),
        drop: drop_erased::<T>,
        fmt: debug_fmt_erased::<T>,
    };
}

fn main() {
    const VTABLE_U8: &'static Vtable = getconst!(VTABLE<u8>);
    assert_eq!(VTABLE_U8.size, 1);
    assert_eq!(VTABLE_U8.align, 1);

    const VTABLE_USIZE: &'static Vtable = getconst!(VTABLE<usize>);
    assert_eq!(VTABLE_USIZE.size, std::mem::size_of::<usize>());
    assert_eq!(VTABLE_USIZE.align, std::mem::align_of::<usize>());

    const VTABLE_STRING: &'static Vtable = getconst!(VTABLE<&str>);
    assert_eq!(VTABLE_STRING.size, std::mem::size_of::<usize>() * 2);
    assert_eq!(VTABLE_STRING.align, std::mem::align_of::<usize>());
}



pub struct Vtable {
    pub size: usize,
    pub align: usize,
    pub drop: unsafe fn(*mut ()),
    pub fmt: unsafe fn(*const (), &mut fmt::Formatter<'_>) -> fmt::Result,
}

unsafe fn drop_erased<T>(ptr: *mut ()) {
    std::ptr::drop_in_place(ptr as *mut T)
}

unsafe fn debug_fmt_erased<T>(ptr: *const (), f: &mut fmt::Formatter<'_>) -> fmt::Result 
where
    T: Debug,
{
    let this = unsafe{ &*(ptr as *const T) };
    
    Debug::fmt(this, f)
}
```

# no-std support

This crate works in `#![no_std]` contexts by default.

# Supported Rust versions

This crate support Rust back to 1.41.0,
requiring cargo features to use language features from newer versions.

<span id = "cargo-features-section"></span>
# Cargo Features

### crate features

The `"all_items"` feature enables all of these features,
you can use it instead of the ones below if you don't mind longer compile-times:

- `"bools"`: Enables the [`BoolExt`] trait, extension trait for `bool`.

- `"callable"`: Enables the [`callable`] module, 
with stably implementable equivalents of the `Fn*` traits.

- `"collections"`: Enables the [`collections`] module, with traits for collection types.

- `"const_default"`:
Enables the [`ConstDefault`] trait, and [`const_default`] macro.

- `"const_val"`:
Enables the [`ConstVal`] trait, [`getconst`] macro, and [`quasiconst`] macro.

- `"integers"`: Enables the [`integers`] module, with extension traits for integer types.

- `"iterators"`: Enables the [`iterators`] module, 
with the [`IteratorExt`] extension trait for iterators, and iterator types.

- `"marker_type"`: Enables the [`MarkerType`] trait,
for trivially constructible, zero-sized, and aligned-to-1 types.

- `"on_drop"`: Enables the [`RunOnDrop`] type,
a wrapper type that runs a closure at the end of the scope.

- `"option_result"`: Enables the [`option_result_ext`] module,
with traits for `Option` and `Result`-like types.

- `"phantom"`: Enables the [`phantom`] module(with `PhantomData`-related items),
[`expr_as_phantom`] macro,[`map_phantomdata`] macro, and [`return_type_phantom`] macro.

- `"self_ops"`: Enables the [`SelfOps`] trait, an extension trait for all types.

- `"slices"`:
Enables the [`slices`] module, with extension traits for `[T]` and `str` slices.

- `"strings"`:
Enables the [`strings`] module, with the [`StringExt`] extension trait for strings.

- `"transparent_newtype"`: Enables the [`transparent_newtype`] module,
with extension traits and functions for `#[repr(transparent)]` newtypes with public fields.

- `"type_asserts"`: Enables the [`type_asserts`] module, with type-level assertiosn,
most useful in tests.

- `"type_identity"`: Enables the [`TypeIdentity`] trait,
for proving that two types are equal, and converting between them in a generic context.

- `"type_level_bool"`: Enables the [`type_level_bool`] module,
which encodes `bool`s on the type-level.

- `"void"`: Enables the [`Void`] type, for impossible situations.

<span id = "cargo-features-lang-section"></span>
### Rust Version numbers

These features enable code that require some Rust version past the minimum supported one:

- "rust_1_46": Makes [`TransparentNewtype`] and [`TypeIdentity`]
associated functions that take `Rc<Self>` or `Arc<Self>` callable as methods.

- "rust_1_51": Enables the "rust_1_46" feature, and impls of traits for all array lengths.

### Support for other crates

`"std"`: Enables `std` library support. Enabled by default. Implies the `"alloc"` feature.

`"alloc"`: Enables `alloc` library support. Enabled by default.

`"serde_"`: Enables serde support. Disabled by default.

### Miscelaneous features

`"track_caller"`:
Enables the "rust_1_46" feature.
Changes `ResultLike` to allow getting the caller location in `ResultLike::into_result_`,
and makes `IsNoneError` store where it was constructed.

`"docsrs"`: Used to document the required features in docs.rs, requires Rust nightly.
Doesn't enable any items itself.


# License

core_extensions is licensed under either of

    Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in core_extensions by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.



[`collections`]: https://docs.rs/core_extensions/1.*/collections/index.html
[`callable`]: https://docs.rs/core_extensions/1.*/callable/index.html
[`integers`]: https://docs.rs/core_extensions/1.*/integers/index.html
[`iterators`]: https://docs.rs/core_extensions/1.*/iterators/index.html
[`option_result_ext`]: https://docs.rs/core_extensions/1.*/option_result_ext/index.html
[`phantom`]: https://docs.rs/core_extensions/1.*/phantom/index.html
[`slices`]: https://docs.rs/core_extensions/1.*/slices/index.html
[`strings`]: https://docs.rs/core_extensions/1.*/strings/index.html
[`transparent_newtype`]: https://docs.rs/core_extensions/1.*/transparent_newtype/index.html
[`type_asserts`]: https://docs.rs/core_extensions/1.*/type_asserts/index.html
[`type_level_bool`]: https://docs.rs/core_extensions/1.*/type_level_bool/index.html

[`BoolExt`]: https://docs.rs/core_extensions/1.*/trait.BoolExt.html
[`ConstDefault`]: https://docs.rs/core_extensions/1.*/trait.ConstDefault.html
[`ConstVal`]: https://docs.rs/core_extensions/1.*/trait.ConstVal.html
[`MarkerType`]: https://docs.rs/core_extensions/1.*/trait.MarkerType.html
[`SelfOps`]: https://docs.rs/core_extensions/1.*/trait.SelfOps.html
[`TypeIdentity`]: https://docs.rs/core_extensions/1.*/trait.TypeIdentity.html
[`TransparentNewtype`]: https://docs.rs/core_extensions/1.*/transparent_newtype/trait.TransparentNewtype.html

[`RunOnDrop`]: https://docs.rs/core_extensions/1.*/struct.RunOnDrop.html
[`Void`]: https://docs.rs/core_extensions/1.*/enum.Void.html

[`const_default`]: https://docs.rs/core_extensions/1.*/macro.const_default.html
[`getconst`]: https://docs.rs/core_extensions/1.*/macro.getconst.html
[`quasiconst`]: https://docs.rs/core_extensions/1.*/macro.quasiconst.html
[`expr_as_phantom`]: https://docs.rs/core_extensions/1.*/macro.expr_as_phantom.html
[`map_phantomdata`]: https://docs.rs/core_extensions/1.*/macro.map_phantomdata.html
[`return_type_phantom`]: https://docs.rs/core_extensions/1.*/macro.return_type_phantom.html

[`IteratorExt`]: https://docs.rs/core_extensions/1.*/iterators/trait.IteratorExt.html
[`StringExt`]: https://docs.rs/core_extensions/1.*/strings/trait.StringExt.html

[`core`]: https://doc.rust-lang.org/core/
[`std`]: https://doc.rust-lang.org/std/

