use std_::fmt;
#[cfg(feature="std")]
use std::panic;

use ::void::Void;
use ::utils::impossible;
#[allow(unused_imports)]
use ::SelfOps;

/// Trait for types with error and item values.
///
/// Types that implement this don't have to have item and error variants,
/// so long as they have values that represent item and error.
///
/// # For Implementors.
/// 
/// There are some things that implementors of this trait must ensure:
/// - [to_result_](#tymethod.to_result_) can't panic ,
/// - that if [ResultLike::is_item](#associatedtype.Item) ==true then 
///        [to_result_](#tymethod.to_result_)
///        returns Ok ([Self::Item](#associatedtype.Item))).
/// - that if [ResultLike::is_error](#associatedtype.Error)==true then 
///        [to_result_](#tymethod.to_result_)
///        returns Err([Self::Error](#associatedtype.Error)).
/// - that [ResultLike::is_error](#method.is_error)(&this) != 
///         [ResultLike::is_item](#tymethod.is_item)(&this)
///
/// # Example
/// ```
/// use core_extensions::ResultLike;
/// 
/// 
/// #[derive(Debug,Clone,Copy,Eq,PartialEq)]
/// pub struct ShouldBeEven(pub u64);
///
/// #[derive(Debug,Clone,Eq,PartialEq)]
/// pub struct WasOddError(pub u64);
///
/// #[derive(Debug,Clone,Eq,PartialEq)]
/// pub struct Even(pub u64);
/// 
/// impl ResultLike for ShouldBeEven{
///     type Item=Even;
///     type Error=WasOddError;
///     
///     fn is_item (&self)->bool{
///         self.to_result_().is_item()
///     }
///     fn to_result_(self)->Result<Self::Item,Self::Error>{
///         if self.0 % 2 ==0 { Ok(Even(self.0)) }else{ Err(WasOddError(self.0)) }
///     }
/// }
///
/// assert_eq!( ShouldBeEven(0).unwrap_()    ,Even(0) );
/// assert_eq!( ShouldBeEven(1).unwrap_err_(),WasOddError(1) );
/// assert_eq!( ShouldBeEven(2).unwrap_()    ,Even(2) );
/// assert_eq!( ShouldBeEven(3).unwrap_err_(),WasOddError(3) );
/// assert_eq!( ShouldBeEven(4).unwrap_()    ,Even(4) );
/// assert_eq!( ShouldBeEven(5).unwrap_err_(),WasOddError(5) );
///
/// ```
pub trait ResultLike: Sized {
    /// The type of the item values
    type Item;
    /// The type of the error values
    type Error;

    /// Converts `self` to a Result.
    /// 
    /// # Panic
    ///
    /// Implementors of this method must ensure that it does not panic.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLike;
    /// use core_extensions::option_result_ext::IsNoneError;
    ///
    /// assert_eq!(Some(0)   .to_result_(),Ok(0));
    /// assert_eq!(None::<()>.to_result_(),Err(IsNoneError));
    ///
    /// assert_eq!(Ok::<i32,()>(0).to_result_(),Ok(0));
    /// assert_eq!(Err::<(),i32>(3).to_result_(),Err(3));
    /// ```
    fn to_result_(self)->Result<Self::Item,Self::Error>;

    /// Queries whether `self` is an item value.
    ///
    /// Note that self.is_item() != self.is_error() must always be true.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLike;
    ///
    /// assert_eq!(Ok ::<i32,()>(10).is_item(),true);
    /// assert_eq!(Err::<i32,()>(()).is_item(),false);
    ///
    /// assert_eq!(Some(10)  .is_item(),true);
    /// assert_eq!(None::<()>.is_item(),false);
    ///
    /// ```
    fn is_item(&self)->bool;

    /// Queries whether `self` is an error value.
    ///
    /// Note that self.is_item() != self.is_error() must always be true.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLike;
    ///
    /// assert_eq!(Ok ::<i32,()>(10).is_error(),false);
    /// assert_eq!(Err::<i32,()>(()).is_error(),true);
    ///
    /// assert_eq!(Some(10)  .is_error(),false);
    /// assert_eq!(None::<()>.is_error(),true);
    ///
    /// ```
    #[inline]
    fn is_error(&self)->bool{
        !self.is_item()
    }

    /// Unwraps the item variant.
    ///
    /// # Panic
    ///
    /// Panics if it's the error variant
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLike;
    ///
    /// assert_eq!(Ok::<i32 ,()>(0      ).unwrap_(),0      );
    /// assert_eq!(Ok::<&str,()>("hello").unwrap_(),"hello");
    ///
    /// ```
    /// 
    /// # Example,panicking
    /// 
    /// ```should_panic
    /// use core_extensions::ResultLike;
    ///
    /// Err::<(),i32 >(0      ).unwrap_();
    /// Err::<(),&str>("hello").unwrap_();
    /// ```
    #[inline]
    fn unwrap_(self) -> Self::Item
    where
        Self::Error: fmt::Debug
    {
        self.to_result_().unwrap()
    }
    /// Unwraps the error variant.
    ///
    /// # Panic
    ///
    /// Panics if it's the item variant
    ///
    /// # Example
    /// ```
    /// use core_extensions::ResultLike;
    ///
    /// assert_eq!(Err::<(),i32 >(0      ).unwrap_err(),0      );
    /// assert_eq!(Err::<(),&str>("hello").unwrap_err(),"hello");
    ///
    /// ```
    /// 
    /// # Example,panicking
    /// ```should_panic
    /// use core_extensions::ResultLike;
    ///
    /// Ok::<i32 ,()>(0      ).unwrap_err();
    /// Ok::<&str,()>("hello").unwrap_err();
    /// ```
    #[inline]
    fn unwrap_err_(self) -> Self::Error
    where
        Self::Item: fmt::Debug
    {
        self.to_result_().unwrap_err()
    }

    #[cfg(any(feature="std",test))]
    #[inline]
    /// Unwraps the item if it is the item value,
    /// otherwise it prints the Error and aborts the process.
    ///
    /// # Panic-safety
    /// 
    /// This method can only panic if `ResultLike::to_result_` panics.
    ///
    /// # Example 
    /// ```
    /// use core_extensions::ResultLike;
    ///
    /// let string_="what \"is\" this";
    /// assert_eq!(Ok::<&str,()>(string_).unwrap_or_abort(),string_);
    /// ```
    fn unwrap_or_abort(self) -> Self::Item
    where 
        Self::Error:fmt::Debug,
    {
        self.to_result_().unwrap_or_else(|e| {
            panic::catch_unwind(panic::AssertUnwindSafe(||{
                println!("{:#?}", e);
            })).drop_();
            ::std::process::abort();
        })
    }

    /// Unwraps the item variant of the type without checking whether this is the current variant.
    ///
    /// # Safety
    ///
    /// You must ensure that it's impossible for this to be the error variant.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLike;
    ///
    /// unsafe{
    ///     assert_eq!(Ok::<_,()>(100).unwrap_unchecked(),100);
    /// }
    /// ```
    #[inline]
    unsafe fn unwrap_unchecked(self) -> Self::Item{
        match self.to_result_() {
            Ok(value) => value,
            Err(_) => impossible(),
        }
    }

    /// Unwraps the error variant of the type without checking whether this is the current variant.
    ///
    /// # Safety
    ///
    /// You must ensure that it's impossible for this to be the item variant.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLike;
    ///
    /// unsafe{
    ///     assert_eq!(Err::<(),_>(100).unwrap_err_unchecked(),100);
    /// }
    /// ```
    #[inline]
    unsafe fn unwrap_err_unchecked(self) -> Self::Error{
        match self.to_result_() {
            Ok(_) => impossible(),
            Err(e) => e,
        }
    }

    /// Unwraps the item knowing that the error is impossible.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLike;
    ///
    /// assert_eq!(Ok(100).unwrap_safe(),100);
    /// ```
    #[inline]
    fn unwrap_safe(self) -> Self::Item
    where
        Self: ResultLike<Error = Void>,
    {
        unsafe { self.unwrap_unchecked() }
    }

    /// Unwraps the error knowing that the item is impossible.
    ///
    /// # Example
    /// ```
    /// use core_extensions::ResultLike;
    ///
    /// assert_eq!(Err(100).unwrap_err_safe(),100);
    /// ```
    #[inline]
    fn unwrap_err_safe(self) -> Self::Error
    where
        Self: ResultLike<Item = Void>,
    {
        unsafe { self.unwrap_err_unchecked() }
    }
}
