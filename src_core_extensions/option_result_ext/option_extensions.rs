#[cfg(feature = "std")]
use std_::error;
use std_::fmt;

use super::ResultLike;
use type_identity::TypeIdentity;

/// Extension trait for [Option].
pub trait OptionExt<T>: ResultLike + TypeIdentity<Type = Option<T>> + Sized {
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
/// 
/// # Example
/// 
/// TODO
/// 
pub trait ToOption {
    /// The type in which the `Option`s are unwrapped.
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

macro_rules! for_tuple {
    ($($t:ident $i:tt),*) => {
        impl<$($t,)*> ToOption for ($(Option<$t>,)*) {
            type Output = ($($t,)*);

            fn to_option(self) -> Option<Self::Output> {
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