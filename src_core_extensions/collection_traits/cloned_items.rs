//! Helper types and functions for implementing Cloned.


#[cfg(not(feature = "alloc"))]
pub use std_::clone::Clone as UsedCloneTrait;

#[cfg(feature = "alloc")]
pub use alloc_::borrow::ToOwned as UsedCloneTrait;

/// The type of each element in the cloned collection.
#[cfg(not(feature = "alloc"))]
pub type ClonedType<This>=This;

/// The type of each element in the cloned collection.
#[cfg(feature = "alloc")]
pub type ClonedType<This>=
    <This as UsedCloneTrait>::Owned;

/// For cloning each element in the collection.
#[cfg(not(feature = "alloc"))]
pub fn clone_this<T>(this:&T)->T
where
    T: UsedCloneTrait
{
    this.clone()
}

/// For cloning each element in the collection.
#[cfg(feature = "alloc")]
pub fn clone_this<T>(this:&T)->T::Owned
where
    T: ?Sized+UsedCloneTrait
{
    this.to_owned()
}

