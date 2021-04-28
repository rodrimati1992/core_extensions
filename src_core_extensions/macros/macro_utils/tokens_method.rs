/// Does slice and iterator operations on tokens, passing the result to a callback macro.
/// 
/// # Elements
/// 
/// The elements that this operates on are token trees, which can be any of:
/// 
/// - A macro parameter(eg: `$foo`)
/// 
/// - A literal
/// 
/// - A pair of matching `[]`/`()`/`{}` (no matter what's contained by the matched pair)
/// 
/// - An identifier
/// 
/// - Punctuation
/// 
/// # Methods
/// 
/// These are the methods that this provides:
/// 
/// - [`first`](#first): Gets the first token tree.
///
/// - [`last`](#last): Gets the last token tree.
///
/// - [`split_first`](#split_first): Gets the first token tree, and the remaining ones.
///
/// - [`split_last`](#split_last): Gets the last token tree, and the remaining ones.
///
/// - [`split_last_n`](#split_last_n): Gets the last n token trees, and the remaining ones.
///
/// - [`split_at`](#split_at): Gets the token trees before the nth one, and from it.
///
/// - [`get`](#get): Gets the token(s) at an index or range.
///
/// - [`split`](#split)/[`split_terminator`](#split_terminator)/
/// [`split_starter`](#split_starter): Splits the tokens with some needle tokens.
///
/// - [`zip_shortest`](#zip_shortest)/[`zip_longest`](#zip_longest): 
/// Return the token trees of every list iterated over in lockstep.
/// 
/// - [`iterate`](#iterate):
/// Nested iteration over multiple lists.
/// 
/// The methods that take integer arguments use
/// [the `<number>` syntax](./macro.gen_ident_range.html#number-syntax) from [`gen_ident_range`]
/// 
/// # Iterators
/// 
/// This macro can iterate over both lists of token trees (eg: `(foo bar baz)`),
/// and iteration functions (eg: `range(0..10)`).
/// 
/// ### Functions
/// 
/// Iteration functions can be used to generate tokens or transform a list of tokens.
/// 
/// These are the functions:
/// 
/// - [`range`](#range-fn): Iterates over all the integers in a range, can be unbounded.
/// 
/// - [`gen_ident_range`](#gen_ident_range-fn):
/// Generates identifiers by using the [`gen_ident_range`] macro.
///
/// - [`chain`](#chain-fn): Concatenates multiple iterators.
/// 
/// When an iterator function generates an unbounded amount of tokens,
/// they must be constrained by another iterator to be used,
/// otherwise producing a compile-time error to prevent the proc macro from running forever.
/// 
/// # Version compatibility
/// 
/// This macro requires Rust 1.45.0 to be invoked inside of a function.
/// 
/// # Examples
/// 
/// Examples that demonstrate properties common to all methods
/// 
/// ### Macro parameters
/// 
/// This demonstrates how you can pass macro parameters to `tokens_method`.
/// 
/// Note that because this example uses this macro in an expression,
/// it requires at least Rust 1.45.0.
/// 
#[cfg_attr(feature = "rust_1_46", doc = "```rust")]
#[cfg_attr(not(feature = "rust_1_46"), doc = "```ignore")]
/// 
/// fn main() {
///     {
///         let arrays = split_array!(2 => 3, 5, 8, 5 + 8, 3 + 5 + 13, 34, 55);
///         assert_eq!(arrays, ([3, 5], [8, 13, 21, 34, 55]));
///     }
///     {
///         let arrays = split_array!(100 => 3, 5, 8, 5 + 8, 3 + 5 + 13, 34, 55);
///         const EMPTY: [i32; 0] = [];
///         assert_eq!(arrays, ([3, 5, 8, 13, 21, 34, 55], EMPTY));
///     }
/// }
/// 
/// 
/// #[macro_export]
/// macro_rules! split_array {
///     ($split_at:literal => $($elem:expr),* $(,)?) => {
///         // `tokens_method` calls `__priv_split_array` with `foo bar` as the first arguments,
///         // passing the return value of `split_at` after them.
///         $crate::__::tokens_method!{
///             __priv_split_array!{foo bar}
///             split_at($split_at):
///             ($($elem)*) // Note the lack of `,`!
///         }
///     }
/// }
///
/// #[doc(hidden)]
/// #[macro_export]
/// macro_rules! __priv_split_array {
///     (foo bar ($($left:expr)*) ($($right:expr)*)) => {
///         ([$($left,)*], [$($right,)*])
///     }
/// }
/// 
/// #[doc(hidden)]
/// mod __ {
///     pub use core_extensions::tokens_method;
/// }
/// ```
/// 
/// # `first`
///
/// Gets the first token tree.
/// 
/// If there are no elements, this produces a `()`.
///
/// ```rust
/// use core_extensions::tokens_method;
/// 
/// fn main() {
///     assert_eq!(foo(), "bar");
///     assert_eq!(baz(), "qux");
/// }
///
/// macro_rules! expects_fooooo {
///     ($func:ident $lit:literal  (1000) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// tokens_method!{
///     expects_fooooo!{ foo "bar" }
///     first:
///     (1000 20 30 (40 50))
/// }
///
/// macro_rules! expects_baaaz {
///     ($func:ident $lit:literal  ((1 2 3)) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// tokens_method!{
///     expects_baaaz!{ baz "qux" }
///     first:
///     ((1 2 3) 20 30 (40 50))
/// }
///
/// ```
///
/// # `last`
///
/// Gets the last token tree.
/// 
/// If there are no elements, this produces a `()`.
///
/// ### Example
///
/// ```rust
/// use core_extensions::tokens_method;
/// 
/// fn main() {
///     assert_eq!(foo(), "bar");
///     assert_eq!(baz(), "qux");
/// }
///
/// macro_rules! expects_fooooo {
///     ($func:ident $lit:literal  (1000) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// tokens_method!{
///     expects_fooooo!{ foo "bar" }
///     last:
///     (20 30 (40 50) 1000)
/// }
///
///
/// macro_rules! expects_baaaz {
///     ($func:ident $lit:literal  ((1 2 3)) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// tokens_method!{
///     expects_baaaz!{ baz "qux" }
///     last:
///     (20 30 (40 50) (1 2 3))
/// }
///
/// ```
///
/// # `split_first`
///
/// Gets the first token tree, and the remaining ones.
/// 
/// If there are no elements, this produces `() ()`.
/// If there is only one element, this produces `($first_element) ()`.
///
/// ### Example
///
/// ```rust
/// use core_extensions::tokens_method;
/// 
/// fn main() {
///     assert_eq!(foo(), "bar");
///     assert_eq!(baz(), "qux");
/// }
///
/// macro_rules! expects_fooooo {
///     ($func:ident $lit:literal  (1000)  (20 30 (40 50)) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// tokens_method!{
///     expects_fooooo!{ foo "bar" }
///     split_first:
///     (1000 20 30 (40 50))
/// }
///
/// macro_rules! expects_baaaz {
///     ($func:ident $lit:literal  ((1 2 3))  (20 30 (40 50)) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// tokens_method!{
///     expects_baaaz!{ baz "qux" }
///     split_first:
///     ((1 2 3) 20 30 (40 50))
/// }
///
/// ```
///
/// # `split_last`
///
/// Gets the last token tree, and the remaining ones.
///
/// If there are no elements, this produces `() ()`.
/// If there is only one element, this produces `() ($last_elemnent)`.
///
/// ### Example
///
/// ```rust
/// use core_extensions::tokens_method;
/// 
/// fn main() {
///     assert_eq!(foo(), "bar");
///     assert_eq!(baz(), "qux");
/// }
///
/// macro_rules! expects_fooooo {
///     ($func:ident $lit:literal  (20 30 (40 50))  (1000) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// tokens_method!{
///     expects_fooooo!{ foo "bar" }
///     split_last:
///     (20 30 (40 50) 1000)
/// }
///
/// macro_rules! expects_baaaz {
///     ($func:ident $lit:literal (20 30 (40 50))  ((1 2 3)) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// tokens_method!{
///     expects_baaaz!{ baz "qux" }
///     split_last:
///     (20 30 (40 50) (1 2 3))
/// }
///
/// ```
///
/// # `split_last_n`
///
/// Gets the last n token trees, and the remaining ones.
///
/// If there's fewer than n token trees in the list,
/// this simply returns the list in `() (here)`.
///
/// ### Example
///
/// ```rust
/// use core_extensions::tokens_method;
/// 
/// fn main() {
///     assert_eq!(foo(), "bar");
///     assert_eq!(baz(), "qux");
/// }
///
/// macro_rules! expects_fooooo {
///     ($func:ident $lit:literal  (20 30)  ((40 50) 1000) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// tokens_method!{
///     expects_fooooo!{ foo "bar" }
///     split_last_n(2):
///     (20 30 (40 50) 1000)
/// }
///
/// macro_rules! expects_baaaz {
///     ($func:ident $lit:literal (10 20)  (30 (40 50) (1 2 3)) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// tokens_method!{
///     expects_baaaz!{ baz "qux" }
///     // Equivalent to `split_last_n(3)`
///     split_last_n(count(_ 1 (2 2 2))):
///     (10 20 30 (40 50) (1 2 3))
/// }
///
/// ```
///
/// # `split_at`
///
/// Gets the token trees before the nth one, and from it.
///
/// If there's fewer than n token trees in the list,
/// this simply returns the list in `(here) ()`.
///
/// ### Example
///
/// ```rust
/// use core_extensions::tokens_method;
/// 
/// fn main() {
///     assert_eq!(foo(), "bar");
///     assert_eq!(baz(), "qux");
/// }
///
/// macro_rules! expects_fooooo {
///     ($func:ident $lit:literal  (20)  (30 (40 50) 1000 2345) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// tokens_method!{
///     expects_fooooo!{ foo "bar" }
///     split_at(1):
///     (20 30 (40 50) 1000 2345)
/// }
///
/// macro_rules! expects_baaaz {
///     ($func:ident $lit:literal (20 30 (40 50))  (1000 2345) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
/// tokens_method!{
///     expects_baaaz!{ baz "qux" }
///     // Equivalent to `split_at(3)`
///     split_at(count(_ 1 (2 2 2))):
///     (20 30 (40 50) 1000 2345)
/// }
///
/// ```
///
/// # `get`
///
/// Gets the token(s) at an index (either an integer or a range).
///
/// IF the integer index is out of bounds, this outputs `()`.
/// 
/// IF the range is out of bounds,
/// this outputs the elements at the in-bound indices (of the range).
///
/// ### Example
///
/// ```rust
/// use core_extensions::tokens_method;
/// 
/// # fn main() {}
///
/// macro_rules! expects_one {
///     (foo bar (6)) => {}
/// }
/// // `tokens_method` invokes `expects_one` here
/// tokens_method!{expects_one!{ foo bar } get(3): (2 3 (4 5) 6 7)}
/// // `count(_ 1 (2 2))` is equivalent to `3`
/// tokens_method!{expects_one!{ foo bar }  get(count(_ 1 (2 2))): (2 3 (4 5) 6 7)}
///
/// macro_rules! expects_two {
///     (baz qux (3 (4 5)) ) => {}
/// }
/// tokens_method!{expects_two!{ baz qux }  get(1..3): (2 3 (4 5) 6 7)}
/// tokens_method!{expects_two!{ baz qux }  get(1..=2): (2 3 (4 5) 6 7)}
///
/// macro_rules! expects_three {
///     (baz qux (2 3 (4 5)) ) => {}
/// }
/// tokens_method!{expects_three!{ baz qux }  get(0..3): (2 3 (4 5) 6 7)}
/// tokens_method!{expects_three!{ baz qux }  get( ..3): (2 3 (4 5) 6 7)}
/// tokens_method!{expects_three!{ baz qux }  get(0..=2): (2 3 (4 5) 6 7)}
/// tokens_method!{expects_three!{ baz qux }  get( ..=2): (2 3 (4 5) 6 7)}
///
/// macro_rules! expects_four {
///     (baz qux (3 (4 5) 6 7) ) => {}
/// }
/// tokens_method!{expects_four!{ baz qux }  get(1..):  (2 3 (4 5) 6 7)}
/// tokens_method!{expects_four!{ baz qux }  get(1..):  (2 3 (4 5) 6 7)}
///
/// ```
/// 
/// # `split`
/// 
/// Splits the tokens with some needle tokens.
///
/// If the needle is at the end of the tokens, this outputs a final `()`.
/// Eg: `X` splits `foo X bar X` into `(foo) (bar) ()`.
/// 
/// If the needle is not found, this outputs all the tokens.
/// 
/// Note that because this example uses this macro in an expression,
/// it requires at least Rust 1.45.0.
/// 
/// ### Example
/// 
#[cfg_attr(feature = "rust_1_46", doc = "```rust")]
#[cfg_attr(not(feature = "rust_1_46"), doc = "```ignore")]
/// fn main() {
///     assert_eq!(
///         piped!(100 |> |x:u32| x + 1 |> |x:u32| x.to_string() ),
///         "101",
///     );
///     assert_eq!(piped!("foo" |> String::from |> repeat), "foofoofoofoo");
/// }
/// 
/// fn repeat<S: AsRef<str>>(s: S) -> String {
///     s.as_ref().repeat(4)
/// }
/// 
/// #[macro_export]
/// macro_rules! piped {
///     ( $($tt:tt)* ) => {
///         $crate::__::tokens_method!(
///             $crate::__priv_piped!(hello)
///             split(|>):
///             ($($tt)*) 
///         )
///     }
/// }
/// 
/// #[doc(hidden)]
/// #[macro_export]
/// macro_rules! __priv_piped {
///     (hello ($value:expr) $(($f:expr))* ) => ({
///         match $value {x => {
///             $( let x = $f(x); )*
///             x
///         }}
///     })
/// }
/// 
/// #[doc(hidden)]
/// pub mod __ {
///     pub use core_extensions::tokens_method;
/// }
/// ```
/// 
/// # `split_terminator`
/// 
/// Splits the tokens with some needle tokens.
///
/// If the needle is at the end of the tokens, this does not output an additional `()`.
/// Eg: `X` splits `foo X bar X` into `(foo) (bar)`.
/// 
/// If the needle is not found, this outputs all the tokens.
/// 
/// Note that because this example uses this macro in an expression,
/// it requires at least Rust 1.45.0.
/// 
/// ### Example
/// 
#[cfg_attr(feature = "rust_1_46", doc = "```rust")]
#[cfg_attr(not(feature = "rust_1_46"), doc = "```ignore")]
/// fn main() {
///     let expected = "hello99_99world";
///     
///     // `++` can be used between strings
///     assert_eq!(concaten!("hello" ++ format!("{0}_{0}", 99) ++ "world"), expected);
///
///     // `++` can also terminate the argument list
///     assert_eq!(concaten!("hello" ++ format!("{0}_{0}", 99) ++ "world" ++), expected);
/// }
/// 
/// #[macro_export]
/// macro_rules! concaten {
///     ( $($tt:tt)* ) => {
///         $crate::__::tokens_method!(
///             $crate::__priv_concaten!(hello)
///             split_terminator(++):
///             ($($tt)*) 
///         )
///     }
/// }
/// 
/// #[doc(hidden)]
/// #[macro_export]
/// macro_rules! __priv_concaten {
///     (hello $(($f:expr))* ) => ({
///         let mut buff = $crate::__::String::new();
///         $(
///             buff.push_str($f.as_ref());
///         )*
///         buff
///     });
///     ($($tt:tt)*) => { core_extensions::compile_error_stringify!{$($tt)*} }
/// }
/// 
/// #[doc(hidden)]
/// pub mod __ {
///     pub use core_extensions::tokens_method;
///     
///     pub use std::string::String;
/// }
/// ```
/// 
/// # `split_starter`
/// 
/// Splits the tokens with some needle tokens.
///
/// If the needle is at the start of the tokens, this does not output a `()` at the start.
/// Eg: `X` splits `X foo X bar` into `(foo) (bar)`.
/// 
/// If the needle is not found, this outputs all the tokens.
/// 
/// Note that because this example uses this macro in an expression,
/// it requires at least Rust 1.45.0.
/// 
/// ### Example
/// 
#[cfg_attr(feature = "rust_1_46", doc = "```rust")]
#[cfg_attr(not(feature = "rust_1_46"), doc = "```ignore")]
/// fn main() {
///     let expected = Flags::Foo.or(Flags::Bar).or(Flags::Baz).or(Flags::Qux);
///     
///     // `|` can be used between flags
///     assert_eq!(combine!(Foo | Bar | Flags::Baz.or(Flags::Qux) ), expected);
///
///     // `|` can also start the argument list
///     const PRE_FLAGS: Flags = combine!(| Foo | returns_flags() | Bar );
///     assert_eq!(PRE_FLAGS, expected);
/// }
/// 
/// const fn returns_flags()-> Flags {
///     combine!(Baz | Qux)
/// }
/// 
/// /// Allows using `Foo | Bar` syntax for Flags in a const context
/// /// (as of Rust 1.51.0, custom types can't overload the `|` operator in const contexts).
/// #[macro_export]
/// macro_rules! combine {
///     ( $($tt:tt)* ) => {
///         $crate::__::tokens_method!(
///             $crate::__priv_combine!(world)
///             split_starter(|):
///             ($($tt)*) 
///         )
///     }
/// }
/// 
/// #[doc(hidden)]
/// #[macro_export]
/// macro_rules! __priv_combine {
///     (world $($param:tt)* ) => (
///         $crate::Flags::Empty $( .or($crate::__priv_combine!(@flag $param)) )*
///     );
///     (@flag ($ident:ident)) => { $crate::Flags::$ident };
///     (@flag ($expression:expr)) => { $expression };
/// }
/// 
/// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// #[repr(transparent)]
/// pub struct Flags(u32);
/// 
/// impl Flags {
///     pub const Empty: Self = Self(0);
///     pub const Foo: Self = Self(1);
///     pub const Bar: Self = Self(2);
///     pub const Baz: Self = Self(4);
///     pub const Qux: Self = Self(8);
/// 
///     pub const fn or(mut self, other: Self) -> Self {
///         self.0 |= other.0;
///         self
///     }
/// }
/// 
/// #[doc(hidden)]
/// pub mod __ {
///     pub use core_extensions::tokens_method;
/// }
/// ```
/// 
/// 
/// # `zip_shortest`
/// 
/// Returns the token trees of every list iterated over in lockstep.
///
/// This returns as many token trees as the shortest list.
///
/// This is similar to [lockstep iteration in macro_rules! macros](#lockstep_iteration),
/// except that those require the lists to be the same length.
/// 
/// ### Example
/// 
/// ```rust
/// use core_extensions::tokens_method;
///
/// fn main() {
///     assert_eq!(foo(), "bar");
///     assert_eq!(baz(), "qux");
/// }
///
/// macro_rules! expected {
///     (
///         $func:ident $value:literal
///         ((foo3) (bar3) (qux3))
///         ((foo5) (bar5) (qux5))
///         ((foo8) (bar8) (qux8))
///         ((foo13) (bar13) (qux13))
///         ((foo21) (bar21) (qux21))
///     ) => {
///         fn $func() -> &'static str {
///             $value
///         }
///     }
/// }
/// 
/// // `tokens_method` calls `expected` here
/// tokens_method!{
///     expected!{foo "bar"}
///     zip_shortest:
///     (foo3 foo5 foo8 foo13 foo21)
///     (bar3 bar5 bar8 bar13 bar21)
///     (qux3 qux5 qux8 qux13 qux21)
/// }
/// 
/// // `tokens_method` calls `expected` here
/// tokens_method!{
///     expected!{baz "qux"}
///     zip_shortest:
///     (foo3 foo5 foo8 foo13 foo21)
///     (bar3 bar5 bar8 bar13 bar21)
///     // this list is truncated because it's longer than the others
///     (qux3 qux5 qux8 qux13 qux21 qux34 qux55)
/// }
/// 
/// ```
/// 
/// <span id="lockstep_iteration"></span>
/// `macro_rules!` requires lockstep iteration to be over lists of the exact same length:
/// ```
/// macro_rules! iteration {
///     ([$($a:tt)*] [$($b:tt)*]) => {
///         bar!( $(($a $b))* )
///     }
/// }
/// ```
/// while `zip_shortest` truncates to the shortest list, 
/// and `zip_longest` fills in `()` for the shorter lists.
/// 
/// 
/// # `zip_longest`
/// 
/// Returns the token trees of every list iterated over in lockstep.
///
/// This returns as many token trees as the longest list,
/// filling in `()` for the shorter lists.
///
/// This is similar to [lockstep iteration in macro_rules! macros](#lockstep_iteration),
/// except that those require the lists to be the same length.
///
/// ### Example
///
/// ```rust
/// use core_extensions::tokens_method;
///
/// fn main() {
///     assert_eq!(baz(), "qux");
/// }
///
/// macro_rules! expected {
///     (
///         $func:ident $value:literal
///         ((0) (bar3) (qux3))
///         ((1) (bar5) (qux5))
///         ((2) (bar8) (qux8))
///         ((3) (bar13) (qux13))
///         ((4) (bar21) (qux21))
///         ((5) ()      (qux34))
///         ((6) ()      (qux55))
///     ) => {
///         fn $func() -> &'static str {
///             $value
///         }
///     }
/// }
/// 
/// // `tokens_method` calls `expected` here
/// tokens_method!{
///     expected!{baz "qux"}
///     zip_longest:
///
///     // Unbounded ranges only generate as many tokens as the longest finite iterator
///     range(0..) 
///     (bar3 bar5 bar8 bar13 bar21)
///     (qux3 qux5 qux8 qux13 qux21 qux34 qux55)
/// }
/// 
/// ```
/// 
/// # `iterate`
/// 
/// Nested iteration over multiple lists.
/// 
/// This can iterate over any amount of lists.
/// 
/// 
/// ### Basic example
///
/// ```rust
/// use core_extensions::tokens_method;
/// 
/// 
/// fn main() {
///     assert_eq!(single(), "111");
///     assert_eq!(double(), "222");
///     assert_eq!(triple(), "333");
/// }
/// 
/// macro_rules! assertion_single{
///     (
///         $func:ident $expr:literal 
///         (foo bar baz)
///     ) => {
///         fn $func() -> &'static str {
///             $expr
///         }
///     }
/// }
/// tokens_method!{
///     assertion_single!{single "111"}
///     iterate:
///     (foo bar baz)
/// }
/// 
///
/// macro_rules! assertion_double{
///     (
///         $func:ident $expr:literal 
///         (
///             { a } (foo bar baz)
///             { b } (foo bar baz)
///             { c } (foo bar baz)
///         )
///     ) => {
///         fn $func() -> &'static str {
///             $expr
///         }
///     }
/// }
/// tokens_method!{
///     assertion_double!{double "222"}
///     iterate:
///     (a b c)
///     (foo bar baz)
/// }
/// 
///
/// macro_rules! assertion_triple{
///     (
///         $func:ident $expr:literal 
///         (
///             { 3 } (
///                 { a } (foo bar baz)
///                 { b } (foo bar baz)
///                 { c } (foo bar baz)
///             ) 
///             { 5 } (
///                 { a } (foo bar baz)
///                 { b } (foo bar baz)
///                 { c } (foo bar baz)
///             )
///             { 8 } (
///                 { a } (foo bar baz)
///                 { b } (foo bar baz)
///                 { c } (foo bar baz)
///             )
///         )
///     ) => {
///         fn $func() -> &'static str {
///             $expr
///         }
///     }
/// }
/// tokens_method!{
///     assertion_triple!{triple "333"}
///     iterate:
///     (3 5 8)
///     (a b c)
///     (foo bar baz)
/// }
/// 
/// ```
/// 
/// ### Enum Example
/// 
/// This demonstrates a macro to declare an enum in which all the variants have the same fields. 
///
/// ```rust
/// 
/// homogeneous_enum!{ 
///     pub enum Foo { Bar, Baz, Qux } 
///     fields = { a, b, c }
///     field_type = u32,
/// }
///
/// fn main() {
///    Foo::Bar{a: 3, b: 5, c: 8};
///    Foo::Baz{a: 13, b: 21, c: 34};
///    Foo::Qux{a: 55, b: 89, c: 144};
/// }
/// 
/// #[macro_export]
/// macro_rules! homogeneous_enum{
///     (
///         $(#[$attr:meta])*
///         $vis:vis enum $enum_name:ident {
///             $($variant:ident),* $(,)?
///         }
///         fields = { $($field:ident),* $(,)? }
///         field_type = $field_ty:ty $(,)?
///     ) => {
///         $crate::__::tokens_method!{
///             $crate::__priv_homogeneous_enum!{
///                 $(#[$attr])*
///                 $vis,
///                 $enum_name,
///                 $field_ty,
///             }
///             iterate:
///             ($($variant)*)
///             ($($field)*)
///         }
///     }
/// }
///
///
/// #[doc(hidden)]
/// #[macro_export]
/// macro_rules! __priv_homogeneous_enum{
///     (
///         $(#[$attr:meta])*
///         $vis:vis,
///         $enum_name:ident,
///         $field_ty:ty,
///         ($(
///             {$variant:ident} ($($field_ident:ident)*)
///         )*)
///     ) => {
///         $(#[$attr])*
///         $vis enum $enum_name {
///             $(
///                 $variant {
///                     $( $field_ident: $field_ty, )*
///                 },
///             )*
///         }
///     }
/// }
///
/// mod __ { 
///     pub use core_extensions::tokens_method;
/// }
/// ```
/// 
/// <span id="range-fn"></span>
/// # `range` iterator function
/// 
/// Iterates over a range, can be bounded or unbounded.
/// 
/// If the range is unbounded, it must be constrained by some other iterator,
/// otherwise causing a compile-time error.
/// 
/// This uses 
/// [the `<number>` syntax](./macro.gen_ident_range.html#number-syntax) from [`gen_ident_range`]
/// for the range bounds.
/// 
/// ### Example
/// 
/// ```
/// use core_extensions::tokens_method;
/// 
/// macro_rules! assertion {
///     ((0 1 2 3 4)) => {}
/// }
///
/// // `tokens_method` calls `assertion` here
/// tokens_method!{assertion!{} iterate: range(0..5)}
/// tokens_method!{assertion!{} iterate: range(..5)}
/// tokens_method!{assertion!{} iterate: range(0..=4)}
/// tokens_method!{assertion!{} iterate: range(..=4)}
/// // You can use `count(....)` to count token trees, using the count as a range bound.
/// tokens_method!{assertion!{} iterate: range(..count(_ _ _ _ _))}
///
/// macro_rules! assert_zip {
///     (((0) (a)) ((1) (b)) ((2) (c)) ((3) ((d f g))) ((4) ({h i j}))) => {}
/// }
/// 
/// // Both of these call `assert_zip` with the same tokens
/// tokens_method!{
///     assert_zip!{}
///     zip_shortest: 
///     range(0..)
///     (a b c (d f g) {h i j})
/// }
/// tokens_method!{
///     assert_zip!{}
///     zip_longest: 
///     range(0..)
///     (a b c (d f g) {h i j})
/// }
/// 
/// ```
/// 
/// <span id="gen_ident_range-fn"></span>
/// # `gen_ident_range` iterator function
/// 
/// Generates identifiers by using the [`gen_ident_range`] macro.
/// 
/// The range can be unbounded so long as it's constrained by some other iterator,
/// 
/// ### Example
/// 
/// ```
/// use core_extensions::tokens_method;
/// 
/// macro_rules! assertion {
///     ((pre_1 pre_2 pre_3 pre_4 pre_5)) => {}
/// }
///
/// // `tokens_method` calls `assertion` here
/// tokens_method!{
///     assertion!{}
///     iterate: gen_ident_range(for pre_* in 1..=5) 
/// }
/// tokens_method!{
///     assertion!{}
///     iterate: gen_ident_range(for pre_* in 1..6) 
/// }
/// tokens_method!{
///     assertion!{}
///     iterate: gen_ident_range(for pre_* in 1..=count(_ _ _ _ _)) 
/// }
/// 
/// 
/// // One way unbounded ranges can be used
/// macro_rules! assertion_zipped {
///     (((a) (foo0)) ((b) (foo1)) ((c) (foo2))) => {}
/// }
///     
/// // `tokens_method` calls `assertion_zipped` here
/// tokens_method!{
///     assertion_zipped!{}
///     zip_shortest:   
///     (a b c)
///     gen_ident_range(for foo* in 0..) 
/// }
/// 
/// ```
/// 
/// <span id="chain-fn"></span>
/// # `chain` iterator function
/// 
/// Concatenates multiple iterators.
/// 
/// The iterators can be unbounded so long as `chain` is constrained by some other iterator,
/// 
/// ### Example
/// 
/// ```
/// use core_extensions::tokens_method;
/// 
/// macro_rules! assertion {
///     ((a b c 0 1 2)) => {}
/// }
///
/// // `tokens_method` calls `assertion` here
/// tokens_method!{
///     assertion!{}
///     iterate: chain((a b c) range(0..=2)) 
/// }
/// 
/// 
/// macro_rules! assertion_zipped {
///     (((0) (a)) ((1) (b)) ((2) (10)) ((3) (11))) => {};
/// }
///
/// // One way unbounded ranges can be used.
/// // `tokens_method` calls `assertion_zipped` here
/// tokens_method!{
///     assertion_zipped!{}
///     zip_shortest:
///     range(0..=3)
///     chain((a b) range(10..)) 
/// }
/// 
/// ```
/// 
/// [`gen_ident_range`]: ./macro.gen_ident_range.html
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "macro_utils")))]
pub use core_extensions_proc_macros::tokens_method;
