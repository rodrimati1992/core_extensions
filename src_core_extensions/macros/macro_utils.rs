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


#[doc(hidden)]
#[macro_export]
macro_rules! __priv_usize_const {
    ($value:literal) => {
        pub const __USIZE_CONST: $crate::__::usize = $value;
    };
}


if_rust_1_46!{
    /// Counts the amount of token trees passed to this macro,
    /// passing the amount to an (optional) callback macro.
    /// 
    /// 
    /// Note that macro parameters (eg: `$foo`) are one token tree,
    /// and matched pairs of `[]`/`()`/`{}` count as one token tree regardless of 
    /// the tokens inside.
    /// 
    /// # Callback
    /// 
    /// You need to pass a callback macro whenever the macro expects a literal.
    /// 
    /// If you only need the count for an expression(ie: the length of an array),
    /// then no callback macro is necessary.
    /// 
    /// # Version compatibility
    /// 
    /// This macro requires Rust 1.45.0 to be invoked with a callback parameter,
    /// inside an expression.
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
    =>
    (
        #[macro_export]
        macro_rules! count_tts {
            ($parentheses:tt) => {{
                mod __ {
                    $crate::__::count_tts!{
                        $crate::__priv_usize_const!{}
                        $parentheses
                    }
                }
                __::__USIZE_CONST
            }};
            ($($everything:tt)*) => {
                $crate::__::count_tts!{$($everything)*}
            };
        }
    )
    (
        pub use core_extensions_proc_macros::count_tts;
    )
}


/// Generates identifiers. passing them to a callback macro.
/// 
/// # Repetition Syntax
/// 
/// The syntax for describing the generated identifiers:
/// 
/// `for <ident> * in <range>`
/// 
/// Where `<ident>` is any valid identifier.
/// 
/// Where `<range>` can be either `<number> .. <number>` or `<number> ..= <number>`.
/// 
/// Where `<number>` can be any of:
/// 
/// - An integer literal
/// 
/// - `count(....)`: Which counts the amount of token trees in `(....)`,
/// the same way that [`count_tts`] does.
/// 
/// 
/// [`count_tts`]: ./macro.count_tts.html
/// 
/// # Version compatibility
/// 
/// This macro requires Rust 1.45.0 to be invoked inside of a function.
/// 
/// # Example
/// 
/// ### Basic
/// 
/// Basic examples of using this macro, and what it passes to a callback macro.
/// 
/// For a more realistic example you can look [at the one below](#realistic-example)
/// 
/// ```rust
/// use core_extensions::gen_ident_range;
/// 
/// fn main() {
///     assert_eq!(hello(), "world");
///     assert_eq!(foo(), "bar");
/// }
/// 
/// // Calls the `expected_0_to_2` macro.
/// gen_ident_range!{
///     crate::expected_0_to_2!{hello "world"}
///     for stuff_* in 0..3
/// }
/// 
/// // Calls the `expected_1_to_4` macro.
/// gen_ident_range!{
///     crate::expected_1_to_4!{foo "bar" baz}
///     // `count(....)` here counts 4 token trees
///     for pre_* in 1..=count(a (b c) [d e f] {g h i j})
/// }
///
/// #[macro_export]
/// macro_rules! expected_0_to_2{
///     ($func:ident $lit:literal  (stuff_0 stuff_1 stuff_2)) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// 
/// #[macro_export]
/// macro_rules! expected_1_to_4{
///     ($func:ident $lit:literal baz  (pre_1 pre_2 pre_3 pre_4)) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// ```
/// 
/// ### More Realistic Example
/// 
/// ```rust
/// use core_extensions::gen_ident_range;
/// 
/// fn main() {
///     assert_eq!(add_unsigned(3, 5, 8), 16);
///     assert_eq!(add_signed(-3, 8), 5);
/// 
/// }
/// 
/// adder_fn!{ pub fn add_unsigned(u16, u32, u64) -> u64 }
/// adder_fn!{ fn add_signed(i8, i16) -> i64 }
/// 
/// 
/// 
/// #[macro_export]
/// macro_rules! adder_fn {
///     ($vis:vis fn $func:ident ($($arg_ty:ty),* $(,)?) -> $ret_ty:ty) => {
///         gen_ident_range!{
///             $crate::__priv_adder_fn!{
///                 ($vis fn $func ($($arg_ty,)*) -> $ret_ty)
///             }
///             for arg_* in 0..count($($arg_ty)*)
///         }
///     }
/// }
/// 
/// #[macro_export]
/// macro_rules! __priv_adder_fn {
///     (
///         ($vis:vis fn $func:ident ($($arg_ty:ty,)*) -> $ret_ty:ty)
///         ($($arg:ident)*)
///     ) => {
///         $vis fn $func($($arg: $arg_ty,)*) -> $ret_ty {
///             // assuming that Default::default is zero or empty
///             <$ret_ty as $crate::__::Default>::default()
///             $(
///                 + <$ret_ty as $crate::__::From<_>>::from($arg)
///             )*
///         }
///     }
/// }
/// 
/// 
/// #[doc(hidden)]
/// pub mod __ {
///     pub use core_extensions::gen_ident_range;
///     
///     pub use std::convert::From;
///     pub use std::default::Default;
/// }
/// 
/// ```
/// 
pub use core_extensions_proc_macros::gen_ident_range;


