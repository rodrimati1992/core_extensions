use std_::mem;

#[cfg(feature = "alloc")]
use alloc_::{
    boxed::Box,
    rc::Rc,
    sync::Arc,
};

use crate::utils::{self, transmute_ignore_size};

/// Allows converting `Self` to
/// [`Self::Type`](./trait.TypeIdentity.html#associatedtype.Type)
/// by proving that both types are equal.
///
/// # Usecases
///
/// This trait allows:
/// 
/// - Defining extension traits without repeating method signatures.
///
/// - Creating a type Alias in a where clause, eg: `Vec<i32>: TypeIdentity<Type = List>`.
///
/// - Unwrapping a generic type, eg: `I: TypeIdentity<Type = Option<U>>`.
///
/// # Example
///
/// Defining an extension trait on Vec<T>.
///
/// ```
/// use core_extensions::TypeIdentity;
///
/// trait VecExt<T>: TypeIdentity<Type = Vec<T>> {
///     fn is_nonempty(&self) -> bool {
///         !self.as_type().is_empty()
///     }
///     fn moved_vec(self) -> Vec<T>
///     where Self: Sized
///     {
///         self.into_type()
///     }
///     fn mutable_vec(&mut self) -> &mut Vec<T> {
///         self.as_type_mut()
///     }
/// }
/// impl<T> VecExt<T> for Vec<T> {}
///
/// assert!( vec![100].is_nonempty());
/// assert!(!Vec::<i32>::new().is_nonempty());
/// ```
///
///
///
///
/// # Example of a method requiring `Self == Other`
/// 
/// `Wrapper::iter` is only callable on `Wrapper<Vec<T>>`
///
/// ```
/// use core_extensions::TypeIdentity;
/// use std::slice;
///
/// struct Wrapper<U>(U);
///
/// impl<U> Wrapper<U> {
///      fn iter<T>(&self) -> slice::Iter<T>
///      where U: TypeIdentity<Type = Vec<T>>
///      {
///          self.0.as_type().iter()
///      }
/// }
///
/// assert_eq!(
///     Wrapper(vec![0,1,2,3,4]).iter().cloned().collect::<Vec<_>>() ,
///     vec![0,1,2,3,4]
/// );
///
/// ```
///
///
/// # Example of creating a type alias in a `where` clause
///
/// ```rust
/// use core_extensions::TypeIdentity;
/// use std::ops::Deref;
///
/// struct Example<T>(T);
///
/// impl<T, Target0, Target1> Deref for Example<T>
/// where
///     T: Deref,
///     <T as Deref>::Target: TypeIdentity<Type = Target0>,
///     Target0: Deref,
///     <Target0 as Deref>::Target: TypeIdentity<Type = Target1>,
/// {   
///     type Target=Target1;
///     
///     fn deref(&self) -> &Target1 {
///         &**self
///     }
/// }
///
/// ```
///
///
pub trait TypeIdentity {
    /// This is always `Self`.
    type Type: ?Sized;

    /// Converts a value back to the original type.
    #[inline(always)]
    fn into_type(self) -> Self::Type
    where
        Self: Sized,
        Self::Type: Sized,
    {
        unsafe { transmute_ignore_size(self) }
    }
    /// Converts a reference back to the original type.
    #[inline(always)]
    fn as_type(&self) -> &Self::Type {
        unsafe { mem::transmute_copy::<&Self, &Self::Type>(&self) }
    }
    /// Converts a mutable reference back to the original type.
    #[inline(always)]
    fn as_type_mut(&mut self) -> &mut Self::Type {
        unsafe { mem::transmute_copy::<&mut Self, &mut Self::Type>(&self) }
    }
    /// Converts a box back to the original type.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
    #[inline(always)]
    fn into_type_box(self: Box<Self>) -> Box<Self::Type> {
        unsafe { utils::transmute_ignore_size(self) }
    }
    /// Converts an Arc back to the original type.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
    #[inline(always)]
    fn into_type_arc(self: Arc<Self>) -> Arc<Self::Type> {
        unsafe { utils::transmute_ignore_size(self) }
    }
    /// Converts an Rc back to the original type.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
    #[inline(always)]
    fn into_type_rc(self: Rc<Self>) -> Rc<Self::Type> {
        unsafe { utils::transmute_ignore_size(self) }
    }

    /// Converts a value back to the original type.
    #[inline(always)]
    fn from_type_val(this: Self::Type) -> Self
    where
        Self: Sized,
        Self::Type: Sized,
    {
        let this = mem::ManuallyDrop::new(this);
        unsafe { mem::transmute_copy::<Self::Type, Self>(&*this) }
    }
    /// Converts a reference back to the original type.
    #[inline(always)]
    fn from_type_ref(this: &Self::Type) -> &Self {
        unsafe { mem::transmute_copy::<&Self::Type, &Self>(&this) }
    }
    /// Converts a mutable reference back to the original type.
    #[inline(always)]
    fn from_type_mut(this: &mut Self::Type) -> &mut Self {
        unsafe { mem::transmute_copy::<&mut Self::Type, &mut Self>(&this) }
    }
    /// Converts a box back to the original type.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
    #[inline(always)]
    fn from_type_box(this: Box<Self::Type>) -> Box<Self> {
        unsafe { utils::transmute_ignore_size(this) }
    }
    /// Converts an Arc back to the original type.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
    #[inline(always)]
    fn from_type_arc(this: Arc<Self::Type>) -> Arc<Self> {
        unsafe { utils::transmute_ignore_size(this) }
    }
    /// Converts an Rc back to the original type.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
    #[inline(always)]
    fn from_type_rc(this: Rc<Self::Type>) -> Rc<Self> {
        unsafe { utils::transmute_ignore_size(this) }
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Prevents creating a trait object of this trait
    fn _dummy_generic_method_preventing_trait_object<F>(self: &Self)
    where
        F: TypeIdentity<Type = Self>,
    {

    }
}

impl<T: ?Sized> TypeIdentity for T {
    type Type = T;
}

/// A type-level identity function
pub type TIdentity<Type> = <Type as TypeIdentity>::Type;
