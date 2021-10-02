/// Derives the [`TransparentNewtype`] trait.
/// 
/// This derive macro requires the type to be a `struct` with a `#[repr(transparent)]` 
/// representation.
/// 
/// If the type has multiple fields, the derive macro errors when 
/// the non-`#[twrap]`-annotated fields don't ìmplement the [`MarkerType`] trait
/// (which ensures that those fields are all zero-sized).
/// 
/// For examples [look below](#examples)
/// 
/// # Attributes
/// 
/// ### Container attributes
/// 
/// Attributes used above the type definition.
/// 
/// `#[twrap(crate = foo::bar)]`([example](#crate-example)): <br>
/// Replaces the path to `core_extensions` with `foo::bar`
/// 
/// `#[twrap(where T: Foo + Bar)]`([example](#where-example)): <br>
/// Adds arbitrary bounds to the `TransparentNewtype` impl.
/// 
/// `#[twrap(debug_print)]`: <br>
/// For diagnostics, causes the derive macro to panic with the code generated by it.
/// 
/// ### Field attributes
/// 
/// `#[twrap]`([example](#twrap-field-example)): <br>
/// Tells the derive macro that this is the wrapped field, and other fields are zero-sized.
/// 
/// `#[twrap(delegate)]`([example](#twrap-delegate-example)): <br>
/// Tells the derive macro to delegate the [`TransparentNewtype`] impl to this field.
/// <br>
/// This adds a `TypeOfThisField: TransparentNewtype` bound to the implementation.
/// 
/// # Examples
/// 
/// ### Single-field struct
/// 
/// ```rust
/// use core_extensions::{TransparentNewtype, TransparentNewtypeExt};
/// 
/// #[derive(Debug, PartialEq, TransparentNewtype)]
/// #[repr(transparent)]
/// struct SingleField<T>(T);
/// 
/// assert_eq!(SingleField::from_inner(3), SingleField(3));
/// assert_eq!(SingleField::from_inner_ref(&5), &SingleField(5));
/// assert_eq!(SingleField::from_inner_mut(&mut 8), &mut SingleField(8));
/// ```
/// 
/// <a id = "twrap-field-example"></a>
/// ### Many-field struct
/// 
/// ```rust
/// use core_extensions::{TransparentNewtype, TransparentNewtypeExt};
/// 
/// use std::marker::PhantomData;
/// 
/// #[derive(Debug, PartialEq, TransparentNewtype)]
/// #[repr(transparent)]
/// struct WithPhantom<T, U>{
///     ghost: PhantomData<U>,
///     // you need to annotate the wrapped field when this has zero-sized fields.
///     #[twrap]
///     value: T,
/// }
/// 
/// assert_eq!(
///     WithPhantom::<_, &str>::from_inner(13), 
///     WithPhantom{value: 13, ghost: PhantomData},
/// );
/// assert_eq!(
///     WithPhantom::<_, String>::from_inner_ref(&21),
///     &WithPhantom{value: 21, ghost: PhantomData},
/// );
/// assert_eq!(
///     WithPhantom::<_, Vec<u8>>::from_inner_mut(&mut 34),
///     &mut WithPhantom{value: 34, ghost: PhantomData},
/// );
/// ```
/// 
/// <a id = "twrap-delegate-example"></a>
/// ### Delegating to type parameter
/// 
/// This example demonstrates how you can delegate the `TransparentNewtype` implementation
/// to a field.
/// 
/// ```rust
/// use core_extensions::{TransparentNewtype, TransparentNewtypeExt};
/// 
/// use std::num::Wrapping;
/// use std::mem::ManuallyDrop as MD;
/// 
/// #[derive(Debug, PartialEq, TransparentNewtype)]
/// #[repr(transparent)]
/// struct Foo<T>(
///     #[twrap(delegate)]
///     T,
/// );
/// 
/// assert_eq!(<Foo<Wrapping<u8>>>::from_inner(13), Foo(Wrapping(13)));
/// assert_eq!(<Foo<Wrapping<u16>>>::from_inner_ref(&21), &Foo(Wrapping(21)));
/// assert_eq!(<Foo<Wrapping<u32>>>::from_inner_mut(&mut 34), &mut Foo(Wrapping(34)));
/// 
/// assert_eq!(<Foo<MD<u8>>>::from_inner(55), Foo(MD::new(55)));
/// assert_eq!(<Foo<MD<u16>>>::from_inner_ref(&89), &Foo(MD::new(89)));
/// assert_eq!(<Foo<MD<u32>>>::from_inner_mut(&mut 144), &mut Foo(MD::new(144)));
/// ```
///
/// <a id = "crate-example"></a>
/// ### Crate attribute
/// 
/// This example demonstrates how you can use this derive macro when this crate is renamed.
/// 
/// ```rust
/// # extern crate core_extensions as cext;
/// # extern crate std as core_extensions;
/// #
/// use cext::{TransparentNewtype, TransparentNewtypeExt};
/// 
/// #[derive(Debug, PartialEq, TransparentNewtype)]
/// #[twrap(crate = cext)]
/// #[repr(transparent)]
/// struct SingleField<T>(T);
/// 
/// assert_eq!(SingleField::from_inner(3), SingleField(3));
/// assert_eq!(SingleField::from_inner_ref(&5), &SingleField(5));
/// assert_eq!(SingleField::from_inner_mut(&mut 8), &mut SingleField(8));
/// ```
/// 
/// <a id = "where-example"></a>
/// ### Additional bounds
/// 
/// This example demonstrates how you can make the [`TransparentNewtype`] impl conditional.
/// 
/// ```rust
/// use core_extensions::{TransparentNewtype, TransparentNewtypeExt};
/// 
/// #[derive(Debug, PartialEq, TransparentNewtype)]
/// #[twrap(where T: Copy)]
/// #[repr(transparent)]
/// struct CopyWrapper<T>{
///     val: T
/// }
/// 
/// assert_eq!(CopyWrapper::from_inner(3), CopyWrapper{val: 3});
/// assert_eq!(CopyWrapper::from_inner_ref(&5), &CopyWrapper{val: 5});
/// assert_eq!(CopyWrapper::from_inner_mut(&mut 8), &mut CopyWrapper{val: 8});
/// ```
/// 
/// This doesn't compile because `CopyWrapper` requires the wrapped type to be `Copy`:
/// 
/// ```compile_fail
/// # use core_extensions::{TransparentNewtype, TransparentNewtypeExt};
/// # 
/// # #[derive(Debug, PartialEq, TransparentNewtype)]
/// # #[twrap(where T: Copy)]
/// # #[repr(transparent)]
/// # struct CopyWrapper<T>(T);
/// # 
/// CopyWrapper::from_inner(String::new());
/// ```
/// 
/// [`TransparentNewtype`]: ./transparent_newtype/trait.TransparentNewtype.html
/// [`MarkerType`]: ./trait.MarkerType.html
#[cfg_attr(feature = "docsrs", doc(cfg(all(feature = "derive", feature = "transparent_newtype"))))]
pub use core_extensions_proc_macros::TransparentNewtype;