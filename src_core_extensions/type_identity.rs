use std_::mem;

#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(feature = "std")]
use std::sync::Arc;

use utils::transmute_ignore_size;

/// Allows converting `Self` to
/// [`Self::Type`](./trait.Identity.html#associatedtype.Type)
/// by proving that both types are equal.
///
/// For extension methods of types,
/// either generic (`Vec<T>`) or fully concrete (`str`),
/// to avoid repeating method and function signatures in both the trait and the impl block.
///
/// To create a type Alias in a where clause ,eg:` Vec<i32>:TypeIdentity<Type=List> `.
///
/// To unwrap a generic type ,eg:` I:TypeIdentity<Type=Option<U>> `.
///
/// # Example
/// Defining an extension trait on Vec<T>.
///
/// ```
/// use core_extensions::TypeIdentity;
///
/// trait VecExt<T>:TypeIdentity<Type=Vec<T>>{
///     fn is_nonempty(&self)->bool{
///         !self.into_type_ref().is_empty()
///     }
///     fn moved_vec(self)->Vec<T>
///     where Self:Sized
///     {
///         self.into_type_val()
///     }
///     fn mutable_vec(&mut self)->&mut Vec<T>{
///         self.into_type_mut()
///     }
/// }
/// impl<T> VecExt<T> for Vec<T> {}
///
/// assert!(  vec![100].is_nonempty() );
/// assert!( !Vec::<i32>::new().is_nonempty() );
/// ```
///
///
///
///
/// # Example of a method requiring Self==Other
/// Wrapper::iter is only callable on Wrapper\<Vec\<T>>
///
/// ```
/// use core_extensions::TypeIdentity;
/// use std::slice;
///
/// struct Wrapper<U>(U);
///
/// impl<U> Wrapper<U>{
///      fn iter<T>(&self)->slice::Iter<T>
///      where U:TypeIdentity<Type=Vec<T>>
///      {
///          self.0.into_type_ref().iter()
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
/// # Example of creating a type alias in a where clause
///
/// ```
/// use core_extensions::TypeIdentity;
/// use std::ops::Deref;
///
/// struct Example<T>(T);
///
/// impl<T,Target0,Target1> Deref for Example<T>
/// where
///     T:Deref,
///     <T as Deref>::Target:TypeIdentity<Type=Target0>,
///     Target0:Deref,
///     <Target0 as Deref>::Target:TypeIdentity<Type=Target1>,
/// {   
///     type Target=Target1;
///     
///     fn deref(&self)->&Target1{
///         &**self
///     }
/// }
///
/// ```
///
///
pub trait TypeIdentity {
    /// The same type as Self.
    ///
    /// Used in bounds to require that a generic type is a particular type.
    type Type: ?Sized;
    /// Converts a value back to the original type.
    #[inline(always)]
    fn into_type_val(self) -> Self::Type
    where
        Self: Sized,
        Self::Type: Sized,
    {
        unsafe { transmute_ignore_size(self) }
    }
    /// Converts a reference back to the original type.
    #[inline(always)]
    fn into_type_ref(&self) -> &Self::Type {
        unsafe { mem::transmute_copy::<&Self, &Self::Type>(&self) }
    }
    /// Converts a mutable reference back to the original type.
    #[inline(always)]
    fn into_type_mut(&mut self) -> &mut Self::Type {
        unsafe { mem::transmute_copy::<&mut Self, &mut Self::Type>(&self) }
    }
    /// Converts a box back to the original type.
    #[cfg(feature = "std")]
    #[inline(always)]
    fn into_type_box(self: Box<Self>) -> Box<Self::Type> {
        unsafe { transmute_ignore_size(self) }
    }
    /// Converts an Arc back to the original type.
    #[cfg(feature = "std")]
    #[inline(always)]
    fn into_type_arc(this: Arc<Self>) -> Arc<Self::Type> {
        unsafe { transmute_ignore_size(this) }
    }
    /// Converts an Rc back to the original type.
    #[cfg(feature = "std")]
    #[inline(always)]
    fn into_type_rc(this: Rc<Self>) -> Rc<Self::Type> {
        unsafe { transmute_ignore_size(this) }
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
    #[cfg(feature = "std")]
    #[inline(always)]
    fn from_type_box(this: Box<Self::Type>) -> Box<Self> {
        unsafe { transmute_ignore_size(this) }
    }
    /// Converts an Arc back to the original type.
    #[cfg(feature = "std")]
    #[inline(always)]
    fn from_type_arc(this: Arc<Self::Type>) -> Arc<Self> {
        unsafe { transmute_ignore_size(this) }
    }
    /// Converts an Rc back to the original type.
    #[cfg(feature = "std")]
    #[inline(always)]
    fn from_type_rc(this: Rc<Self::Type>) -> Rc<Self> {
        unsafe { transmute_ignore_size(this) }
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
