/// Rewraps the tokens inside macro parameters into parentheses.
///
/// # Syntax
/// 
/// This macro transforms `~` immediately followed by a macro parameter 
/// into its tokens wrapped in parentheses.
/// 
/// You can escape `~` by writing it twice (`~~`), returning a single `~` from the macro.
/// 
/// # Example
/// 
/// ```rust
/// pub use core_extensions::rewrap_macro_parameters;
/// 
/// crate::constify!{
///     pub fn foo() -> u32 {
///         100
///     }
///     pub unsafe fn bar() -> u32 {
///         200
///     }
/// }
/// 
/// const X: &[u32] = unsafe{ &[foo(), bar()] };
/// 
/// fn main() {
///     assert_eq!(X, &[100, 200]);
/// }
/// 
/// #[macro_export]
/// macro_rules! constify {
///     ($($item:item)*) => {
///         rewrap_macro_parameters!{
///             $crate::__priv_constify_inner!{
///                 hello world ~~~~
///                 // `__priv_constify_inner` can't destructure `$item`,
///                 // so you need to use `rewrap_macro_parameters` and prefix the parameter with
///                 // `~` to rewrap its tokens in parentheses
///                 $(~$item)*
///             }
///         }
///     }
/// }
/// 
/// #[macro_export]
/// #[doc(hidden)]
/// macro_rules! __priv_constify_inner{
///     (
///         hello world ~ ~
///         $((
///             $(#[$attr:meta])*
///             $vis:vis
///             $(unsafe $(@$unsafe:tt@)?)?
///             fn
///             $($rem:tt)*
///         ))*
///     ) => {
///         $(
///             $(#[$attr])*
///             $vis const $(unsafe $(@$unsafe@)?)? fn
///             $($rem)*
///         )*
///     }
/// }
/// 
/// 
/// 
/// ```
/// 
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "macro_utils")))]
#[macro_export]
macro_rules! rewrap_macro_parameters {
    (
        $($tokens:tt)*
    ) => {
        $crate::__::__priv_rewrap_macro_parameters!{
            $($tokens)*
        }
    };
}
