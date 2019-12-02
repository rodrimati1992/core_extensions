/*!
Extension traits implemented for multiple types
*/

pub mod cloned_items;

mod array_impls;
mod tuple_impls;


///////////////////////////////////////////////////////////////////////////////

/// Clones a fixed-sized collection of references into a collection of values.
///
/// # How features affect the trait impls
///
/// Enabling the "alloc" or "std" features changes the impl from this crate from
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
/// ```
/// use core_extensions::collection_traits::Cloned;
///
/// assert_eq!( (&2,).cloned_(), (2,) );
/// assert_eq!( (&2,&3).cloned_(), (2,3) );
/// assert_eq!( (&2,&3,&5).cloned_(), (2,3,5) );
/// assert_eq!( (&2,&3,&5,&8).cloned_(), (2,3,5,8) );
///
/// ```
///
pub trait Cloned{
    /// The type of this tuple with owned values instead of references to them.
    type Cloned;

    /// Clones a tuple of references into a tuple of values.
    fn cloned_(&self)->Self::Cloned;
}

///////////////////////////////////////////////////////////////////////////////


/// Converts a fixed length collection to an array.
///
/// # Example
///
/// ```
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
pub trait IntoArray{
    /// The type of the array of the same length as the tuple
    type Array;

    /// Converts the tuple to an array..
    fn into_array(self)->Self::Array;
}




///////////////////////////////////////////////////////////////////////////////


