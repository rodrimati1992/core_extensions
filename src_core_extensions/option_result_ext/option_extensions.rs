#[cfg(feature = "std")]
use std_::error;
use std_::fmt;

use super::ResultLike;
use type_identity::TypeIdentity;

/// Extension trait for [Option].
pub trait OptionExt<T>: ResultLike + TypeIdentity<Type = Option<T>> + Sized {
    /// Allows using Option::filter before Rust 1.27.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::OptionExt;
    ///
    /// let text="what the ";
    ///
    /// assert_eq!(Some(text).filter_(|x| x.len()==9 ).is_some(),true);
    ///
    /// assert_eq!(
    ///     text.split_whitespace().next()
    ///         .filter_(|x| x.len()==4 ),
    ///     Some("what"));
    ///
    /// assert_eq!(Some(text).filter_(|x| x.len()==20 ),None);
    ///
    /// assert_eq!(
    ///     text.split_whitespace().next()
    ///         .filter_(|x| x.len()==10 ),
    ///     None);
    ///
    /// ```
    ///
    #[inline]
    fn filter_<F>(self, predicate: F) -> Option<T>
    where
        F: FnOnce(&T) -> bool,
    {
        if let Some(v) = self.into_type_val() {
            if predicate(&v) {
                return Some(v);
            }
        }
        None
    }
    /// Maps as reference to the contents.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::OptionExt;
    ///
    /// struct User{
    ///     name:String,
    ///     surname:String,
    /// }
    ///
    /// let user=Some(User{name:"Matt".into(),surname:"Parker".into()});
    /// let name   =user.map_ref(|v| v.name.as_str() );
    /// let surname=user.map_ref(|v| v.surname.as_str() );
    ///
    /// assert_eq!(name,Some("Matt"));
    /// assert_eq!(surname,Some("Parker"));
    ///
    /// ```
    #[inline]
    fn map_ref<'a, U, F>(&'a self, f: F) -> Option<U>
    where
        T: 'a,
        F: FnOnce(&'a T) -> U,
    {
        self.into_type_ref().as_ref().map(f)
    }
    /// Maps as mutable reference to the contents.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::OptionExt;
    ///
    /// struct User{
    ///     name:String,
    ///     surname:String,
    /// }
    ///
    /// let mut user=Some(User{name:"Matt".into(),surname:"Parker".into()});
    /// {
    ///     let name   =user.map_mut(|v|{
    ///         v.name.push_str("hew") ;
    ///         v.name.as_str()
    ///     });
    ///     
    ///     assert_eq!(name,Some("Matthew"));
    /// }
    /// assert_eq!(user.unwrap().name,"Matthew");
    ///
    /// ```
    #[inline]
    fn map_mut<'a, U, F>(&'a mut self, f: F) -> Option<U>
    where
        T: 'a,
        F: FnOnce(&'a mut T) -> U,
    {
        self.into_type_mut().as_mut().map(f)
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
    fn to_result_(self) -> Result<Self::Item, Self::Error> {
        self.ok_or(IsNoneError)
    }
}

////////////////////////////////////////////////////////////////////////////////////

/// The [ResultLike::Error]
/// value for Option<T>
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IsNoneError;

impl fmt::Display for IsNoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("attempted to unwrap an Option that was None")
    }
}

#[cfg(feature = "std")]
impl error::Error for IsNoneError {
    fn description(&self) -> &str {
        "attempted to unwrap an Option that was None"
    }
}

////////////////////////////////////////////////////////////////////////////////////

/// Converts a type containing options into an option containing the type
pub trait ToOption {
    /// The type in which the `Option`s are unwrapped.
    ///
    /// Example:
    /// Self==(Option<i32>,Option<i32>)
    /// type Output=(i32,i32);
    type Output;
    /// Performs the conversion
    fn to_option(self) -> Option<Self::Output>;
}

impl<T> ToOption for Option<T> {
    type Output = T;
    fn to_option(self) -> Option<Self::Output> {
        self
    }
}

impl<T> ToOption for (Option<T>, Option<T>) {
    type Output = (T, T);

    fn to_option(self) -> Option<Self::Output> {
        Some((try_opt!(self.0), try_opt!(self.1)))
    }
}

impl<T> ToOption for (Option<T>, Option<T>, Option<T>) {
    type Output = (T, T, T);

    fn to_option(self) -> Option<Self::Output> {
        Some((try_opt!(self.0), try_opt!(self.1), try_opt!(self.2)))
    }
}

impl<T> ToOption for (Option<T>, Option<T>, Option<T>, Option<T>) {
    type Output = (T, T, T, T);

    fn to_option(self) -> Option<Self::Output> {
        Some((
            try_opt!(self.0),
            try_opt!(self.1),
            try_opt!(self.2),
            try_opt!(self.3),
        ))
    }
}

impl<T> ToOption for (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>) {
    type Output = (T, T, T, T, T);

    fn to_option(self) -> Option<Self::Output> {
        Some((
            try_opt!(self.0),
            try_opt!(self.1),
            try_opt!(self.2),
            try_opt!(self.3),
            try_opt!(self.4),
        ))
    }
}
