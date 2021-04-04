#[cfg(feature = "std")]
use std_::error;
use std_::fmt;

use super::ResultLike;
use type_identity::TypeIdentity;

/// Extension trait for [`Option`].
/// 
/// 
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
pub trait OptionExt<T>: ResultLike + TypeIdentity<Type = Option<T>> + Sized {
    /// Maps as reference to the contents.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::OptionExt;
    ///
    /// struct User{
    ///     name: String,
    ///     surname: String,
    /// }
    ///
    /// let user = Some(User{name: "Bob".to_string(), surname: "Math".to_string()});
    /// let name    = user.map_ref(|v| v.name.as_str() );
    /// let surname = user.map_ref(|v| v.surname.as_str() );
    ///
    /// assert_eq!(name, Some("Bob"));
    /// assert_eq!(surname, Some("Math"));
    ///
    /// ```
    #[inline]
    fn map_ref<'a, U, F>(&'a self, f: F) -> Option<U>
    where
        T: 'a,
        F: FnOnce(&'a T) -> U,
    {
        match self.as_type() {
            Some(x) => Some(f(x)),
            None => None,
        }
    }
    /// Maps as mutable reference to the contents.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::OptionExt;
    ///
    /// struct User{
    ///     name: String,
    ///     surname: String,
    /// }
    ///
    /// let mut user = Some(User{name: "Matt".into(), surname: "Parker".into()});
    /// {
    ///     let name = user.map_mut(|v|{
    ///         v.name.push_str("hew");
    ///         v.name.as_str()
    ///     });
    ///     
    ///     assert_eq!(name, Some("Matthew"));
    /// }
    /// assert_eq!(user.unwrap().name, "Matthew");
    ///
    /// ```
    #[inline]
    fn map_mut<'a, U, F>(&'a mut self, f: F) -> Option<U>
    where
        T: 'a,
        F: FnOnce(&'a mut T) -> U,
    {
        match self.as_type_mut() {
            Some(x) => Some(f(x)),
            None => None,
        }
    }
}

impl<T> OptionExt<T> for Option<T> {}

impl<T> ResultLike for Option<T> {
    type Item = T;
    type Error = IsNoneError;

    #[inline]
    fn is_item(&self) -> bool {
        self.is_some()
    }
    #[inline]
    #[cfg_attr(feature = "track_caller", track_caller)]
    fn into_result_(self) -> Result<Self::Item, Self::Error> {
        match self {
            Some(x) => Ok(x),
            None => Err(IsNoneError::new()),
        }
    }

    #[inline]
    fn from_item(item: Self::Item) -> Self {
        Some(item)
    }
    
    #[inline]
    fn from_error(_err: Self::Error) -> Self {
        None
    }
}

////////////////////////////////////////////////////////////////////////////////////

/// The [`ResultLike::Error`] for `Option`
/// 
/// [`ResultLike::Error`]: trait.ResultLike.html#associatedtype.Error
#[derive(Debug, Copy, Clone)]
pub struct IsNoneError (
    #[cfg(feature = "track_caller")]
    &'static std_::panic::Location<'static>,

    #[cfg(not(feature = "track_caller"))]
    (),
);

impl IsNoneError {
    /// Constructs an IsNoneError
    #[cfg_attr(feature = "track_caller", track_caller)]
    #[inline]
    pub fn new() -> Self {
        cfg_if!(
            (feature = "track_caller") {
                Self(std_::panic::Location::caller())
            } else {
                Self(())
            }
        )
    }
}

impl std_::cmp::PartialEq for IsNoneError {
    fn eq(&self, _: &IsNoneError) -> bool {
        true
    }
}

impl std_::cmp::Eq for IsNoneError {}

impl fmt::Display for IsNoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("attempted to unwrap an Option that was None")
    }
}

#[cfg(feature = "std")]
impl error::Error for IsNoneError {}

////////////////////////////////////////////////////////////////////////////////////

/// Converts a type containing options into an option containing the type
/// 
/// # Example
/// 
/// ```rust
/// use core_extensions::TransposeOption;
/// 
/// assert_eq!((Some(3), Some(5)).transpose_opt(), Some((3, 5)));
/// assert_eq!((Some(3), None::<u32>).transpose_opt(), None);
///
/// let ok_some: Result<Option<u32>, ()> = Ok(Some(8));
/// let ok_none: Result<Option<u32>, ()> = Ok(None);
/// let err: Result<Option<u32>, ()> = Err(());
///
/// assert_eq!(ok_some.transpose_opt(), Some(Ok(8)));
/// assert_eq!(ok_none.transpose_opt(), None);
/// assert_eq!(err.transpose_opt(), Some(Err(())));
///
/// ```
/// 
pub trait TransposeOption {
    /// The type in which the `Option`s are unwrapped.
    type Output;
    /// Performs the conversion
    fn transpose_opt(self) -> Option<Self::Output>;
}

impl<T> TransposeOption for Option<T> {
    type Output = T;
    #[inline]
    fn transpose_opt(self) -> Option<Self::Output> {
        self
    }
}

impl<T, E> TransposeOption for Result<Option<T>, E> {
    type Output = Result<T, E>;
    #[inline]
    fn transpose_opt(self) -> Option<Result<T, E>> {
        self.transpose()
    }
}

macro_rules! for_tuple {
    ($($t:ident $i:tt),*) => {
        impl<$($t,)*> TransposeOption for ($(Option<$t>,)*) {
            type Output = ($($t,)*);

            #[inline]
            fn transpose_opt(self) -> Option<Self::Output> {
                Some(($(self.$i?,)*))
            }
        }
    };
}

for_tuple!{A 0}
for_tuple!{A 0, B 1}
for_tuple!{A 0, B 1, C 2}
for_tuple!{A 0, B 1, C 2, D 3}
for_tuple!{A 0, B 1, C 2, D 3, E 4}