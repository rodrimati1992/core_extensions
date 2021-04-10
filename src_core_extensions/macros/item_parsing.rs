
/// For splitting an impl into attributes, safety, generics, trait, type, where clause, and body.
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
/// use core_extensions::impl_split;
/// 
/// fn main(){
///     assert_eq!(hello(), "world");
/// }
/// 
/// // impl_split invokes `bar` here
/// impl_split!{
///     crate::bar!{
///         // The first tokens passed to the `bar` macro
///         hello "world" foo bar 
///     }
///     (
///         #[foo]
///         unsafe impl<T: Foo> Trait<X, Y> for Type
///         where U: Bar 
///         {
///             fn hello(){} 
///         }
///     )
/// }
/// 
/// #[macro_export]
/// macro_rules! bar {
///     (
///         $fn_name:ident $returns:literal foo bar 
///         // the attributes
///         (#[foo])
///         // the qualifiers (if `const impl` becomes a thing, i'll be included here)
///         (unsafe)
///         // the generic parameters
///         (T: Foo)
///         // the imlpemented trait.
///         // If this not a trait impl, then `trait(....)` is not passed
///         trait(Trait<X, Y>)
///         // the type that this is an impl for
///         (Type)
///         // inside the where clause
///         (U: Bar,)
///         // the body of the impl
///         ({ fn hello() {} })
///     ) => {
///         fn $fn_name() -> &'static str {
///             $returns
///         }
///     }
/// }
/// 
/// ```
/// <div id = "realistic-example"> </div>
/// 
/// ### More Realistic Example
/// 
/// ```
/// pub use core_extensions::{impl_split, rewrap_opaque};
/// 
/// struct Wrapper<T>(T, [u32; 3]);
/// 
/// crate::constify_methods!{
///     impl<T> Wrapper<T> {
///         pub fn get(&self) -> &T {
///             &self.0
///         }
///         pub fn sum(&self) -> u32 {
///             let [a, b, c] = self.1;
///             a + b + c
///         }
///     }
/// }
/// 
/// const SUM: u32 = Wrapper((), [3, 5, 8]).sum();
/// 
/// fn main() {
///     assert_eq!(SUM, 16);
/// }
/// 
/// #[macro_export]
/// macro_rules! constify_methods {
///     ($impl:item) => {
///         $crate::impl_split!{
///             $crate::__priv_constify_methods!{@parse_impl}
///             ($impl)
///         }
///     }
/// }
/// 
/// #[doc(hidden)]
/// #[macro_export]
/// macro_rules! __priv_constify_methods{
///     (
///         @parse_impl
///         ($(#[$impl_attr:meta])*)
///         ($($qualifiers:tt)*) // Can be `unsafe` (maybe `const` in the future)
///         ($($generics:tt)*)
///         $( trait($($trait:tt)*) )?
///         ($($type:tt)*)
///         ($($where:tt)*)
///         ({ $($item:item)* })
///     ) => {
///         $(#[$impl_attr])*
///         $($qualifiers)* impl<$($generics)*> $($($trait)* for )? $($type)* 
///         where
///             $($where)*
///         {
///             $crate::rewrap_opaque!{$(
///                 $crate::__priv_constify_methods!{
///                     @method
///                     $item
///                     ~$item
///                 }
///             )*}
///         }
///     };
///     (
///         @method $item:item (
///             $(#[$attr:meta])*
///             $vis:vis
///             $(unsafe $(@$unsafe:tt@)?)?
///             fn
///             $($rem:tt)*
///         )
///     ) => {
///         $(#[$attr])*
///         $vis const $(unsafe $(@$unsafe@)?)? fn
///         $($rem)*
///     };
///     (@method $item:item $paren:tt) => {
///         $item
///     };
/// }
/// 
/// 
/// 
/// ```
/// 
#[macro_export]
macro_rules! impl_split {
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* ! $prefix:tt
        ($($split:tt)*)
    ) => {
        $crate::__::__priv_split_impl!{
            ($($split)*)

            $(:: $(@$leading@)? )? $first $(:: $trailing)* ! $prefix
        }
    };
}


