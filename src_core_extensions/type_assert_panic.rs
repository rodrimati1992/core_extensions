mod sealed{
    pub trait Sealed{}

}

use self::sealed::Sealed;


/// A trait used to cause a compile-time error mentioning the types constrained by it.
///
/// This trait is usable when there are generic parameters,
/// to give the user an error message telling them why an impl is not usable. 
///
/// This trait is unimplementable.
///
pub trait TypePanic:Sealed{}



