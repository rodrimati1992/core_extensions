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





/// Counts the amount of token trees passed to this macro,
/// passing the amount to an (optional) callback macro.
/// 
/// 
/// Note that macro parameters (eg: `$foo`) are one token tree,
/// and matched pairs of `[]`/`()`/`{}` count as one token tree regardless of the tokens inside.
/// 
/// # Callback
/// 
/// You need to pass a callback macro whenever the macro expects a literal.
/// 
/// If you only need the count for an expression(ie: the length of an array),
/// then no callback macro is necessary.
/// 
/// # Example
/// 
/// ```rust
/// use core_extensions::count_tts;
/// 
/// fn main() {
///     // The counted tokens must be wrapped in parentheses,
///     // otherwise passing a callback macro would be syntactically ambiguous.
///     assert_eq!(count_tts!(()), 0);
///     assert_eq!(count_tts!((zero)), 1);
///     assert_eq!(count_tts!((zero one)), 2);
///     assert_eq!(count_tts!((zero (one two three) four)), 3);
///     
///     assert_eq!(hello(), "hello");
/// }
///
/// macro_rules! expects_5{
///     (
///         foo $ident:ident baz
///         5
///     ) => {
///         fn $ident() -> &'static str {
///             stringify!($ident) 
///         }
///     }
/// }
/// 
/// // Calls the `expects_5` macro.
/// count_tts!{
///     // The invoked macro, and the first arguments passed to it
///     expects_5!{foo hello baz}
///
///     // The token trees to count
///     (a [b c d] (e f) {g h i} 10 )
/// }
/// ```
/// 
pub use core_extensions_proc_macros::count_tts;





