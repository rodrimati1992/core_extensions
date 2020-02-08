/*!
Extension traits implemented for multiple types
*/

#[cfg(feature = "colltraits")]
pub mod cloned_items;

#[cfg(feature = "colltraits")]
mod array_impls;

#[cfg(feature = "colltraits")]
mod tuple_impls;

///////////////////////////////////////////////////////////////////////////////

/// Clones a collection of references into a collection of values.
///
/// # Features
///
/// This trait is only implemented
/// if the "colltraits" cargo feature is enabled.
///
/// Enabling the "alloc" or "std" features changes the impl for references from
/// using `Clone` bounds to using `ToOwned`
/// (`ToOwned` is declared in the `alloc` crate).
///
/// `ToOwned` is implemented for all types that implement `Clone`,
/// and is not declared in `core`,
/// which means that you can't call `cloned_` on `(&str,)` or `(&[T],)`
/// without enabling either the "alloc" or "std" features.
///
/// # Tuple Example
///
#[cfg_attr(feature = "colltraits", doc = " ```")]
#[cfg_attr(not(feature = "colltraits"), doc = " ```ignore")]
/// use core_extensions::collection_traits::Cloned;
///
/// assert_eq!( (&2,).cloned_(), (2,) );
/// assert_eq!( (&2,&3).cloned_(), (2,3) );
/// assert_eq!( (&2,&3,&5).cloned_(), (2,3,5) );
/// assert_eq!( (&2,&3,&5,&8).cloned_(), (2,3,5,8) );
///
/// ```
///
pub trait Cloned {
    /// The type of this with owned values instead of references to them.
    type Cloned;

    /// Clones a tuple of references into a tuple of values.
    fn cloned_(&self) -> Self::Cloned;
}

/// The type that `This` is cloned into.
///
/// Eg: `&u32` is cloned into `u32`,
/// `&str` is cloned into `String` (this example requires the "alloc" feature).
pub type ClonedOut<This> = <This as Cloned>::Cloned;

///////////////////////////////////////////////////////////////////////////////

/// Converts a fixed length collection to an array.
///
/// # Features
///
/// This trait is only implemented
/// if the "colltraits" cargo feature is enabled.
///
/// # Example
///
#[cfg_attr(feature = "colltraits", doc = " ```")]
#[cfg_attr(not(feature = "colltraits"), doc = " ```ignore")]
/// use core_extensions::collection_traits::IntoArray;
///
/// assert_eq!( (2,).into_array(), [2] );
/// assert_eq!( (2,3).into_array(), [2,3] );
/// assert_eq!( (2,3,5).into_array(), [2,3,5] );
/// assert_eq!( (2,3,5,8).into_array(), [2,3,5,8] );
///
/// ```
///
///
pub trait IntoArray {
    /// The type of the array of the same length as the tuple
    type Array;

    /// Converts the tuple to an array..
    fn into_array(self) -> Self::Array;
}

///////////////////////////////////////////////////////////////////////////////
