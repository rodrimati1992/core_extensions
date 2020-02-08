//! Helper types and functions for implementing Cloned.

#[cfg(not(feature = "alloc"))]
pub use std_::clone::Clone as UsedCloneTrait;

#[cfg(feature = "alloc")]
pub use alloc_::borrow::ToOwned as UsedCloneTrait;

/// The type of each element in the cloned collection.
#[cfg(not(feature = "alloc"))]
pub type ClonedType<This> = This;

/// The type of each element in the cloned collection.
#[cfg(feature = "alloc")]
pub type ClonedType<This> = <This as UsedCloneTrait>::Owned;

/// For cloning each element in the collection.
#[cfg(not(feature = "alloc"))]
pub fn clone_this<T>(this: &T) -> T
where
    T: UsedCloneTrait,
{
    this.clone()
}

/// For cloning each element in the collection.
#[cfg(feature = "alloc")]
pub fn clone_this<T>(this: &T) -> T::Owned
where
    T: ?Sized + UsedCloneTrait,
{
    this.to_owned()
}

///////////////////////////////////////////////////////////////////////////////

use super::Cloned;

impl<'a, T> Cloned for &'a T
where
    T: ?Sized + UsedCloneTrait,
{
    type Cloned = ClonedType<T>;

    fn cloned_(&self) -> Self::Cloned {
        clone_this(*self)
    }
}

impl<'a, T> Cloned for &'a mut T
where
    T: ?Sized + UsedCloneTrait,
{
    type Cloned = ClonedType<T>;

    fn cloned_(&self) -> Self::Cloned {
        clone_this(*self)
    }
}

impl<T> Cloned for Option<T>
where
    T: Cloned,
{
    type Cloned = Option<T::Cloned>;

    fn cloned_(&self) -> Self::Cloned {
        match *self {
            Some(ref x) => Some(x.cloned_()),
            None => None,
        }
    }
}

impl<T, E> Cloned for Result<T, E>
where
    T: Cloned,
    E: Cloned,
{
    type Cloned = Result<T::Cloned, E::Cloned>;

    fn cloned_(&self) -> Self::Cloned {
        match *self {
            Ok(ref x) => Ok(x.cloned_()),
            Err(ref x) => Err(x.cloned_()),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc_::string::ToString;

    #[test]
    fn refs() {
        assert_eq!((&8).cloned_(), 8);
        assert_eq!((&mut 13).cloned_(), 13);
        assert_eq!(<&u8 as Cloned>::cloned_(&&21), 21);
        assert_eq!(<&mut u8 as Cloned>::cloned_(&&mut 34), 34);

        #[cfg(feature = "alloc")]
        {
            assert_eq!("5".cloned_(), "5".to_string());
            assert_eq!((&"8").cloned_(), "8");
            assert_eq!((&mut "13").cloned_(), "13");
            assert_eq!(<&str as Cloned>::cloned_(&"17"), "17".to_string());
            assert_eq!(<&&str as Cloned>::cloned_(&&"21"), "21");
            assert_eq!(<&mut &str as Cloned>::cloned_(&&mut "34"), "34");
        }
    }

    #[test]
    fn options() {
        assert_eq!(None::<&()>.cloned_(), None);
        assert_eq!(Some(&3).cloned_(), Some(3));

        #[cfg(feature = "alloc")]
        {
            assert_eq!(Some("5").cloned_(), Some("5".to_string()));
        }

        assert_eq!((Some(&3), Some(&5)).cloned_(), (Some(3), Some(5)));
        assert_eq!((Some(&mut 3), Some(&mut 5)).cloned_(), (Some(3), Some(5)));
    }

    #[test]
    fn results() {
        assert_eq!(Ok::<&u8, &u8>(&13).cloned_(), Ok(13));
        assert_eq!(Err::<&u8, &u8>(&21).cloned_(), Err(21));

        assert_eq!(Ok::<Option<&u8>, &()>(Some(&21)).cloned_(), Ok(Some(21)));
        assert_eq!(Err::<&(), Option<&u8>>(Some(&34)).cloned_(), Err(Some(34)));
        assert_eq!(
            Err::<&(), Option<(&u8, &bool)>>(Some((&34, &false))).cloned_(),
            Err(Some((34, false)))
        );
    }
}
