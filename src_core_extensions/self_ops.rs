//! Universal extension trait.Implemented for every type.


/// Extension trait for every type.
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "self_ops")))]
pub trait SelfOps {
    /// Compares the address of `self` with the address of `other`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SelfOps;
    ///
    /// let a = 5.to_string();
    /// let b = 5.to_string();
    ///
    /// assert!(!a.eq_id(&b));
    /// assert!(!b.eq_id(&a));
    /// assert!( a.eq_id(&a));
    /// assert!( b.eq_id(&b));
    /// assert_eq!(a,b);
    ///
    /// ```
    fn eq_id(&self, other: &Self) -> bool {
        core::ptr::eq(self as *const Self, other as *const Self)
    }

    /// Emulates the pipeline operator, allowing method syntax in more places.
    ///
    /// Allows calling functions as part of a method chain.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SelfOps;
    ///
    /// use std::sync::{Mutex, Arc};
    ///
    /// let hello = "hello"
    ///     .to_string()
    ///     .mutated(|s| s.push_str("_world"))
    ///     .piped(Mutex::new)
    ///     .piped(Arc::new);
    ///
    /// assert_eq!(hello.lock().unwrap().as_str(), "hello_world");
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
    /// 
    /// "what"
    ///     .piped(|x| opposed(x) + "-")
    ///     .observe(|s| assert_eq!(s, "whattahw-"))
    ///     .piped(opposed)
    ///     .observe(|s| assert_eq!(s, "whattahw--whattahw"));
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
    /// let problem = "air".to_string();
    /// let edited = problem.piped_ref(|s| format!("{} problems.", s));
    ///
    /// println!("{:?}", problem); // problem wasn't consumed by `.piped_ref`
    /// assert_eq!(edited, "air problems.");
    ///
    /// ```
    ///
    #[inline(always)]
    fn piped_ref<'a, F, U>(&'a self, f: F) -> U
    where
        F: FnOnce(&'a Self) -> U,
    {
        f(self)
    }

    /// The same as `piped`, except that the function takes `&mut Self`.
    /// Useful for functions that take `&mut Self` instead of `Self`.
    ///
    #[inline(always)]
    fn piped_mut<'a, F, U>(&'a mut self, f: F) -> U
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
    ///
    /// let list = Vec::new().mutated(|v|{
    ///     v.push("This");
    ///     v.push("is");
    ///     v.push("[redacted]");
    /// });
    ///
    /// assert_eq!(list.join(" "), "This is [redacted]");
    ///
    /// ```
    ///
    /// # Example of mutating in a method chain
    /// ```
    /// use core_extensions::SelfOps;
    ///
    /// "what".to_string()
    ///     .mutated(|v| v.push_str(" the") )
    ///     .observe(|v| assert_eq!(v, "what the") );
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
    /// Observes the value of self, passing it along unmodified.
    /// Useful in long method chains.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SelfOps;
    ///
    /// let v = "1234"
    ///     .parse()
    ///     .observe(|d| assert_eq!(&Ok(1234), d))
    ///     .unwrap();
    ///
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

    /// Performs a conversion with `Into`.
    /// using the turbofish `.into_::<_>()` syntax.
    ///
    /// # Example
    /// ```
    /// use core_extensions::SelfOps;
    /// use std::borrow::Cow;
    ///
    /// let word = "hello";
    ///
    /// assert_eq!(word, word.into_::<Cow<'_, _>>());
    /// assert_eq!(word, word.into_::<Cow<'_, str>>());
    /// assert_eq!(word, word.into_::<String>());
    ///
    /// let vec_=||vec![0,1,2,3];
    /// assert_eq!(vec_().into_::<Cow<'_, _>>(), vec_());
    /// assert_eq!(vec_().into_::<Cow<'_, _>>(), vec_());
    /// assert_eq!(vec_().into_::<Vec<_>>()    , vec_());
    /// assert_eq!(vec_().into_::<Vec<_>>()    , vec_());
    ///
    /// ```
    #[inline(always)]
    fn into_<T>(self) -> T
    where
        Self: Into<T>,
    {
        self.into()
    }

    /// Performs a reference to reference conversion with `AsRef`,
    /// using the turbofish `.as_ref_::<_>()` syntax.
    ///
    /// # Example
    /// ```
    /// use core_extensions::SelfOps;
    ///
    /// let s = "the path";
    ///
    /// assert_eq!(s, s.as_ref_::<str>());
    /// ```
    #[inline(always)]
    fn as_ref_<T: ?Sized>(&self) -> &T
    where
        Self: AsRef<T>,
    {
        self.as_ref()
    }
    /// Performs a mutable reference to mutable reference conversion with `AsMut`,
    /// using the turbofish `.as_mut_::<_>()` syntax.
    ///
    /// # Example
    /// ```
    /// use core_extensions::SelfOps;
    ///
    /// let mut s_0 = vec![1, 2, 3, 4];
    /// let mut s_1 = s_0.clone();
    ///
    /// assert_eq!(s_1, s_0.as_mut_::<[_]>());
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
    ///
    /// use core_extensions::SelfOps;
    ///
    /// let mut buff=String::new();
    ///
    /// buff.write_str("hello_").drop_();
    /// buff.write_str("world").drop_();
    ///
    /// assert_eq!(buff, "hello_world");
    ///
    /// ```
    #[inline(always)]
    fn drop_(self)
    where
        Self: Sized,
    {
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Prevents creating a trait object of this trait
    fn _dummy_generic_method_preventing_trait_object<F>(self: &Self) {}
}
impl<T: ?Sized> SelfOps for T {}
