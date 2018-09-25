//! Module containing definitions and operations for type level booleans
//!
//! All operations are type aliases for convenience
//!
//!
//! # Example
//! 
//! Access of privileges based on renamed Boolean types.
//!
//! ```
//! use std::mem;
//! use std::marker::PhantomData;
//! use core_extensions::VariantPhantom;
//! pub use core_extensions::type_level_bool::{
//!     Boolean as PrivilegeLevel,
//!     False as Unprivileged ,
//!     True as Privileged ,
//! };
//! 
//! 
//!
//! #[derive(Debug)]
//! struct User<P>{
//!     name:String,
//!     privilege_level:VariantPhantom<P>,
//! }
//!
//! impl<P:PrivilegeLevel> User<P>{
//!     fn new(name:String,privilege_level:P)->Self{
//!         Self{ name , privilege_level:PhantomData }
//!     }
//!     fn name(&self)->&str{ 
//!         &self.name
//!     }
//!     fn into_unprivileged(self)->User<Unprivileged>{
//!         User{ name:self.name , privilege_level:PhantomData }
//!     }
//!     fn as_unprivileged(self:&mut User<P>)->&mut User<Unprivileged>{
//!         // Only the type parameter P changes here,which is itself a MarkerType.
//!         unsafe{ mem::transmute(self) }
//!     }
//! }
//!
//! impl User<Privileged>{
//!     fn set_name(&mut self,name:String){
//!         self.name=name;
//!     }
//! }
//!
//! let mut user:User<Privileged>=
//!     User::new("bob".into(),Privileged);
//! assert_eq!(user.name(),"bob");
//!
//! user.set_name("paul".into());
//! assert_eq!(user.name(),"paul");
//!
//! {
//!     let user:&mut User<Unprivileged>=
//!         user.as_unprivileged();
//! 
//!     // Unprivileged Users can't change their name.
//!     // user.set_name("james".into()); 
//! 
//!     assert_eq!(user.name(),"paul");
//! }
//!
//! user.set_name("john".into());
//! assert_eq!(user.name(),"john");
//!
//! ```
//!
//!


use VariantPhantom;


use std_::fmt::Debug;
use std_::ops;

use ::marker_traits::MarkerType;
/// Represents a type-level `true`
#[derive(Debug,Copy,Clone,Default)]
pub struct True ;


/// Represents a type-level `false`
#[derive(Debug,Copy,Clone,Default)]
pub struct False ;

/// Used to represent a type-level boolean when used as a type parameter.
#[derive(Debug,Copy,Clone,Default)]
pub struct BooleanType;


mod sealed{
    use super::{True,False};
    pub trait Sealed{}
    impl Sealed for True{}
    impl Sealed for False{}
}
use self::sealed::Sealed;

unsafe impl MarkerType for True{}
unsafe impl MarkerType for False{}

/// Represents a type-level `bool`
///
/// Only implemented on [True] and [False].
///
/// For examples look at [the module-level documentation](./index.html).
///
/// This trait is sealed and cannot be implemented for types outside this crate.
pub trait Boolean: 
    Sealed+
    MarkerType+
    Default+
    Sized+
    Debug+
    Copy+Clone+
    ops::BitAnd<True>+ops::BitAnd<False>+
    ops::BitOr <True>+ops::BitOr <False>+
    ops::BitXor<True>+ops::BitXor<False>+
    ops::Not+
{
    /// The negation of this type.
    type Not: Boolean<Not = Self>;
    /// The [bool] value of this type
    const VALUE: bool;

    /// If Self==True,runs the closure and returns Some , otherwise returns None.
    fn if_true <U,F: FnOnce()->U>(_: F)-> Option<U> { None }
    /// If Self==False,runs the closure and returns Some , otherwise returns None.
    fn if_false<U,F: FnOnce()->U>(_: F)-> Option<U> { None }
}

impl Boolean for True {
    type Not = False;
    const VALUE: bool = true;
    fn if_true<U,F: FnOnce()->U>(f: F)-> Option<U> {
        Some(f())
    }
}
impl Boolean for False {
    type Not = True;
    const VALUE: bool = false;
    fn if_false<U,F: FnOnce()->U>(f: F)-> Option<U> {
        Some(f())
    }
}


///
pub mod internals {
    use super::*;

    /// Type that a Boolean operation evaluates to.
    pub trait BooleanOp {
        /// 
        type Value;
    }

    #[doc(hidden)]
    /// Whether `B` is True or False.Usable for cases where either is required
    pub struct IsTrueOrFalse<B: Boolean>(VariantPhantom<B>);
    impl BooleanOp for IsTrueOrFalse<True> {
        type Value = True;
    }
    impl BooleanOp for IsTrueOrFalse<False> {
        type Value = True;
    }

    impl ops::Not for True{
        type Output=False;
        fn not(self)->Self::Output{ Default::default() }
    }
    impl ops::Not for False{
        type Output=True;
        fn not(self)->Self::Output{ Default::default() }
    }

    impl<B> ops::BitAnd<B> for False{
        type Output=False;
        fn bitand(self,_:B)->Self::Output{ Default::default() }
    }
    impl<B> ops::BitAnd<B> for True{
        type Output=B;
        fn bitand(self,v:B)->Self::Output{ v }
    }

    impl<B> ops::BitOr<B> for True{
        type Output=True;
        fn bitor(self,_:B)->Self::Output{ Default::default() }
    }
    impl<B> ops::BitOr<B> for False{
        type Output=B;
        fn bitor(self,v:B)->Self::Output{ v }
    }

    impl<B:Boolean> ops::BitXor<B> for True{
        type Output=B::Not;
        fn bitxor(self,_:B)->Self::Output{ Default::default() }
    }
    impl<B> ops::BitXor<B> for False{
        type Output=B;
        fn bitxor(self,v:B)->Self::Output{ v }
    }

    /// Struct representing a type-level if.
    pub struct IfElseOp<Cond,Then,Else>(
        VariantPhantom<(Cond,Then,Else)>
    );
    impl<Then,Else> BooleanOp for IfElseOp<True, Then,Else> {
        type Value = Then;
    }
    impl<Then,Else> BooleanOp for IfElseOp<False, Then,Else> {
        type Value = Else;
    }
}

pub use self::internals::*;


/// Negates a [Boolean].
///
/// This type alias takes [Boolean] parameters and evaluates to
/// either [True] or [False].
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(Not::<True >::VALUE,false);
///
///     assert_eq!(Not::<False>::VALUE,true);
///
pub type Not<T> = <T as ops::Not>::Output;

/// `And`s two [Boolean]s
///
/// This type alias takes [Boolean] parameters and evaluates to
/// either [True] or [False].
///
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(And::<True ,True >::VALUE,true);
///
///     assert_eq!(And::<False,True >::VALUE,false);
///
///     assert_eq!(And::<True ,False>::VALUE,false);
///
///     assert_eq!(And::<False,False>::VALUE,false);
///
pub type And<L, R> = <L as ops::BitAnd<R>>::Output;

/// `Or`s two [Boolean]s
///
/// This type alias takes [Boolean] parameters and evaluates to
/// either [True] or [False].
///
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(Or::<True ,True >::VALUE,true);
///
///     assert_eq!(Or::<False,True >::VALUE,true);
///
///     assert_eq!(Or::<True ,False>::VALUE,true);
///
///     assert_eq!(Or::<False,False>::VALUE,false);
///
pub type Or<L, R> = <L as ops::BitOr<R>>::Output;

/// `Nand`s two [Boolean]s
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(Nand::<True ,True >::VALUE,false);
///
///     assert_eq!(Nand::<False,True >::VALUE,true);
///
///     assert_eq!(Nand::<True ,False>::VALUE,true);
///
///     assert_eq!(Nand::<False,False>::VALUE,true);
///
pub type Nand<L, R> = Not<And<L, R>>;

/// `Nor`s two [Boolean]s
///
/// This type alias takes [Boolean] parameters and evaluates to
/// either [True] or [False].
///
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(Nor::<True ,True >::VALUE,false);
///
///     assert_eq!(Nor::<False,True >::VALUE,false);
///
///     assert_eq!(Nor::<True ,False>::VALUE,false);
///
///     assert_eq!(Nor::<False,False>::VALUE,true);
///
pub type Nor<L, R> = Not<Or<L, R>>;

/// `Xor`s two [Boolean]s
///
/// This type alias takes [Boolean] parameters and evaluates to
/// either [True] or [False].
///
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(Xor::<True ,True >::VALUE,false);
///
///     assert_eq!(Xor::<False,True >::VALUE,true);
///
///     assert_eq!(Xor::<True ,False>::VALUE,true);
///
///     assert_eq!(Xor::<False,False>::VALUE,false);
///
pub type Xor<L, R> = <L as ops::BitXor<R>>::Output;

/// `Xnor`s two [Boolean]s
///
/// This type alias takes [Boolean] parameters and evaluates to
/// either [True] or [False].
///
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(Xnor::<True ,True >::VALUE,true);
///
///     assert_eq!(Xnor::<False,True >::VALUE,false);
///
///     assert_eq!(Xnor::<True ,False>::VALUE,false);
///
///     assert_eq!(Xnor::<False,False>::VALUE,true);
///
pub type Xnor<L, R> = Not<Xor<L, R>>;

/// Logical implication
///
/// This type alias takes [Boolean] parameters and evaluates to
/// either [True] or [False].
///
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(Implies::<True ,True >::VALUE,true);
///
///     assert_eq!(Implies::<False,True >::VALUE,true);
///
///     assert_eq!(Implies::<True ,False>::VALUE,false);
///
///     assert_eq!(Implies::<False,False>::VALUE,true);
///
pub type Implies<L, R> = Or<Not<L>, R>;

/// Material non-implication
///
/// This type alias takes [Boolean] parameters and evaluates to
/// either [True] or [False].
///
///
///     # use core_extensions::type_level_bool::*;
///     assert_eq!(NonImp::<True ,True >::VALUE,false);
///
///     assert_eq!(NonImp::<False,True >::VALUE,false);
///
///     assert_eq!(NonImp::<True ,False>::VALUE,true );
///
///     assert_eq!(NonImp::<False,False>::VALUE,false);
///
pub type NonImp<L, R> = And<L, Not<R>>;

/// Type level conditional.
///
/// if `Cond`==True the type is `Then`.
///
/// if `Cond`==False the type is `Else`.
///
/// This will be more ergonomic once specialization is stable.
///
/// # Example
///
/// ```
/// use core_extensions::type_level_bool::{IfElse,True,False};
/// let a:IfElse<True ,i32,&str>=0;
/// let b:IfElse<False,i32,&str>="type level conditionals";
/// ```
///
/// # Example of field whose type depends on a [Boolean](::type_level_bool::Boolean).
///
/// Currently it is necessary to add a where clause to the type containing a conditional field,
/// this limitation will be lifted once specialization is stable.
///
/// ```
/// use core_extensions::VariantPhantom;
/// use core_extensions::type_level_bool::{IfElse,Boolean,False,True};
/// use core_extensions::type_level_bool::internals::{IfElseOp,BooleanOp};
///
/// struct Conditional<ZST>
/// where 
///     IfElseOp<ZST,&'static str,usize>:BooleanOp,
/// {
///     pub value:IfElse<ZST,&'static str,usize>,
///     pub cond:ZST,
/// }
/// 
/// let string_=Conditional{ value:"wtf" ,cond:True  };
/// let int_   =Conditional{ value:99    ,cond:False };
///
/// // The 2 lines bellow won't compile.
/// // let string_=Conditional{ value:"wtf",cond:False };
/// // let int_   =Conditional{ value:99   ,cond:True };
/// ```
/// 
///
///
///
pub type IfElse<Cond,Then,Else>=
    <IfElseOp<Cond,Then,Else> as BooleanOp>::Value;

