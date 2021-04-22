/// Slice-like operations on tokens
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
/// 
/// 
/// 
/// # Methods
/// 
/// These are the methods that this provides:
/// 
/// - [`first`](#first): Gets the first token tree.
/// - [`last`](#last): Gets the last token tree.
/// - [`split_first`](#split_first): Gets the first token tree, and the remaining ones.
/// - [`split_last`](#split_last): Gets the last token tree, and the remaining ones.
/// - [`split_last_n`](#split_last_n): Gets the last n token trees, and the remaining ones.
/// - [`split_at`](#split_at): Gets the token trees before the nth one, and from it.
/// - [`get`](#get): Gets the token(s) at an index or range.
/// 
/// The operations that take integer arguments use
/// [the `<number>` syntax](./macro.gen_ident_range.html#number-syntax) from [`gen_ident_range`]
/// 
/// # Version compatibility
/// 
/// This macro requires Rust 1.45.0 to be invoked inside of a function.
/// 
/// # Examples
/// 
/// ### `first`
///
/// Gets the first token tree.
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
///     first
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
///     first
///     ((1 2 3) 20 30 (40 50))
/// }
///
/// ```
///
/// ### `last`
///
/// Gets the last token tree.
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
///     last
///     (20 30 (40 50) 1000)
/// }
///
///
/// tokens_method!{
///     expects_baaaz!{ baz "qux" }
///     last
///     (20 30 (40 50) (1 2 3))
/// }
///
/// macro_rules! expects_baaaz {
///     ($func:ident $lit:literal  ((1 2 3)) ) => {
///         fn $func() -> &'static str {
///             $lit
///         }
///     }
/// }
///
/// ```
///
/// ### `split_first`
///
/// Gets the first token tree, and the remaining ones.
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
///     split_first
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
///     split_first
///     ((1 2 3) 20 30 (40 50))
/// }
///
/// ```
///
/// ### `split_last`
///
/// Gets the last token tree, and the remaining ones.
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
///     split_last
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
///     split_last
///     (20 30 (40 50) (1 2 3))
/// }
///
/// ```
///
/// ### `split_last_n`
///
/// Gets the last n token trees, and the remaining ones.
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
///     split_last_n(2)
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
///     split_last_n(count(_ 1 (2 2 2)))
///     (10 20 30 (40 50) (1 2 3))
/// }
///
/// ```
///
/// ### `split_at`
///
/// Gets the token trees before the nth one, and from it.
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
///     split_at(1)
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
///     split_at(count(_ 1 (2 2 2)))
///     (20 30 (40 50) 1000 2345)
/// }
///
/// ```
///
/// ### `get`
///
/// Gets the token(s) at an index (either an integer or a range).
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
/// tokens_method!{expects_one!{ foo bar } get(3)  (2 3 (4 5) 6 7)}
/// // `count(_ 1 (2 2))` is equivalent to `3`
/// tokens_method!{expects_one!{ foo bar }  get(count(_ 1 (2 2)))  (2 3 (4 5) 6 7)}
///
/// macro_rules! expects_two {
///     (baz qux (3 (4 5)) ) => {}
/// }
/// tokens_method!{expects_two!{ baz qux }  get(1..3)  (2 3 (4 5) 6 7)}
/// tokens_method!{expects_two!{ baz qux }  get(1..=2)  (2 3 (4 5) 6 7)}
///
/// macro_rules! expects_three {
///     (baz qux (2 3 (4 5)) ) => {}
/// }
/// tokens_method!{expects_three!{ baz qux }  get(0..3)  (2 3 (4 5) 6 7)}
/// tokens_method!{expects_three!{ baz qux }  get( ..3)  (2 3 (4 5) 6 7)}
/// tokens_method!{expects_three!{ baz qux }  get(0..=2)  (2 3 (4 5) 6 7)}
/// tokens_method!{expects_three!{ baz qux }  get( ..=2)  (2 3 (4 5) 6 7)}
///
/// macro_rules! expects_four {
///     (baz qux (3 (4 5) 6 7) ) => {}
/// }
/// tokens_method!{expects_four!{ baz qux }  get(1..)  (2 3 (4 5) 6 7)}
/// tokens_method!{expects_four!{ baz qux }  get(1..)  (2 3 (4 5) 6 7)}
///
/// ```
/// 
/// [`gen_ident_range`]: ./macro.gen_ident_range.html
pub use core_extensions_proc_macros::tokens_method;
