//! Universal extension trait.Implemented for every type.

use std_::marker::PhantomData;

use ::phantom_variances::*;

// use std::mem;

/// Extension trait for every type.
/// 
/// The most importand methods in this are:
///
/// - [piped](#method.piped): 
///      Allows emulating the pipeline operator.
///
/// - [mutated](#method.mutated):
///      Allows mutating `self` with a closure passing it along the method chain
///
/// - [observe](./trait.SelfOps.html#method.observe):
///     Observes the value of `self` with a closure passing 
///     it along the method chain unmodified.
///
/// - [into_](#method.into_),
///   [as_ref_](#method.as_ref_),
///   [as_mut_](#method.as_mut_):
///      Alternative syntax for the standard conversion methods.
///
pub trait SelfOps {
    /// Represents Self by using a VariantPhantom<Self>,
    /// using the syntax `Type::T` to pass it in methods with `_:VariantPhantom<T>` parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::{SelfOps,IteratorExt};
    ///
    /// assert_eq!((0..4).collect_(Vec::T       ),vec![0,1,2,3]);
    /// assert_eq!((0..4).collect_(Vec::<_>::T  ),vec![0,1,2,3]);
    /// assert_eq!((0..4).collect_(Vec::<i32>::T),vec![0,1,2,3]);
    /// ```
    ///
    ///
    const T: VariantPhantom<Self> = PhantomData;

    /// Represents Self by using a VariantDropPhantom<Self>,for specialized cases.
    ///
    /// For advanced use cases when one needs a drop check on a PhantomData.
    const T_D: VariantPhantom<Self> = PhantomData;

    #[inline(always)]
    /// Asserts that `other` is the same type as `self`.
    fn assert_ty(self,_other:VariantPhantom<Self>)->Self
    where Self:Sized
    {
        self
    }
    
    #[inline(always)]
    /// Asserts that `other` is the same type as `self`.
    fn assert_ty_ref(&self,_other:VariantPhantom<Self>)->&Self
    where Self:Sized
    {
        self
    }


    #[inline(always)]
    /// Asserts that `other` is the same type as `self`.
    fn assert_ty_mut(&mut self,_other:VariantPhantom<Self>)->&mut Self
    where Self:Sized
    {
        self
    }


    #[inline(always)]
    /// Equivalent to [SelfOps::T](#associatedconstant.T),as a method.
    ///
    /// Reasons for calling this method instead:
    ///
    /// - The type is longer that the code required to instantiate it.
    /// 
    /// - To assert that 2 variables have the same type,using `var0.ty_()==var1.ty_()`.
    ///
    fn ty_(&self) -> VariantPhantom<Self> {
        PhantomData
    }

    #[inline(always)]
    /// Equivalent to [Self::ty_],for specialized cases.
    ///
    /// For specialized cases when one needs a drop check on a PhantomData.
    fn ty_d(&self) -> VariantDropPhantom<Self>{
        PhantomData
    }

    #[inline(always)]
    /// Equivalent to [Self::ty_] with an invariant type.
    fn ty_inv(&self) -> InvariantPhantom<Self>{
        PhantomData
    }

    #[inline(always)]
    /// Equivalent to [Self::ty_] with an invariant lifetime.
    fn ty_inv_ref(&self)-> InvariantRefPhantom<Self>{
        PhantomData
    }

    /// Identity comparison to another value of the same type.
    ///
    /// Comparing the address of `self` with the address of `other`.
    ///
    /// # Example 
    ///
    /// ```
    /// use core_extensions::SelfOps;
    ///
    /// let a=5.to_string();
    /// let b=5.to_string();
    ///
    /// assert!(!a.eq_id(&b));
    /// assert!(!b.eq_id(&a));
    /// assert!( a.eq_id(&a));
    /// assert!( b.eq_id(&b));
    /// assert_eq!(a,b);
    ///
    /// ```
    fn eq_id(&self, other: &Self) -> bool {
        (self as *const Self) == (other as *const Self)
    }

    /// Emulates the pipeline operator,allowing method syntax in more places.
    ///
    /// Allows calling functions as part of a method chain.
    /// 
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SelfOps;
    /// use std::sync::{Mutex,Arc};
    ///
    /// let hello="hello"
    ///     .to_string()
    ///     .mutated(|s| s.push_str("_world") )
    ///     .piped(Mutex::new)
    ///     .piped(Arc::new);
    ///
    /// assert_eq!(hello.lock().unwrap().as_str(),"hello_world");
    /// ```
    /// 
    /// # Example,calling functions
    /// ```
    /// use core_extensions::SelfOps;
    ///
    /// # fn opposed<S:AsRef<str>>(s:S)->String{
    /// #     let s=s.as_ref();
    /// #     format!("{}{}",s,s.chars().rev().collect::<String>())
    /// # }
    /// #
    /// "what"
    ///     .piped(|x|opposed(x)+"-")
    ///     .observe(|s| assert_eq!(s,"whattahw-") ) 
    ///     .piped(opposed)
    ///     .observe(|s| assert_eq!(s,"whattahw--whattahw") );
    /// ```
    ///
    #[inline(always)]
    fn piped<F, U>(self, f: F) -> U
    where
        F: FnOnce(Self) -> U,
        Self: Sized,
    {
        f(self)
    }
    
    /// The same as `piped` except that the function takes `&Self`
    /// Useful for functions that take `&Self` instead of `Self`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SelfOps;
    ///
    /// let problem="air".to_string();
    /// let edited=problem.piped_ref(|s| format!("{} problems.",s) );
    ///
    /// println!("{:?}",problem); // problem wasn't consumed by `.piped_ref`
    /// assert_eq!(edited,"air problems.");
    ///
    /// ```
    ///
    #[inline(always)]
    fn piped_ref<'a,F, U>(&'a self, f: F) -> U
    where
        F: FnOnce(&'a Self) -> U,
    {
        f(self)
    }
    
    /// The same as `piped` except that the function takes `&mut Self`.
    /// Useful for functions that take `&mut Self` instead of `Self`.
    ///
    #[inline(always)]
    fn piped_mut<'a,F, U>(&'a mut self, f: F) -> U
    where
        F: FnOnce(&'a mut Self) -> U,
    {
        f(self)
    }
    
    /// Mutates self using a closure taking self by mutable reference,
    /// passing it along the method chain.
    ///
    /// This is useful for initializing a variable and then freezing it.
    ///
    /// # Example of initialization
    ///
    /// ```
    /// use core_extensions::SelfOps;
    /// let list=Vec::new().mutated(|v|{
    ///     v.push("This");
    ///     v.push("is");
    ///     v.push("[redacted]");
    /// });
    /// assert_eq!(list.join(" "),"This is [redacted]");
    ///
    /// ```
    ///
    /// # Example of mutating in a method chain 
    /// ```
    /// use core_extensions::SelfOps;
    /// 
    /// "what".to_string()
    ///     .mutated(|v| v.push_str(" the") )
    ///     .observe(|v| assert_eq!(v,"what the") );
    ///
    /// ```
    ///  
    #[inline(always)]
    fn mutated<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
        Self: Sized,
    {
        f(&mut self);
        self
    }
    /// Observes the value of self passing it along unmodified.
    /// Useful in a long method chain.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SelfOps;
    /// let v="1234"
    ///     .parse()
    ///     .observe(|d|assert_eq!(&Ok(1234),d))
    ///     .unwrap();
    /// assert_eq!(v,1234);
    /// ```
    ///
    #[inline(always)]
    fn observe<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self),
        Self: Sized,
    {
        f(&self);
        self
    }

    /// Performs a conversion using Into.
    ///
    /// This method is defined to allow using the `.into_(String::T)` syntax.
    ///
    /// type::T is an associated constant defined for every type
    /// [here](#associatedconstant.T).
    ///
    /// # Example
    /// ```
    /// use core_extensions::SelfOps;
    /// use std::borrow::Cow;
    /// 
    /// let word="hello";
    /// assert_eq!(word,word.into_(Cow::T));
    /// assert_eq!(word,word.into_(Cow::<str>::T));
    /// assert_eq!(word,word.into_(String::T));
    /// 
    /// let vec_=||vec![0,1,2,3];
    /// assert_eq!(vec_().into_(Cow::T)           ,vec_());
    /// assert_eq!(vec_().into_(Cow::<[usize]>::T),vec_());
    /// assert_eq!(vec_().into_(Vec::T)           ,vec_());
    /// assert_eq!(vec_().into_(Vec::<usize>::T)  ,vec_());
    ///
    /// ```
    #[inline(always)]
    fn into_<T>(self, _: VariantPhantom<T>) -> T
    where
        Self: Into<T>,
    {
        self.into()
    }
    
    /// Performs a reference to reference conversion using AsRef,
    /// using the turbofish `.as_ref_::<_>()` syntax.
    ///
    /// # Example
    /// ```
    /// use core_extensions::SelfOps;
    /// let s="the path";
    /// assert_eq!( s,s.as_ref_::<str>());
    /// ```
    #[inline(always)]
    fn as_ref_<T: ?Sized>(&self) -> &T
    where
        Self: AsRef<T>,
    {
        self.as_ref()
    }
    /// Performs a mutable reference to mutable reference conversion using AsMut,
    /// using the turbofish `.as_mut_::<_>()` syntax.
    ///
    /// # Example
    /// ```
    /// use core_extensions::SelfOps;
    /// let mut s_0=vec![1,2,3,4];
    /// let mut s_1=s_0.clone();
    /// assert_eq!(s_1,s_0.as_mut_::<[_]>());
    /// ```
    #[inline(always)]
    fn as_mut_<T: ?Sized>(&mut self) -> &mut T
    where
        Self: AsMut<T>,
    {
        self.as_mut()
    }
    /// Drops `self` using method notation.
    /// Alternative to `std::mem::drop`.
    /// 
    /// # Example,ignore #\[must_use\] values.
    /// ```
    /// #![deny(unused_must_use)]
    /// use std::fmt::Write;
    /// use core_extensions::SelfOps;
    /// 
    /// let mut buff=String::new();
    /// 
    /// buff.write_str("hello_").drop_();
    /// buff.write_str("world").drop_();
    /// assert_eq!(buff,"hello_world");
    ///
    /// ```
    #[inline(always)]
    fn drop_(self)
    where
        Self: Sized,
    {
    }

}
impl<T: ?Sized> SelfOps for T {}



