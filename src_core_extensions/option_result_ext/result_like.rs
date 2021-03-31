use std_::{
    convert::Infallible,
    fmt,
};

use utils::impossible;

/// Trait for types with error and item values.
///
/// Types that implement this don't have to have item and error variants,
/// so long as they have distinct values that represent items and errors.
///
/// For more provided methods, you can import the [`ResultLikeExt`] trait.
///
/// # For Implementors.
///
/// There are some things that implementors of this trait must ensure:
///
/// - [`ResultLike::into_result_`](#tymethod.into_result_) must not panic.
///
/// - If [`ResultLike::is_item`](#associatedtype.Item) returns `true`, 
///   then [`ResultLike::into_result_`](#tymethod.into_result_) must return `Ok`.
///
/// - If [`ResultLike::is_error`](#associatedtype.Error) returns `true`, 
///   then [`ResultLike::into_result_`](#tymethod.into_result_) must return `Err`.
///
/// - [`ResultLike::is_error`](#method.is_error) must not equal
///         [`ResultLike::is_item`](#tymethod.is_item) for the same value.
///
/// - If [`ResultLike::from_result_`](#method.from_result_) is overriden,
/// it must return a value equivalent to the default implementation.
///
///
/// # Examples
///
/// ### Implementing `ResultLike`
///
/// ```
/// use core_extensions::{ResultLike, ResultLikeExt};
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
///     type Item = Even;
///     type Error = WasOddError;
///     
///     fn is_item (&self) -> bool {
///         self.into_result_().is_item()
///     }
///     fn into_result_(self) -> Result<Self::Item, Self::Error> {
///         if self.0 % 2 == 0 { Ok(Even(self.0)) } else { Err(WasOddError(self.0)) }
///     }
///     fn from_item(x: Even) -> Self {
///         ShouldBeEven(x.0)
///     }
///     fn from_error(x: WasOddError) -> Self {
///         ShouldBeEven(x.0)
///     }
/// }
///
/// assert_eq!(ShouldBeEven(0).unwrap_()    , Even(0));
/// assert_eq!(ShouldBeEven(1).unwrap_err_(), WasOddError(1));
/// assert_eq!(ShouldBeEven(2).unwrap_()    , Even(2));
/// assert_eq!(ShouldBeEven(3).unwrap_err_(), WasOddError(3));
/// assert_eq!(ShouldBeEven(4).unwrap_()    , Even(4));
/// assert_eq!(ShouldBeEven(5).unwrap_err_(), WasOddError(5));
///
/// assert_eq!(ShouldBeEven::from_result_(Ok(Even(10))), ShouldBeEven(10));
/// assert_eq!(ShouldBeEven::from_result_(Err(WasOddError(3))), ShouldBeEven(3));
///
/// ```
/// 
/// ### `and_then` function
/// 
/// ```rust
/// use core_extensions::ResultLike;
/// use core_extensions::option_result_ext::IsNoneError;
/// 
/// fn and_then<R, P, F>(x: R, func: F) -> P
/// where
///     R: ResultLike,
///     P: ResultLike<Error = R::Error>,
///     F: FnOnce(R::Item) -> P
/// {
///     match x.into_result_() {
///         Ok(x) => func(x),
///         Err(e) => P::from_error(e),
///     }
/// }
/// 
/// assert_eq!(and_then(None, |x: u32| x.checked_sub(10)), None);
/// assert_eq!(and_then(Some(10), |x: u32| x.checked_sub(10)), Some(0));
/// assert_eq!(and_then(Some(10), |x: u32| x.checked_sub(11)), None);
/// 
/// assert_eq!(and_then(Ok("100"), |x| x.parse::<i32>() ), Ok(100));
/// assert_eq!(and_then(Err(()), |x: &str| x.parse::<i32>().map_err(drop) ), Err(()));
///
/// // Converting a Result to an Option
/// assert_eq!(and_then(Ok(10), Some), Some(10));
/// assert_eq!(and_then(Err(IsNoneError::new()), Some), None::<&str>);
/// 
/// 
/// ```
///
/// [`ResultLikeExt`]: ./trait.ResultLikeExt.html
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
    /// assert_eq!(Some(0)   .into_result_(), Ok(0));
    /// assert_eq!(None::<()>.into_result_(), Err(IsNoneError::new()));
    ///
    /// assert_eq!(Ok::<i32, ()>(0).into_result_(), Ok(0));
    /// assert_eq!(Err::<(),i32>(3).into_result_(), Err(3));
    /// ```
    fn into_result_(self) -> Result<Self::Item, Self::Error>;

    /// Queries whether `self` is an item value.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLike;
    ///
    /// assert_eq!(Ok ::<i32, ()>(10).is_item(), true);
    /// assert_eq!(Err::<i32, ()>(()).is_item(), false);
    ///
    /// assert_eq!(Some(10)  .is_item(), true);
    /// assert_eq!(None::<()>.is_item(), false);
    ///
    /// ```
    fn is_item(&self) -> bool;

    /// Queries whether `self` is an error value.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLike;
    ///
    /// assert_eq!(Ok ::<i32, ()>(10).is_error(), false);
    /// assert_eq!(Err::<i32, ()>(()).is_error(), true);
    ///
    /// assert_eq!(Some(10)  .is_error(), false);
    /// assert_eq!(None::<()>.is_error(), true);
    ///
    /// ```
    #[inline]
    fn is_error(&self) -> bool {
        !self.is_item()
    }

    /// Constructs `Self` from a `Result`
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLike;
    /// use core_extensions::option_result_ext::IsNoneError;
    ///
    /// assert_eq!(Option::from_result_(Ok(0)), Some(0));
    /// assert_eq!(Option::<()>::from_result_(Err(IsNoneError::new())), None);
    ///
    /// assert_eq!(Result::<i32, ()>::from_result_(Ok(0)), Ok(0));
    /// assert_eq!(Result::<(), i32>::from_result_(Err(3)), Err(3));
    /// ```
    ///
    #[inline]
    fn from_result_(res: Result<Self::Item, Self::Error>) -> Self {
        match res {
            Ok(x) => Self::from_item(x),
            Err(x) => Self::from_error(x),
        }
    }

    /// Constructs `Self` from the [`Item`](#associatedtype.Item) type.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLike;
    ///
    /// assert_eq!(Option::from_item(0), Some(0));
    ///
    /// assert_eq!(Result::<i32, ()>::from_item(0), Ok(0));
    /// ```
    ///
    fn from_item(res: Self::Item) -> Self;

    /// Constructs `Self` from the [`Error`](#associatedtype.Error) type.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLike;
    /// use core_extensions::option_result_ext::IsNoneError;
    ///
    /// assert_eq!(Option::<()>::from_error(IsNoneError::new()), None);
    ///
    /// assert_eq!(Result::<(), i32>::from_error(3), Err(3));
    /// ```
    ///
    fn from_error(res: Self::Error) -> Self;
}

/// Extension trait for [`ResultLike`] implementors.
/// 
/// [`ResultLike`]: ./trait.ResultLike.html
pub trait ResultLikeExt: ResultLike {
    /// Unwraps the item variant, otherwise calls `func` with the error
    ///  
    /// # Example 
    /// 
    /// ```
    /// use core_extensions::ResultLikeExt;
    /// 
    /// assert_eq!(Some(3).unwrap_or_else_(|_| unreachable!()), 3);
    /// assert_eq!(None.unwrap_or_else_(|_| 5 ), 5);
    /// 
    /// assert_eq!(Ok::<u32, u32>(3).unwrap_or_else_(|_| unreachable!()), 3);
    /// assert_eq!(Err::<u32, u32>(5).unwrap_or_else_(|e| e * 2), 10);
    /// 
    /// ```
    ///  
    #[inline]
    #[cfg_attr(feature = "track_caller", track_caller)]
    fn unwrap_or_else_<F>(self, func: F) -> Self::Item
    where
        F: FnOnce(Self::Error) -> Self::Item
    {
        match self.into_result_() {
            Ok(x) => x,
            Err(e) => func(e),
        }
    }

    /// Unwraps the error variant, otherwise calls `func` with the item 
    ///  
    /// # Example 
    /// 
    /// ```
    /// use core_extensions::ResultLikeExt;
    /// use core_extensions::option_result_ext::IsNoneError;
    /// 
    /// assert_eq!(Some(3).unwrap_err_or_else_(|_| IsNoneError::new()), IsNoneError::new());
    /// assert_eq!(None.unwrap_err_or_else_(|_: u32| unreachable!()), IsNoneError::new());
    /// 
    /// assert_eq!(Ok::<u32, u32>(3).unwrap_err_or_else_(|e| e *7), 21);
    /// assert_eq!(Err::<u32, u32>(5).unwrap_err_or_else_(|e| unreachable!()), 5);
    /// 
    /// ```
    ///  
    #[inline]
    #[cfg_attr(feature = "track_caller", track_caller)]
    fn unwrap_err_or_else_<F>(self, func: F) -> Self::Error
    where
        F: FnOnce(Self::Item) -> Self::Error
    {
        match self.into_result_() {
            Ok(x) => func(x),
            Err(e) => e,
        }
    }
    
    /// Unwraps the item variant, otherwise returns `default`.
    /// 
    /// # Example 
    /// 
    /// ```
    /// use core_extensions::ResultLikeExt;
    /// 
    /// assert_eq!(Some(3).unwrap_or_(5), 3);
    /// assert_eq!(None.unwrap_or_(8), 8);
    /// 
    /// assert_eq!(Ok::<u32, ()>(13).unwrap_or_(21), 13);
    /// assert_eq!(Err::<u32, ()>(()).unwrap_or_(55), 55);
    /// 
    /// ```
    #[inline]
    #[cfg_attr(feature = "track_caller", track_caller)]
    fn unwrap_or_(self, default: Self::Item) -> Self::Item {
        match self.into_result_() {
            Ok(x) => x,
            Err(_) => default,
        }
    }

    /// Unwraps the error variant, otherwise returns `default`.
    /// 
    /// # Example 
    /// 
    /// ```
    /// use core_extensions::ResultLikeExt;
    /// 
    /// assert_eq!(Ok::<u32, u32>(13).unwrap_err_or_(21), 21);
    /// assert_eq!(Err::<u32, u32>(34).unwrap_err_or_(55), 34);
    /// 
    /// ```
    #[inline]
    #[cfg_attr(feature = "track_caller", track_caller)]
    fn unwrap_err_or_(self, default: Self::Error) -> Self::Error {
        match self.into_result_() {
            Ok(_) => default,
            Err(e) => e,
        }
    }

    /// Unwraps the item variant.
    ///
    /// # Panic
    ///
    /// Panics if this is the error variant
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLikeExt;
    ///
    /// assert_eq!(Some(13).unwrap_(), 13);
    ///
    /// assert_eq!(Ok::<i32, ()>(0).unwrap_(), 0);
    /// assert_eq!(Ok::<&str, ()>("hello").unwrap_(), "hello");
    ///
    /// ```
    ///
    /// # Example, panicking
    ///
    /// ```should_panic
    /// use core_extensions::ResultLikeExt;
    ///
    /// None::<()>.unwrap_();
    ///
    /// Err::<(), i32>(0).unwrap_();
    /// Err::<(), &str>("hello").unwrap_();
    /// ```
    #[inline]
    #[cfg_attr(feature = "track_caller", track_caller)]
    fn unwrap_(self) -> Self::Item
    where
        Self::Error: fmt::Debug,
    {
        let f = {
            #[cold]
            #[inline(never)]
            |e| panic!("called unwrap_ on an error variant: {:?}", e)
        };
        self.unwrap_or_else_(f)
    }
    /// Unwraps the error variant.
    ///
    /// # Panic
    ///
    /// Panics if this is the item variant
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLikeExt;
    /// use core_extensions::option_result_ext::IsNoneError;
    ///
    /// assert_eq!(None::<u32>.unwrap_err_(), IsNoneError::new());
    ///
    /// assert_eq!(Err::<(), i32>(0).unwrap_err_(), 0);
    /// assert_eq!(Err::<(), &str>("hello").unwrap_err_(), "hello");
    ///
    /// ```
    ///
    /// # Example, panicking
    ///
    /// ```should_panic
    /// use core_extensions::ResultLikeExt;
    ///
    /// Some(10).unwrap_err_();
    ///
    /// Ok::<i32, ()>(0).unwrap_err_();
    /// Ok::<&str, ()>("hello").unwrap_err_();
    /// ```
    #[inline]
    #[cfg_attr(feature = "track_caller", track_caller)]
    fn unwrap_err_(self) -> Self::Error
    where
        Self::Item: fmt::Debug,
    {
        let f = {
            #[cold]
            #[inline(never)]
            |e| panic!("called unwrap_err_ on an item variant: {:?}", e)
        };

        self.unwrap_err_or_else_(f)
    }

    /// Unwraps the item variant,
    /// otherwise prints the error and aborts the process.
    ///
    /// This method also aborts if [`ResultLike::into_result_`] panics.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLikeExt;
    ///
    /// let string = "what \"is\" this";
    /// let res: Result<&str, ()> = Ok(string);
    /// assert_eq!(res.unwrap_or_abort(), string);
    /// ```
    /// 
    /// [`ResultLike::into_result_`]: ./trait.ResultLike.html#tymethod.into_result_
    #[cfg_attr(feature = "track_caller", track_caller)]
    #[cfg(feature = "std")]
    #[inline]
    fn unwrap_or_abort(self) -> Self::Item
    where
        Self::Error: fmt::Debug,
    {
        use self::for_abort::AbortBomb;
        let bomb = AbortBomb::<Self>{
            #[cfg(feature = "track_caller")]
            location: std_::panic::Location::caller(),
            ty: std_::marker::PhantomData,
        };
        let result = self.into_result_();
        std_::mem::forget(bomb);

        #[cold]
        #[inline(never)]
        #[cfg_attr(feature = "track_caller", track_caller)]
        fn on_err<S: ResultLike>(e: S::Error) -> S::Item 
        where
            S::Error: fmt::Debug,
        {
            {
                let aborter = for_abort::AbortOnDrop;

                #[cfg(feature = "track_caller")]
                eprintln!("inside: {}", std_::panic::Location::caller());
                
                eprintln!("called unwrap_or_abort on an error variant: {:?}", e);
                std_::mem::forget(aborter);
            }
            std_::process::abort()
        }

        match result {
            Ok(x) => x,
            Err(e) => on_err::<Self>(e),
        }
    }

    /// Unwraps the item variant of the type without checking whether this is the item variant.
    ///
    /// # Safety
    ///
    /// You must ensure that it's impossible for this to be the error variant.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLikeExt;
    ///
    /// unsafe{
    ///     assert_eq!(Some(21).unwrap_unchecked(), 21);
    ///     assert_eq!(Ok::<_, ()>(100).unwrap_unchecked(), 100);
    /// }
    /// ```
    #[inline]
    unsafe fn unwrap_unchecked(self) -> Self::Item {
        self.unwrap_or_else_(|_| impossible())
    }

    /// Unwraps the error variant of the type without checking whether this is the error variant.
    ///
    /// # Safety
    ///
    /// You must ensure that it's impossible for this to be the item variant.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::ResultLikeExt;
    /// use core_extensions::option_result_ext::IsNoneError;
    ///
    /// unsafe{
    ///     assert_eq!(None::<u32>.unwrap_err_unchecked(), IsNoneError::new());
    ///     assert_eq!(Err::<(), _>(100).unwrap_err_unchecked(), 100);
    /// }
    /// ```
    #[inline]
    unsafe fn unwrap_err_unchecked(self) -> Self::Error {
        self.unwrap_err_or_else_(|_| impossible())
    }

    /// Unwraps the item knowing that the error variant can't be constructed.
    ///
    /// # Example
    ///
    /// With `std::convert::Infallible` as the error 
    /// ```
    /// use core_extensions::ResultLikeExt;
    ///
    /// use std::convert::Infallible;
    ///
    /// let res: Result<i32, Infallible> = Ok(100);
    /// assert_eq!(res.into_item(), 100);
    /// ```
    ///
    /// With `core_extensions::Void` as the error 
    #[cfg_attr(feature = "void", doc = " ```rust")]
    #[cfg_attr(not(feature = "void"), doc = " ```ignore")]
    /// use core_extensions::{ResultLikeExt, Void};
    ///
    /// let res: Result<i32, Void> = Ok(100);
    /// assert_eq!(res.into_item(), 100);
    /// ```
    #[inline]
    fn into_item(self) -> Self::Item
    where
        Self: ResultLike,
        Infallible: From<Self::Error>,
    {
        match self.into_result_() {
            Ok(x) => x,
            Err(e) => match Infallible::from(e) {},
        }
    }

    /// Unwraps the error knowing that the item variant can't be constructed.
    ///
    /// # Example
    ///
    /// With `std::convert::Infallible` as the item variant.
    ///
    /// ```
    /// use core_extensions::ResultLikeExt;
    /// use core_extensions::option_result_ext::IsNoneError;
    ///
    /// use std::convert::Infallible;
    ///
    /// assert_eq!(None::<Infallible>.into_error(), IsNoneError::new());
    ///
    /// let res: Result<Infallible, i32> = Err(100);
    /// assert_eq!(res.into_error(), 100);
    /// ```
    /// 
    /// With `core_extensions::Void` as the item variant.
    #[cfg_attr(feature = "void", doc = " ```rust")]
    #[cfg_attr(not(feature = "void"), doc = " ```ignore")]
    /// use core_extensions::{ResultLikeExt, Void};
    /// use core_extensions::option_result_ext::IsNoneError;
    ///
    /// assert_eq!(None::<Void>.into_error(), IsNoneError::new());
    ///
    /// let res: Result<Void, i32> = Err(100);
    /// assert_eq!(res.into_error(), 100);
    /// ```
    #[inline]
    fn into_error(self) -> Self::Error
    where
        Self: ResultLike,
        Infallible: From<Self::Item>,
    {
        match self.into_result_() {
            Ok(x) => match Infallible::from(x) {},
            Err(e) => e,
        }
    }
}

impl<T> ResultLikeExt for T
where T: ResultLike
{}


#[cfg(feature = "std")]
mod for_abort {
    use std_::marker::PhantomData;

    pub struct AbortOnDrop;

    impl Drop for AbortOnDrop {
        fn drop(&mut self) {
            std_::process::abort();
        }
    }

    pub struct AbortBomb<T>{
        #[cfg(feature = "track_caller")]
        pub(super) location: &'static std_::panic::Location<'static>,
        pub(super) ty: PhantomData<fn() -> T>,
    }

    impl<T> Drop for AbortBomb<T>{
        #[allow(unreachable_code, unused_variables)]
        fn drop(&mut self) {
            cfg_if!{
                (feature = "track_caller") {
                    ffi_panic_message_track(self.location, std_::any::type_name::<T>())
                } else {
                    ffi_panic_message(std_::any::type_name::<T>())
                }
            }
        }
    }

    #[cold]
    #[inline(never)]
    #[cfg(feature = "track_caller")]
    pub fn ffi_panic_message_track(
        loc: &'static std_::panic::Location,
        type_name: &'static str,
    ) -> ! {
        eprintln!("{}", loc);
        ffi_panic_message(type_name);
    }

    #[cold]
    #[inline(never)]
    pub fn ffi_panic_message(type_name: &'static str) -> ! {
        eprintln!(
            "The `ResultLike::into_result_` implementation for this type panicked: `{}`",
            type_name,
        );
        eprintln!("Aborting to handle the panic...\n");
        std_::process::abort();
    }
}
