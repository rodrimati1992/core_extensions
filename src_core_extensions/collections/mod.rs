/*!
Extension traits for collection types.
*/

mod cloned_items;

mod array_impls;

mod tuple_impls;

///////////////////////////////////////////////////////////////////////////////


pub use self::cloned_items::{CloneBound, CloneType, clone_this};

///////////////////////////////////////////////////////////////////////////////

/// Clones a collection of references into a collection of values.
///
/// # Features
///
/// Enabling the "alloc" or "std" features changes the impl for references from
/// using [`Clone`] bounds to using [`ToOwned`].
///
/// Enabling the "const_generics" feature allows arrays of all lengths to implement this trait,
/// otherwise it's only implemented for arrays up to 32 elements long.
///
/// [`ToOwned`] is implemented for all types that implement [`Clone`],
/// and is not declared in the [`core`] crate,
/// which means that you can't call `cloned_` on `(&str,)` or `(&[T],)`
/// without enabling either the "alloc" or "std" features.
///
/// # Examples
///
/// ### Tuples
///
/// ```rust
/// use core_extensions::collections::Cloned;
///
/// assert_eq!((&2,).cloned_(), (2,));
/// assert_eq!((&2, &3).cloned_(), (2, 3));
/// assert_eq!((&2, &3, &5).cloned_(), (2, 3, 5));
/// assert_eq!((&2, &3, &5, &8).cloned_(), (2, 3, 5, 8));
///
/// ```
///
/// ### Arrays
///
/// ```rust
/// use core_extensions::collections::Cloned;
///
/// assert_eq!([&13].cloned_(), [13]);
/// assert_eq!([&13, &21].cloned_(), [13, 21]);
/// assert_eq!([&13, &21, &34].cloned_(), [13, 21, 34]);
/// assert_eq!([&13, &21, &34, &55].cloned_(), [13, 21, 34, 55]);
///
/// ```
/// 
/// ### "alloc" feature
///
/// This demonstrates how `&str` and `&[T]` elements can be cloned with the "alloc" feature
/// 
/// ```rust
/// use core_extensions::collections::Cloned;
/// 
/// assert_eq!(["foo"].cloned_(), ["foo".to_string()]);
/// assert_eq!(["bar", "baz"].cloned_(), ["bar".to_string(), "baz".to_string()]);
/// 
/// assert_eq!((&[3, 5, 8][..],).cloned_(), (vec![3, 5, 8],));
/// assert_eq!((&[13, 21][..], &[34, 55][..]).cloned_(), (vec![13, 21], vec![34, 55]));
/// 
/// ```
///
/// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
/// [`ToOwned`]: https://doc.rust-lang.org/std/borrow/trait.ToOwned.html
/// [`core`]: https://doc.rust-lang.org/core
///
pub trait Cloned {
    /// The type of this with owned values instead of references to them.
    type Cloned;

    /// Clones a collection of references into a collection of values.
    fn cloned_(&self) -> Self::Cloned;
}

/// The type that `This` is cloned into, with [`Cloned::cloned_`].
///
/// [`Cloned::cloned_`]: ./trait.Cloned.html#tymethod.cloned_
/// 
///
///
pub type ClonedOut<This> = <This as Cloned>::Cloned;

///////////////////////////////////////////////////////////////////////////////




///////////////////////////////////////////////////////////////////////////////

/// Converts a fixed length collection to an array.
///
/// # Features
/// 
/// Enabling the "const_generics" feature allows arrays of all lengths to implement this trait,
/// otherwise it's only implemented for arrays up to 32 elements long.
/// 
/// # Examples
///
/// ### Tuples
///
/// ```
/// use core_extensions::collections::IntoArray;
///
/// assert_eq!((2,).into_array(), [2]);
/// assert_eq!((2, 3).into_array(), [2, 3]);
/// assert_eq!((2, 3, 5).into_array(), [2, 3, 5]);
/// assert_eq!((2, 3, 5, 8).into_array(), [2, 3, 5, 8]);
///
/// ```
///
/// ### Arrays
///
/// ```rust
/// use core_extensions::collections::IntoArray;
///
/// assert_eq!([13].into_array(), [13]);
/// assert_eq!([13, 21].into_array(), [13, 21]);
/// assert_eq!([13, 21, 34].into_array(), [13, 21, 34]);
/// assert_eq!([13, 21, 34, 55].into_array(), [13, 21, 34, 55]);
///
/// ```
/// 
pub trait IntoArray {
    /// The type of the array of the same length.
    type Array;

    /// Converts the tuple to an array..
    fn into_array(self) -> Self::Array;
}

///////////////////////////////////////////////////////////////////////////////
