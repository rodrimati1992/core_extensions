
/// Evaluates to true if the expression matches any of the patterns
/// (this macro can have multiple patterns).
///
/// This is equivalent to the [`std::matches`] macro, which requires Rust 1.42.0 .
///
/// # Example
///
/// ```
/// use core_extensions::matches;
///
/// let some = Some(10);
/// assert!( matches!(some, Some(10)));
/// assert!( matches!(some, Some(x) if x == 10));
/// assert!(!matches!(some, None));
///
/// let none = None;
/// assert!(!matches!(none, Some(10)));
/// assert!(!matches!(none, Some(x) if x == 10));
/// assert!( matches!(none, None));
///
/// 
/// for num in &[0, 1, 2, 3][..] {
///     assert!(matches!(num, 0 | 1 | 2 | 3))
/// }
/// 
/// 
/// enum Primitive {
///     Signed(i128),
///     Unsigned(u128),
///     Bool(bool),
///     String(&'static str),
/// }
/// 
/// let prim = Primitive::Bool(false);
/// assert!(matches!(
///     prim,
///     | Primitive::Signed(_)
///     | Primitive::Unsigned(_)
///     | Primitive::Bool(_)
/// ));
/// 
/// ```
/// 
/// [`std::matches`]: https://doc.rust-lang.org/std/macro.matches.html
#[macro_export]
macro_rules! matches {
    ( $expr:expr, $(|)? $pat:pat $(| $prev_pat:pat)* $(if $cond:expr)?)=>{
        match $expr {
            $pat $( | $prev_pat)* =>true,
            _=>false
        }
    };
}