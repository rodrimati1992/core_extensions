
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
///         type(Type)
///         // inside the where clause, this always has a trailing comma
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
///         type ($($type:tt)*)
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



/// For splitting an impl into attributes, safety, parsed generics, trait, type,
/// where clause, and body.
/// 
/// The generic parameters are transformed to be easily parsed by `macro_rules!` macros.
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
/// use core_extensions::impl_parse_generics;
/// 
/// fn main(){
///     assert_eq!(hello(), "world");
/// }
/// 
/// // impl_parse_generics invokes `bar` here
/// impl_parse_generics!{
///     crate::bar!{
///         // The first tokens passed to the `bar` macro
///         hello "world" foo bar 
///     }
///     (
///         #[foo]
///         unsafe impl<'a: 'b, T: Foo, U, const X: usize> Trait<X, Y> for Type
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
///         // The generic parameters are classified by kind
///         // Bounds always have a trailing `+``
///         // Generic parameters always have a trailing `,`
///         (
///             ('a:('b +),)          // lifetimes
///             (T:(Foo +), U:(),)  // types
///             (X: $const_ty:ty,) // constants
///         )
///         // the imlpemented trait.
///         // If this not a trait impl, then `trait(....)` is not passed
///         trait(Trait<X, Y>)
///         // the type that this is an impl for
///         type(Type)
///         // inside the where clause, this always has a trailing comma
///         (U: Bar,)
///         // the body of the impl
///         ({ fn hello() {} })
///     ) => {
///         fn $fn_name() -> &'static str {
///             $returns
///         }
///     };
/// }
/// 
/// ```
/// <div id = "realistic-example"> </div>
/// 
/// ### More Realistic Example
/// 
/// This example demonstrates a macro to avoid having to repeat generic parameters and bounds.
/// 
/// 
/// ```rust
/// use std::ops::Index;
/// 
/// fn main() {
///     let foo = Foo([3, 5], vec![8, 13, 21]);
///     
///     assert_eq!(foo.get(), [3, 5]);
///     assert_eq!(foo[0], 8);
///     assert_eq!(foo[1], 13);
///     assert_eq!(foo[2], 21);
/// }
/// 
/// struct Foo<T, U>(T, U);
/// 
/// repeat_generics!{
///     impl<T: Clone, U> Foo<T, U>
///     where 
///         U: IntoIterator<Item = u32>;
///     
///     impl Self {
///         fn get(&self) -> T {
///             self.0.clone()
///         }
///     }
///     
///     impl<V> Index<V> for Self 
///     where
///         U: Index<V>
///     {
///         type Output = U::Output;
///         
///         fn index(&self, index: V) -> &U::Output {
///             &self.1[index]
///         }
///     }
/// }
/// 
/// 
/// 
/// #[macro_export]
/// macro_rules! repeat_generics {
///     ($($tokens:tt)*)=>{
///         $crate::__::impl_parse_generics!{
///             $crate::__priv_inner_repeat_generics!{@process}
///             ($($tokens)*)
///         }
///     }
/// }
/// 
/// #[doc(hidden)]
/// #[macro_export]
/// macro_rules! __priv_inner_repeat_generics {
///     (
///         @process
///         $attrs:tt
///         ()
///         (
///             ($($lt:lifetime :($($lt_bound:tt)*),)*)
///             ($($ty:ident :($($ty_bound:tt)*),)*)
///             ($($const:ident: $const_ty:ty,)*)
///         )
///         $(trait $trait:tt)?
///         type $Self:tt
///         $where_preds:tt
///         ( ; $($items:tt)* )
///     ) => {
///         $(
///             $crate::__::compile_error!{concat!(
///                 "cannot implement a trait in the impl without a body: ",
///                 stringify!($trait),
///             )}
///         )?
/// 
///         $crate::__priv_inner_repeat_generics!{
///             @iterate
///             (
///                 $attrs
///                 (
///                     ($($lt: $($lt_bound)*,)*) 
///                     ($($ty: $($ty_bound)*,)*)
///                     ($(const $const: $const_ty,)*)
///                 )
///                 $Self
///                 $where_preds
///             )
///             $($items)*
///         }
///     };
///     ( @iterate $params:tt $($item:item)* )=>{
///         $(
///             $crate::__::impl_parse_generics!{
///                 $crate::__priv_inner_repeat_generics!{@inner $params}
///                 ($item)
///             }
///         )*
///     };
///     (
///         @inner
///         (
///             ($($out_attrs:tt)*)
///             ( ($($out_lt:tt)*) ($($out_ty:tt)*) ($($out_const:tt)*) )
///             ($Self:ty)
///             ($($outer_where:tt)*)
///         )
/// 
///         ($($in_attrs:tt)*)
///         ($($qualifiers:tt)*)
///         (
///             ($($lt:lifetime :($($lt_bound:tt)*),)*)
///             ($($ty:ident :($($ty_bound:tt)*),)*)
///             ($($const:ident: $const_ty:ty,)*)
///         )
///         $(trait($trait:ty))?
///         type(Self)
///         ($($inner_where:tt)*)
///         ({ $($items:tt)* })
///     ) => {
///         $($out_attrs)*
///         $($in_attrs)*
///         $($qualifiers)* 
///         impl<
///             $($out_lt)* $($lt: $($lt_bound)*,)*
///             $($out_ty)* $($ty: $($ty_bound)*,)*
///             $($out_const)* $(const $const: $const_ty,)*
///         > $( $trait for )? $Self
///         where
///             $($outer_where)*
///             $($inner_where)*
///         {
///             $($items)*
///         }
///     }
/// }
/// 
/// 
/// #[doc(hidden)]
/// pub mod __ {
///     pub use std::compile_error;
/// 
///     pub use core_extensions::impl_parse_generics;
/// }
/// 
/// 
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "generics_parsing")))]
#[macro_export]
macro_rules! impl_parse_generics {
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* ! $prefix:tt

        ($($tt:tt)*)
    ) => {
        $crate::impl_split!{
            $crate::__ipg_unparsed_generics!{
                ($(:: $(@$leading@)? )? $first $(:: $trailing)*) ! $prefix
            }
            ($($tt)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ipg_unparsed_generics {
    (
        $path:tt! $params:tt

        $attrs:tt
        $qualifiers:tt
        ($($generics:tt)*)
        $(trait $trait:tt)?
        type $type:tt
        $where_clause:tt
        $after_where:tt
    ) => {
        $crate::parse_split_generics!{
            $crate::__ipg_parsed_generics!{
                $path ! $params
                $attrs
                $qualifiers
                $(trait $trait)?
                type $type
                $where_clause
                $after_where
            }

            ($($generics)*)
        }
    }
}


#[doc(hidden)]
#[macro_export]
macro_rules! __ipg_parsed_generics {
    (
        ($($path:tt)*)! {$($prefix:tt)*}
        
        $attrs:tt
        $qualifiers:tt
        $(trait $trait:tt)?
        type $type:tt
        $where_clause:tt
        $after_where:tt

        $gen_in_order:tt
        $gen_by_kind:tt
    ) => {
        $($path)* ! {
            $($prefix)*

            $attrs
            $qualifiers
            $gen_by_kind
            $(trait $trait)?
            type $type
            $where_clause
            $after_where
        }
    }
}
