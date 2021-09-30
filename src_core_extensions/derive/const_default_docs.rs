/// Derives the [`ConstDefault`] trait for structs and enums.
/// 
/// [For examples look here](#examples)
/// 
/// For enums, this requires a `#[cdef(default)]` attribute on exactly one variant.
/// 
/// # Default behavior
/// 
/// By default, this derive macro generates a [`ConstDefault`] impl with:
/// - [`ConstDefault`] bounds on all type parameters.
/// - [`ConstDefault::DEFAULT`] as the value of all the fields.
/// 
/// # Attributes
/// 
/// ### Container attributes
/// 
/// Attributes used above the type definition.
/// 
/// `#[cdef(crate = foo::bar)]`([example](#crate-example)): <br>
/// Replaces the path to `core_extensions` with `foo::bar`
/// 
/// `#[cdef(bound(T: Foo + Bar))]`([example](#bound-example)): <br>
/// Replaces the default bound (`ConstDefault`) of the T type parameter with
/// the passed-in bounds.<br>
/// `#[cdef(bound(T: ))]` is allowed.
/// 
/// `#[cdef(no_bounds)]`([example](#no_bounds-example)): <br>
/// Removes the `ConstDefault` bound for all type parameters
/// 
/// `#[cdef(field_bound)]`([example](#field_bound-example)): <br>
/// Removes the `ConstDefault` bound for type parameters,
/// replacing them with `ConstDefault` bounds on all of the field types.
/// 
/// `#[cdef(where T: Foo + Bar)]`([example](#where-example)): <br>
/// Adds arbitrary bounds to the `ConstDefault` impl.
/// 
/// `#[cdef(debug_print)]`: <br>
/// For diagnostics, causes the derive macro to panic with the code generated by it.
/// 
/// ### Variant attributes
/// 
/// `#[cdef(default)]`([example](#default-value-example)): <br>
/// Uses that variant for the default value.
/// This must be used on exactly one variant.
/// 
/// ### Field attributes
/// 
/// `#[cdef(default = <expression>)]`([example](#default-value-example)): <br>
/// Replaces the default value of the field ([`ConstDefault::DEFAULT`]) with `<expression>`,
/// which must be usable in a const context.
/// 
/// `#[cdef(field_bound)]`([example](#field_bound_field-example)): <br>
/// Adds a [`ConstDefault`] bound for the field type.
/// 
/// # Examples
/// 
/// ### Basic struct
/// 
/// ```rust
/// use core_extensions::ConstDefault;
/// 
/// #[derive(Debug, PartialEq, ConstDefault)]
/// struct Foo {
///     bar: u32,
///     baz: Option<String>,
/// }
/// 
/// assert_eq!(Foo::DEFAULT, Foo{bar: 0, baz: None});
/// 
/// ```
/// 
/// ### Basic enum
/// 
/// ```rust
/// use core_extensions::ConstDefault;
/// 
/// #[derive(Debug, PartialEq, ConstDefault)]
/// enum Foo {
///     Bar(u32),
///     #[cdef(default)]
///     Baz(Option<String>),
/// }
/// 
/// assert_eq!(Foo::DEFAULT, Foo::Baz(None));
/// 
/// ```
/// 
/// <a id = "crate-example"></a>
/// ### Crate renaming
/// 
/// This example demonstrates how the `core_extensions` crate can be renamed,
/// passing the new name to the derive macro.
/// 
/// ```rust
/// # extern crate core_extensions as cext;
/// # extern crate std as core_extensions;
/// #
/// use cext::ConstDefault;
/// 
/// #[derive(Debug, PartialEq, ConstDefault)]
/// #[cdef(crate = cext)]
/// struct Foo {
///     bar: u32,
///     baz: Option<String>,
/// }
/// 
/// # fn main() {
/// assert_eq!(Foo::DEFAULT, Foo{bar: 0, baz: None});
/// # }
/// ```
/// 
/// <a id = "default-value-example"></a>
/// ### Different default value
/// 
/// This example demonstrates replacing the default value for one field.
/// The assigned expression can be anything, so long as it's usable in a const context.
/// 
/// ```rust
/// use core_extensions::ConstDefault;
/// 
/// #[derive(Debug, PartialEq, ConstDefault)]
/// struct Foo {
///     #[cdef(default = power(5))]
///     bar: u32,
///     baz: Option<String>,
/// }
/// 
/// const fn power(n: u32) -> u32 {
///     1 << n
/// }
/// 
/// assert_eq!(Foo::DEFAULT, Foo{bar: 32, baz: None});
/// ```
/// 
/// <a id = "no_bounds-example"></a>
/// ### No Bounds
/// 
/// This example demonstrates removing the default `ConstDefault` bound on all type parameters.
/// 
/// ```rust
/// use core_extensions::ConstDefault;
/// 
/// use std::cmp::Ordering;
/// use std::marker::PhantomData;
/// 
/// #[derive(Debug, PartialEq)]
/// struct NoDefault<T>(T);
/// 
/// 
/// #[derive(Debug, PartialEq, ConstDefault)]
/// // removes the default `ConstDefault` bound on all type parameters
/// #[cdef(no_bounds)]
/// struct NoBounds<T, U, V: 'static> {
///     bar: Option<T>,
///     baz: PhantomData<U>,
///     qux: &'static [V],
/// }
/// 
/// assert_eq!(
///     NoBounds::<Ordering, NoDefault<u32>, NoDefault<String>>::DEFAULT,
///     NoBounds{bar: None, baz: PhantomData, qux: &[]}
/// );
/// ```
/// 
/// <a id = "bound-example"></a>
/// ### Replaced Bounds
/// 
/// This example demonstrates replacing the default `ConstDefault` bound on 
/// type parameters with other bounds.
/// 
/// ```rust
/// use core_extensions::ConstDefault;
/// 
/// use std::marker::PhantomData;
/// 
/// #[derive(Debug, PartialEq)]
/// struct NoDefault<T>(T);
/// 
/// #[derive(Debug, PartialEq, ConstDefault)]
/// // replaces the default `ConstDefault` bound on the `T` type parameter with no bounds.
/// #[cdef(bound(T: ))]
/// // replaces the default bound on `U` with `Copy + ConstDefault`
/// #[cdef(bound(U: Copy + ConstDefault))]
/// struct PartialBounds<T, U> {
///     bar: PhantomData<T>,
///     baz: U,
/// }
/// 
/// let def = PartialBounds::<NoDefault<()>, u32>::DEFAULT;
/// assert_eq!(def, PartialBounds{bar: PhantomData, baz: 0});
/// 
/// ```
/// 
/// <a id = "field_bound-example"></a>
/// ### Field Bounds
/// 
/// This example demonstrates how the default `ConstDefault` bound on 
/// type parameters can be replaced with `ConstDefault` bounds on field types.
/// 
/// ```rust
/// use core_extensions::ConstDefault;
///
/// use std::marker::PhantomData;
/// 
/// #[derive(Debug, PartialEq, ConstDefault)]
/// // replaces the default `T: ConstDefault` and `U: ConstDefault` bounds with 
/// // bounds on the types of the fields:
/// // `PhantomData<T>: ConstDefault` and `Custom<U>: ConstDefault`
/// #[cdef(field_bound)]
/// struct FieldBounds<T, U> {
///     bar: PhantomData<T>,
///     baz: Custom<U>,
/// }
/// 
/// let def = FieldBounds::<NoDefault<u8>, i32>::DEFAULT;
/// assert_eq!(def, FieldBounds{bar: PhantomData, baz: Custom(0)});
/// 
/// 
/// #[derive(Debug, PartialEq, ConstDefault)]
/// #[cdef(bound(T: ConstDefault + Copy))]
/// struct Custom<T>(T);
/// 
/// #[derive(Debug, PartialEq)]
/// struct NoDefault<T>(T);
/// ```
/// 
/// <a id = "field_bound_field-example"></a>
/// ### Field Bound
/// 
/// This example demonstrates how the `ConstDefault` bound can be required 
/// for only some field types.
/// 
/// ```rust
/// use core_extensions::ConstDefault;
///
/// use std::marker::PhantomData;
/// 
/// #[derive(Debug, PartialEq, ConstDefault)]
/// // removes the default `T: ConstDefault` and `U: ConstDefault` bounds
/// #[cdef(no_bounds)]
/// struct FieldBound<T, U> {
///     bar: PhantomData<T>,
///     // Adds a `Custom<U>: ConstDefault` bound 
///     #[cdef(field_bound)]
///     baz: Custom<U>,
/// }
/// 
/// let def = FieldBound::<NoDefault<u8>, i32>::DEFAULT;
/// assert_eq!(def, FieldBound{bar: PhantomData, baz: Custom(0)});
/// 
/// 
/// #[derive(Debug, PartialEq, ConstDefault)]
/// #[cdef(bound(T: ConstDefault + Copy))]
/// struct Custom<T>(T);
/// 
/// #[derive(Debug, PartialEq)]
/// struct NoDefault<T>(T);
/// ```
/// 
/// <a id = "where-example"></a>
/// ### Extra bounds
/// 
/// This example demonstrates how additional bounds can be put in the
/// `ConstDefault` impl.
/// 
/// ```rust
/// use core_extensions::ConstDefault;
///
/// #[derive(Debug, PartialEq, ConstDefault)]
/// // Adds `T: Copy` and `u128: From<T>` bounds to the ConstDefault impl.
/// #[cdef(where T: Copy, u128: From<T>)]
/// struct ExtraBounds<T>(T);
/// 
/// assert_eq!(ExtraBounds::<u8>::DEFAULT, ExtraBounds(0));
/// assert_eq!(ExtraBounds::<u32>::DEFAULT, ExtraBounds(0));
/// ```
/// 
/// [`ConstDefault::DEFAULT`]: ./trait.ConstDefault.html#associatedconstant.DEFAULT
/// [`ConstDefault`]: ./trait.ConstDefault.html
#[cfg_attr(feature = "docsrs", doc(cfg(all(feature = "derive", feature = "const_default"))))]
pub use core_extensions_proc_macros::ConstDefault;