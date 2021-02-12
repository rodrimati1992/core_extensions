//! A variety of macros.


#[macro_use]
pub mod phantomdata;

//////////////////////////////////////////////////////////////////////////////////////////////


/// Macro that evaluates to true if the expression matches any of the patterns
/// (this macro can have multiple patterns).
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate core_extensions;
/// # fn main(){
///
/// use std::num::ParseIntError;
///
/// #[derive(Debug,Copy,Clone)]
/// pub struct Even(u64);
///
/// impl Even{
///     fn value(self)->u64{ self.0 }
///
///     fn parse(n:&str)->Result<Option<Even>,ParseIntError>{
///         match n.parse::<u64>() {
///             Ok(v)if v%2==0 =>Ok(Some(Even(v))),
///             Ok(v)          =>Ok(None),
///             Err(e)=>Err(e),
///         }
///     }
/// }
///
/// let mut even_nums=0;
/// for i in 0..11 {
///     let parsed=Even::parse(&i.to_string());
///     if let Ok(Some(Even(j)))=parsed{
///         assert_eq!(i,j);
///         even_nums+=1;
///     }
///     assert!(matches!(
///         |Ok(Some(Even(0)))
///         |Ok(Some(Even(2)))
///         |Ok(Some(Even(4)))
///         |Ok(Some(Even(6)))
///         |Ok(Some(Even(8)))
///         |Ok(Some(Even(10)))
///         |Ok(None)
///         =parsed
///     ));
///     assert!( ! matches!( Err(_)=parsed ));
/// }
/// assert_eq!(even_nums,6);
///
/// assert!(   matches!( Ok(Some(Even(0))) =Even::parse("0") ));
/// assert!( ! matches!( Ok(None)          =Even::parse("0") ));
///
/// assert!(   matches!( Ok(None   )       =Even::parse("1") ));
/// assert!( ! matches!( Ok(Some(_))       =Even::parse("1") ));
///
/// assert!(   matches!( Ok(Some(Even(2))) =Even::parse("2") ));
/// assert!( ! matches!( Ok(None         ) =Even::parse("2") ));
///
/// assert!(   matches!( Ok(None   )       =Even::parse("3") ));
/// assert!( ! matches!( Ok(Some(_))       =Even::parse("3") ));
///
/// assert!(   matches!( Err(_)            =Even::parse("what") ));
/// assert!( ! matches!( Ok (_)            =Even::parse("what") ));
///
/// assert!(   matches!( |Err(_)            =Even::parse("1a") ));
/// assert!( ! matches!( |Ok (_)            =Even::parse("1a") ));
///
/// // you can prefix the first pattern with any ammount of space separated '|'.
/// // "||" is parsed as a short-circuiting logical or.
/// assert!(   matches!( | | Err(_)            =Even::parse("-1") ));
/// assert!( ! matches!( | | Ok (_)            =Even::parse("-1") ));
///
///
/// # }
/// ```
#[macro_export]
macro_rules! matches{
    ( $(|)* $pat:pat $(| $prev_pat:pat)*  =$expr:expr)=>{
        match $expr {
            $pat $( | $prev_pat)* =>true,
            _=>false
        }
    };
}

//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////


/// For implementing the `TransparentNewtype` trait.
#[macro_export]
macro_rules! impl_transparent_newtype {
    ($S:ty) => (
        #[inline(always)]
        fn from_inner_raw(from: *const <$S as $crate::TransparentNewtype>::Inner) -> *const $S {
            from as _
        }

        #[inline(always)]
        fn from_inner_raw_mut(from: *mut <$S as $crate::TransparentNewtype>::Inner) -> *mut $S {
            from as _
        }

        fn as_inner_raw(this: *const $S) -> *const <$S as $crate::TransparentNewtype>::Inner {
            this as _
        }

        fn as_inner_raw_mut(this: *mut $S) -> *mut <$S as $crate::TransparentNewtype>::Inner {
            this as _
        }
    )
}


//////////////////////////////////////////////////////////////////////////////////////////////


