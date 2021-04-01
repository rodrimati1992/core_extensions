//! Helper types and functions for implementing Cloned.
#![allow(missing_docs)]

macro_rules! declare_clone_bounds {
    ($($docs:tt)*) => {
        $($docs)*
        #[cfg(not(feature = "alloc"))]
        pub trait CloneBound: std_::clone::Clone {}

        #[cfg(not(feature = "alloc"))]
        impl<T> CloneBound for T 
        where T: std_::clone::Clone
        {}

        $($docs)*
        #[cfg(feature = "alloc")]
        pub trait CloneBound: alloc::borrow::ToOwned {}

        #[cfg(feature = "alloc")]
        impl<T> CloneBound for T 
        where T: ?Sized + alloc::borrow::ToOwned
        {}
    };
}

declare_clone_bounds!{
    /// Trait alias for either [`Clone`] or [`ToOwned`] 
    /// depending on the "alloc" feature,
    /// used by the [`clone_this`] function
    /// 
    /// # Features
    /// 
    /// If the `"alloc"` feature is disabled then this aliases [`Clone`], 
    /// if it's enabled then this aliases [`ToOwned`].
    /// 
    /// 
    /// 
    /// [`clone_this`]: ./fn.clone_this.html
    /// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
    /// [`ToOwned`]: https://doc.rust-lang.org/std/borrow/trait.ToOwned.html
    /// 
    /// 
}

macro_rules! declare_cloned_type {
    ($($docs:tt)*) => {
        $($docs)*
        #[cfg(not(feature = "alloc"))]
        pub type CloneType<This> = This;

        $($docs)*
        #[cfg(feature = "alloc")]
        pub type CloneType<This> = <This as alloc::borrow::ToOwned>::Owned;
    }
}

declare_cloned_type!{
    /// The type that `This` is cloned into
    /// when using either [`Clone`] or [`ToOwned`].
    /// 
    /// # Features
    /// 
    /// If the `"alloc"` feature is disabled then this aliases `This`, 
    /// if it's enabled then this aliases `<This as ToOwned>::Owned`.
    /// 
    /// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
    /// [`ToOwned`]: https://doc.rust-lang.org/std/borrow/trait.ToOwned.html
}




macro_rules! declare_cloned_this {
    ($($docs:tt)*) => {
        $($docs)*
        #[cfg(not(feature = "alloc"))]
        pub fn clone_this<T>(this: &T) -> T
        where
            T: CloneBound,
        {
            this.clone()
        }

        $($docs)*
        #[cfg(feature = "alloc")]
        pub fn clone_this<T>(this: &T) -> T::Owned
        where
            T: ?Sized + CloneBound,
        {
            this.to_owned()
        }
    }
}


declare_cloned_this!{
    /// For cloning something with either [`Clone`] or [`ToOwned`] depending on 
    /// the "alloc" feature.
    /// 
    /// # Features
    /// 
    /// If the `"alloc"` feature is disabled then this requires [`Clone`], 
    /// if it's enabled this requires [`ToOwned`].
    ///
    /// [`ToOwned`] is implemented for all types that implement [`Clone`],
    /// and you can't call this function on `&str` or `&[T]` arguments
    /// without enabling the "alloc" feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use core_extensions::collections::clone_this;
    ///
    /// assert_eq!(clone_this(&0), 0);
    /// assert_eq!(clone_this(&"hello"), "hello");
    ///
    /// ```
    /// 
    /// With the "alloc" feature enabled you can clone `&str` into `String`,
    /// and `&[T]` into `Vec<T>`.
    ///
    #[cfg_attr(feature = "alloc", doc = " ```rust")]
    #[cfg_attr(not(feature = "alloc"), doc = " ```ignore")]
    /// use core_extensions::collections::clone_this;
    ///
    /// assert_eq!(clone_this("world"), "world".to_string());
    /// assert_eq!(clone_this(&[3, 5, 8][..]), vec![3, 5, 8]);
    ///
    /// ```
    /// 
    /// 
    /// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
    /// [`ToOwned`]: https://doc.rust-lang.org/std/borrow/trait.ToOwned.html
    /// [`core`]: https://doc.rust-lang.org/core
}

///////////////////////////////////////////////////////////////////////////////

use super::Cloned;

impl<'a, T> Cloned for &'a T
where
    T: ?Sized + CloneBound,
{
    type Cloned = CloneType<T>;

    fn cloned_(&self) -> Self::Cloned {
        clone_this(*self)
    }
}

impl<'a, T> Cloned for &'a mut T
where
    T: ?Sized + CloneBound,
{
    type Cloned = CloneType<T>;

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
    use alloc::string::ToString;

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
