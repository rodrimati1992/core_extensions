/// For parsing item definitions,
/// passing the generic parameters unchanged to a callback macro.
/// 
/// # Version compatibility
/// 
/// This macro can only be used inside of functions since Rust 1.45.0,
/// before that version it can only be used outside of functions.
/// 
/// # Examples
/// 
/// ### Basic
/// 
/// Basic example of using this macro, and what it passes to a callback macro.
/// 
/// For a more realistic example you can look [at the one below](#realistic-example)
/// 
/// ```rust
/// use core_extensions::split_generics_and_where;
/// 
/// fn main() {
///     assert_eq!(hello(), "world")
/// }
/// 
/// // `split_generics_and_where` calls `crate::foo` here
/// split_generics_and_where! {
///     crate::foo!{ 
///         // The first tokens passed to the `crate::foo` macro
///         hello "world" foo bar 
///     }
///     
///     (
///         // The parsed tokens
///         <'a, T: Foo, const N: usize> (param: Type) -> u32 
///         where
///             T: Bar,
///         { println }
///     )
/// }
/// 
/// #[macro_export]
/// macro_rules! foo {
///     (
///         $fn_name:ident $string:literal foo bar
///         ('a, T: Foo, const N: usize) // the generic parameters
///         ((param: Type) -> u32 )      // before the where clause
///         (T: Bar,)                    // inside the where clause
///         ( { println } )              // after the where clause
///     ) => {
///         fn $fn_name() -> &'static str {
///             $string
///         }
///     };
/// }
/// ```
/// 
/// <div id = "realistic-example"> </div>
/// 
/// ### Parsing a function
/// 
/// This demonstrates how you can parse a function.
/// 
/// ```rust
/// use core_extensions::split_generics_and_where;
/// 
/// use std::ops::Mul;
/// 
/// crate::inject_increment! {
///     pub fn square(x: u32) -> u32 {
///         x * x
///     }
/// }
/// crate::inject_increment! {
///     pub(crate) unsafe fn cube<T>(x: T) -> T 
///     where
///         T: Mul<Output = T> + Copy
///     {
///         x * x * x
///     }
/// }
/// fn main() {
///     assert_eq!(get_count(), 0);
///
///     assert_eq!(square(3), 9);
///     assert_eq!(get_count(), 1);
///
///     assert_eq!(unsafe{ cube(5) }, 125);
///     assert_eq!(get_count(), 2);
/// }
/// 
/// #[macro_export]
/// macro_rules! inject_increment {
///     (
///         $(#[$attr:meta])*
///         $vis:vis 
///         $(unsafe $(@$unsafe:tt@)?)?
///         fn $name:ident $($rem:tt)*
///     ) => {
///         split_generics_and_where!{
///             $crate::__priv_inject_increment! {
///                 $(#[$attr])*
///                 $vis,
///                 ($(unsafe $(@$unsafe@)?)?)
///                 fn $name
///             }
///             ($($rem)*)
///         }
///     }
/// }
/// 
/// #[doc(hidden)]
/// #[macro_export]
/// macro_rules! __priv_inject_increment{
///     (
///         $(#[$attr:meta])*
///         $vis:vis,
///         ($($unsafe:tt)?)
///         fn $name:ident 
///         ( $($generics:tt)* )
///         ( ($($fn_params:tt)*) $( -> $ret_ty:ty )? )
///         ( $($where_preds:tt)* )
///         ( { $($code:tt)* } )
///     ) => {
///         $(#[$attr])*
///         $vis
///         $($unsafe)?
///         fn $name< $($generics)* > ( $($fn_params)* ) $( -> $ret_ty )? 
///         where $($where_preds)*
///         {
///             $crate::increment_count();
///             $($code)* 
///         }
///     }
/// }
/// 
/// use std::sync::atomic::{AtomicU64, Ordering as AtomOrd};
/// 
/// pub static COUNT: AtomicU64 = AtomicU64::new(0);
/// 
/// fn increment_count() {
///     COUNT.fetch_add(1, AtomOrd::Relaxed);
/// }
/// 
/// fn get_count() -> u64 {
///     COUNT.load(AtomOrd::Relaxed)
/// }
/// ```
/// 
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "generics_parsing")))]
#[macro_export]
macro_rules! split_generics_and_where {
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* ! $prefix:tt
        ($($generics:tt)*)
    ) => {
        $crate::__::__priv_split_generics!{
            ($($generics)*)

            $(:: $(@$leading@)? )? $first $(:: $trailing)* ! $prefix
        }
    };
}


/// For writing macros that parse item definitions,
/// with the generic parameters transformed for use in type definitions,
/// impl blocks and generic arguments.
/// 
/// 
/// # Version compatibility
/// 
/// This macro can only be used inside of functions since Rust 1.45.0,
/// before that version it can only be used outside of functions.
/// 
/// # Examples
/// 
/// ### Basic
/// 
/// Basic example of the syntax this macro expects and passes to a callback macro.
/// 
/// For a more realistic example you can look [at the one below](#realistic-example)
/// 
/// ```rust
/// use core_extensions::parse_generics_and_where;
/// 
/// fn main() {
///     assert_eq!(hello(), "world")
/// }
/// 
/// // `parse_generics_and_where` calls `crate::foo` here
/// parse_generics_and_where! {
///     crate::foo!{ 
///         // The first tokens passed to the `crate::foo` macro
///         hello "world" foo bar 
///     }
///     
///     (
///         // The parsed tokens, in this case it's for a tuple struct.
///         <'a, T: Foo = A, const N: usize>
///         (Foo, Bar, Baz)
///         where
///             T: Bar;
///     )
/// }
/// 
/// #[macro_export]
/// macro_rules! foo {
///     (
///         $fn_name:ident $string:literal foo bar
///
///         // generics for use in type/trait declarations
///         ('a, T: Foo + = $default_ty:ty, const N: $const_ty0:ty,)
///
///         // generics for use in `impl<...>`, and function`declarations
///         ('a, T: Foo + , const N: $const_ty1:ty,)
///
///         // generics for use in generic arguments
///         ('a, T, N,)
///
///         // `PhantomData` type that uses all lifetimes and types
///         ($phantom:ty)
///
///         // before the where clause
///         ((Foo, Bar, Baz))
///
///         // inside the where clause, this always has a trailing comma
///         (T: Bar,)
///
///         // after the where clause
///         ( ; )
///     ) => {
///         fn $fn_name() -> &'static str {
///             $string
///         }
///     };
/// }
/// ```
/// 
/// <div id = "realistic-example"> </div>
///
/// ### Struct constructor
/// 
/// This demonstrates how you can parse a generic struct, to make a constructor function for it.
/// 
/// ```rust
/// use core_extensions::parse_generics_and_where;
/// 
/// with_constructor! {
///     /// This is Foo
///     #[derive(Debug, PartialEq)]
///     pub struct Foo<T = u32> 
///     where
///         T: Copy,    
///     {
///         foo: T,
///         bar: [T; 2],
///     }
/// }
/// 
/// 
/// fn main() {
///     let bar: Foo = Foo::new(3, [5, 8]);
///     assert_eq!(bar, Foo{foo: 3, bar: [5, 8]});
/// 
///     let baz: Foo<&'static str> = Foo::new("13", ["21", "34"]);
///     assert_eq!(baz, Foo{foo: "13", bar: ["21", "34"]});
/// }
/// 
/// 
/// #[macro_export]
/// macro_rules! with_constructor {
///     (
///         $(#[$attr:meta])*
///         $vis:vis
///         struct $struct_name:ident $($remaining:tt)*
///     ) => {
///         parse_generics_and_where!{
///             $crate::__priv_with_constructor! {
///                 $(#[$attr])*
///                 $vis,
///                 $struct_name,
///             }
///             ($($remaining)*)
///         }
///     }
/// }
/// 
/// #[doc(hidden)]
/// #[macro_export]
/// macro_rules! __priv_with_constructor {
///     (
///         $(#[$attr:meta])*
///         $vis:vis,
///         $struct_name:ident,
///         
///         ($($struct_generics:tt)*)
///         ($($impl_gen:tt)*)
///         ($($gen_args:tt)*)
///         $phantom:tt
///         (/* if this was a tuple struct, it'd get passed the fields here */)
///         ($($where:tt)*)
///         ({
///             $(
///                 $(#[$fattr:meta])* $fvis:vis $fname:ident : $fty:ty ,
///             )*
///         })
///     ) => {
///         $(#[$attr])*
///         $vis struct $struct_name <$($struct_generics)*>
///         where $($where)*
///         {$(
///             $(#[$fattr])* $fvis $fname : $fty ,
///         )*}
///         
///         impl<$($impl_gen)*> $struct_name<$($gen_args)*>
///         where
///             $($where)*
///         {
///             $vis fn new($($fname: $fty),*) -> Self {
///                 Self{ $($fname),* }
///             }
///         }
///     }
/// }
/// 
/// ```
/// 
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "generics_parsing")))]
#[macro_export]
macro_rules! parse_generics_and_where {
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* ! $prefix:tt

        ($($generics:tt)*)
    ) => {
        $crate::__::__priv_split_generics!{
            ($($generics)*)

            $crate::__pgaw_unparsed_generics!{
                ($(:: $(@$leading@)? )? $first $(:: $trailing)*) ! $prefix
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __pgaw_unparsed_generics {
    (
        $path:tt! $params:tt
        ($($generics:tt)*)
        $after_generics:tt
        $where_clause:tt
        $after_where:tt
    ) => {
        $crate::parse_generics!{
            $crate::__pgaw_parsed_generics!{
                $path ! $params
                $after_generics
                $where_clause
                $after_where
            }

            ($($generics)*)
        }
    }
}


#[doc(hidden)]
#[macro_export]
macro_rules! __pgaw_parsed_generics {
    (
        ($($path:tt)*)! {$($prefix:tt)*}
        
        $after_generics:tt
        $where_clause:tt
        $after_where:tt

        $struct_params:tt
        $impl_params:tt
        $impl_args:tt
        $phantoms:tt
    ) => {
        $($path)* ! {
            $($prefix)*

            $struct_params
            $impl_params
            $impl_args
            $phantoms

            $after_generics
            $where_clause
            $after_where
        }
    }
}

/// Transforms generic parameters for use in type definitions,
/// impl blocks and generic arguments, passing them to a callback macro.
/// 
/// 
/// # Version compatibility
/// 
/// This macro can only be used inside of functions since Rust 1.45.0,
/// before that version it can only be used outside of functions.
/// 
/// # Examples
/// 
/// ### Basic
/// 
/// Basic example of the syntax this macro expects and passes to a callback macro.
/// 
/// ```rust
/// use core_extensions::parse_generics;
/// 
/// fn main() {
///     assert_eq!(hello(), "world")
/// }
/// 
/// // `parse_generics` calls `crate::foo` here
/// parse_generics! {
///     crate::foo!{ 
///         // The first tokens passed to the `crate::foo` macro
///         hello "world" foo bar 
///     }
///     
///     (
///         // The parsed tokens
///         'a, T: Foo = A, const N: usize
///     )
/// }
/// 
/// #[macro_export]
/// macro_rules! foo {
///     (
///         $fn_name:ident $string:literal foo bar
///
///         // generics for use in type/trait declarations
///         ('a, T: Foo + = $default_ty:ty, const N: $const_ty0:ty,)
///
///         // generics for use in `impl<...>`, and function`declarations
///         ('a, T: Foo +, const N: $const_ty1:ty,)
///
///         // generics for use in generic arguments
///         ('a, T, N,)
///
///         // `PhantomData` type that uses all lifetimes and types
///         ($phantom:ty)
///     ) => {
///         fn $fn_name() -> &'static str {
///             $string
///         }
///     };
/// }
/// ```
/// 
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "generics_parsing")))]
#[macro_export]
macro_rules! parse_generics {
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* ! {$($prefix:tt)*}

        ($($generics:tt)*)
    )=>{
        $crate::__pg_inner!{
            (
                ($(:: $(@$leading@)? )? $first $(:: $trailing)*) {$($prefix)*}
            )
            () () () ()
            ($($generics)* ,)
        }
    }
}


#[doc(hidden)]
#[macro_export]
macro_rules! __pg_inner {
    (
        (
            ($($path:tt)*) {$($prefix:tt)*}
        )
        $struct_params:tt
        $impl_params:tt
        $impl_args:tt
        $phantoms:tt
        ($(,)*)
    ) => {
        $($path)* ! {
            $($prefix)*

            $struct_params
            $impl_params
            $impl_args
            ($crate::__::PD<$phantoms>)
        }
    };
    (
        $other:tt
        ($($struct_params:tt)*)
        ($($impl_params:tt)*)
        ($($impl_args:tt)*)
        ($($phantoms:tt)*)
        ($lifetime:lifetime $(: $($bound:lifetime $(+)? )*)? , $($rem:tt)*)
    ) => {
        $crate::__pg_inner!{
            $other
            ($($struct_params)* $lifetime $(: $($bound + )*)?,)
            ($($impl_params)* $lifetime $(: $($bound + )*)?,)
            ($($impl_args)* $lifetime,)
            ($($phantoms)* &$lifetime (),)
            ($($rem)*)
        }
    };
    (
        $other:tt
        ($($struct_params:tt)*  )
        ($($impl_params:tt)*)
        ($($impl_args:tt)*)
        ($($phantoms:tt)*)
        ( $type:ident $(= $default:ty)? , $($rem:tt)* )
    ) => {
        $crate::__pg_inner!{
            $other
            ($($struct_params)* $type $(= $default)? ,)
            ($($impl_params)* $type ,)
            ($($impl_args)* $type,)
            ($($phantoms)* $crate::__::PD<$type>,)
            ($($rem)*)
        }
    };
    (
        $other:tt
        $struct_params:tt
        $impl_params:tt
        $impl_args:tt
        $phantoms:tt
        ( $type:ident : $($rem:tt)* )
    ) => {
        $crate::__pg_type_param_bounds!{
            (
                $other
                $type
                $struct_params
                $impl_params
                $impl_args
                $phantoms
            )
            ()
            ( + $($rem)*)
        }
    };
    (
        $other:tt
        ($($struct_params:tt)*  )
        ($($impl_params:tt)*)
        ($($impl_args:tt)*)
        $phantoms:tt
        ( const $constp:ident : $constty:ty $(= $default:expr)? , $($rem:tt)* )
    ) => {
        $crate::__pg_inner!{
            $other
            ($($struct_params)* const $constp: $constty $(= $default)? ,)
            ($($impl_params)* const $constp: $constty,)
            ($($impl_args)* $constp,)
            $phantoms
            ($($rem)*)
        }
    };
    (
        $other:tt
        $struct_params:tt
        $impl_params:tt
        $impl_args:tt
        $phantoms:tt
        ( $($rem:tt)* )
    ) => {
        compile_error!{concat!(
            "Cannot parse these generics:\n\t",
            $(stringify!($rem),)*
        )}
    };
}


#[doc(hidden)]
#[macro_export]
macro_rules! __pg_type_param_bounds {
    (
        (
            $other:tt
            $type:ident
            ($($struct_params:tt)*)
            ($($impl_params:tt)*)
            ($($impl_args:tt)*)
            ($($phantoms:tt)*)
        )
        ($($bounds:tt)*)
        ( $(= $default:ty)? , $($rem:tt)*)
    ) => {
        $crate::__pg_inner!{
            $other
            ($($struct_params)* $type : $($bounds)* $(= $default)? ,)
            ($($impl_params)* $type : $($bounds)*,)
            ($($impl_args)* $type,)
            ($($phantoms)* $crate::__::PD<$type>,)
            ($($rem)*)
        }
    };
    (
        $fixed:tt
        ($($boundts:tt)*)
        ( + $lt:lifetime $($rem:tt)* )
    ) => {
        $crate::__pg_type_param_bounds!{
            $fixed
            ($($boundts)* $lt + )
            ($($rem)*)
        }
    };
    (
        $fixed:tt
        ($($boundts:tt)*)
        ( + ($($parenthesized:tt)*) $($rem:tt)* )
    ) => {
        $crate::__pg_type_param_bounds!{
            $fixed
            ($($boundts)* ($($parenthesized)*) + )
            ($($rem)*)
        }
    };
    (
        $fixed:tt
        $prev_bounds:tt
        ( + $rem_bounds:ty $(= $default:ty)? , $($rem:tt)* )
    ) => {
        $crate::__::__priv_unwrap_bound!{
            $rem_bounds

            $crate::__pg_type_param_finish!{
                $fixed
                $prev_bounds
                ( ($($default)?) $($rem)* )
            }
        }
    };
    (
        $fixed:tt
        ($($boundts:tt)*)
        ( $($rem:tt)* )
    ) => {
        compile_error!{concat!(
            "Cannot parse bounds at the start of these tokens,\n\
             you need to wrap them in parentheses:\n\t",
            $(stringify!($rem),)*
        )}
    };
}




#[doc(hidden)]
#[macro_export]
macro_rules! __pg_type_param_finish {
    (
        (
            $other:tt
            $type:ident
            ($($struct_params:tt)*)
            ($($impl_params:tt)*)
            ($($impl_args:tt)*)
            ($($phantoms:tt)*)
        )
        ($($bounds:tt)*)
        ( ($($($default:tt)+)?) $($rem:tt)* )
        ($($rem_bounds:tt)*)
    ) => {
        $crate::__pg_inner!{
            $other
            ($($struct_params)* $type : $($bounds)* $($rem_bounds)* $(= $($default)+ )? ,)
            ($($impl_params)* $type : $($bounds)* $($rem_bounds)*,)
            ($($impl_args)* $type,)
            ($($phantoms)* $crate::__::PD<$type>,)
            ($($rem)*)
        }
    };
}



























////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////




/// For parsing item definitions,
/// transforming generics to a form easily parsable by a callback macro.
/// 
/// 
/// # Version compatibility
/// 
/// This macro can only be used inside of functions since Rust 1.45.0,
/// before that version it can only be used outside of functions.
/// 
/// # Examples
/// 
/// ### Basic
/// 
/// Basic example of using this macro, and what it passes to a callback macro.
/// 
/// For a more realistic example you can look [at the one below](#realistic-example)
/// 
/// ```rust
/// use core_extensions::parse_split_generics_and_where;
/// 
/// fn main() {
///     assert_eq!(hello(), "world")
/// }
/// 
/// // `parse_split_generics_and_where` calls `crate::foo` here
/// parse_split_generics_and_where! {
///     crate::foo!{ 
///         // The first tokens passed to the `crate::foo` macro
///         hello "world" foo bar 
///     }
///     
///     (
///         // The parsed tokens
///         <'a: 'b, 'b, T: 'a + Foo = Bar, const X: u32 = 10, U = Baz, V> (param: Type) -> u32 
///         where
///             T: Bar
///         { println }
///     )
/// }
/// 
/// #[macro_export]
/// macro_rules! foo {
///     (
///         $fn_name:ident $string:literal foo bar
///
///         // The generic paremeters in the order they came in
///         // Bounds always have a trailing `+``
///         (
///             ('a:('b +))
///             ('b:()) 
///             (type T:('a + Foo +) = $def_t:ty,)
///             (const X: $ty_x:ty = $def_x:expr,)
///             (type U:() = $def_u:ty,)
///             (type V:(),)
///         )
///         // The generic parameters are classified by kind
///         // Bounds always have a trailing `+``
///         // Generic parameters always have a trailing `,`
///         (
///             ('a:('b +), 'b:(),)                                      // lifetimes
///             (T:('a + Foo +) = $defb_t:ty, U:() = $defb_u:ty, V:(),)  // types
///             (X: $tyb_x:ty = $defb_x:expr,)                           // constants
///         )
///
///         // before the where clause
///         ((param: Type) -> u32 )
///
///         // inside the where clause
///         (T: Bar,)
///
///         // after the where clause
///         ( { println } )
///     ) => {
///         fn $fn_name() -> &'static str {
///             $string
///         }
///     };
///     ($($tt:tt)*) => { compile_error!{ stringify!($($tt)*) } }
/// }
/// ```
/// 
/// <div id = "realistic-example"> </div>
/// 
/// ### Derive macro
/// 
/// This demonstrates how you can implement a derive through a `macro_rules!` macro.
/// 
/// ```rust
/// use core_extensions::parse_split_generics_and_where;
/// 
/// fn main() {
///     assert_eq!(Foo{bar: "hi", baz: vec![0]}.add_up(), 2);
///     assert_eq!(Foo{bar: "hello", baz: vec![0]}.add_up(), 5);
///     assert_eq!(Foo{bar: "hello", baz: vec![3]}.add_up(), 8);
///     assert_eq!(Foo{bar: "hello", baz: vec![3, 5]}.add_up(), 13);
/// }
/// 
/// crate::derives!{
///     #[derive(crate::derive_AddUp)]
///     
///     struct Foo<T> {
///         bar: &'static str,
///         baz: T,
///     }
/// }
/// 
/// pub trait AddUp {
///     fn add_up(&self) -> u128;
/// }
/// 
/// impl AddUp for u8 {
///     fn add_up(&self) -> u128 {
///         *self as u128
///     }
/// }
/// 
/// impl AddUp for &str {
///     fn add_up(&self) -> u128 {
///         self.len() as u128
///     }
/// }
/// 
/// impl<T: AddUp> AddUp for &[T] {
///     fn add_up(&self) -> u128 {
///         self.iter().map(AddUp::add_up).sum()
///     }
/// }
///
/// impl<T: AddUp> AddUp for Vec<T> {
///     fn add_up(&self) -> u128 {
///         self.iter().map(AddUp::add_up).sum()
///     }
/// }
/// 
/// 
/// 
/// #[macro_export]
/// macro_rules! derives {
///     (#[derive $derives:tt]  $($type:tt)*) => {
///         $($type)*
///         
///         $crate::derives!{@inner $derives {$($type)*} }
///     };
///     (@inner ($($derive:path),*) $type:tt ) => {
///         $( $derive! $type )*
///     }
/// }
/// 
/// #[macro_export]
/// macro_rules! derive_AddUp {
///     (
///         $(#[$attr:meta])*
///         $vis:vis 
///         struct $name:ident $($generics:tt)*
///     ) => {
///         parse_split_generics_and_where!{
///             $crate::__priv_derive_AddUp! {
///                 $name,
///             }
///             ($($generics)*)
///         }
///     }
/// }
/// 
/// #[doc(hidden)]
/// #[macro_export]
/// macro_rules! __priv_derive_AddUp{
///     (
///         $name:ident,
///         (
///             $( ($lt:lifetime :( $($lt_bounds:tt)* )) )*
///             $((
///                 $( type $typ:ident :($($ty_bound:tt)*) $(= $ty_default:ty )? , )?
///                 $( const $const:ident :($($const_bound:tt)*) $(= $const_default:expr )? , )?
///             ))*
///         )
///         $generic_in_order:tt
///         $__between_generics_and_where:tt
///         ( $($where_preds:tt)* )
///         ({
///             $(
///                 $(#[$fattr:meta])* $fvis:vis $fname:ident : $fty:ty 
///             ),* $(,)?
///         })
///     ) => {
///         impl<
///             $($lt: $($lt_bounds)* ,)*
///             $(
///                 $($typ: $($ty_bound)* $crate::AddUp + )?
///                 $(const $const: $($const_bound)* )?,
///             )*
///         > $name<$($lt,)* $($($typ)? $($const)? ,)*>
///         where
///             $($where_preds)*
///         {
///             fn add_up(&self) -> u128 {
///                 0
///                 $( + self.$fname.add_up() )*
///             }
///         }
///     };
/// }
/// 
/// ```
/// 
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "generics_parsing")))]
#[macro_export]
macro_rules! parse_split_generics_and_where {
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* ! $prefix:tt
        ($($generics:tt)*)
    ) => {
        $crate::__::__priv_split_generics!{
            ($($generics)*)

            $crate::__psgw_unparsed_generics!{
                ($(:: $(@$leading@)? )? $first $(:: $trailing)*) ! $prefix
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __psgw_unparsed_generics {
    (
        $path:tt! $params:tt

        $generics:tt
        $post_generics:tt
        $where_clause:tt
        $after_where:tt
    ) => {
        $crate::parse_split_generics!{
            $crate::__psgw_parsed_generics!{
                $path ! $params
                $post_generics
                $where_clause
                $after_where
            }

            $generics
        }
    }
}


#[doc(hidden)]
#[macro_export]
macro_rules! __psgw_parsed_generics {
    (
        ($($path:tt)*)! {$($prefix:tt)*}
        
        $post_generics:tt
        $where_clause:tt
        $after_where:tt

        $gen_in_order:tt
        $gen_by_kind:tt
    ) => {
        $($path)* ! {
            $($prefix)*

            $gen_in_order
            $gen_by_kind
            $post_generics
            $where_clause
            $after_where
        }
    }
}






/// Transforms generic parameters to a form easily parsable by a callback macro.
/// 
/// 
/// # Version compatibility
/// 
/// This macro can only be used inside of functions since Rust 1.45.0,
/// before that version it can only be used outside of functions.
/// 
/// # Examples
/// 
/// ### Basic
/// 
/// Basic example of the syntax this macro expects and passes to a callback macro.
/// 
/// ```
/// use core_extensions::parse_split_generics;
/// 
/// parse_split_generics!{
///     // The first tokens passed to the `crate::foo` macro
///     foo!{ hello "world" }
///     // The parsed tokens
///     ('a: 'b, 'b, T: 'a + Foo = Bar, const X: u32 = 10, U = Baz, V)
/// }
/// 
/// #[macro_export]
/// macro_rules! foo {
///     (
///         $fn_name:ident $value:literal
///         // The generic paremeters in the order they came in
///         // Bounds always have a trailing `+``
///         (
///             ('a:('b +))
///             ('b:()) 
///             (type T:('a + Foo +) = $def_t:ty,)
///             (const X: $ty_x:ty = $def_x:expr,)
///             (type U:() = $def_u:ty,)
///             (type V:(),)
///         )
///         // The generic parameters are classified by kind
///         // Bounds always have a trailing `+``
///         // Generic parameters always have a trailing `,`
///         (
///             ('a:('b +), 'b:(),)                                      // lifetimes
///             (T:('a + Foo +) = $defb_t:ty, U:() = $defb_u:ty, V:(),)  // types
///             (X: $tyb_x:ty = $defb_x:expr,)                           // constants
///         )
///     ) => {
///
///     };
/// }
///
/// # fn main() {}
/// ```
/// 
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "generics_parsing")))]
#[macro_export]
macro_rules! parse_split_generics {
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* ! {$($prefix:tt)*}

        ($($generics:tt)*)
    )=>{
        $crate::__psg_inner!{
            (
                ($(:: $(@$leading@)? )? $first $(:: $trailing)*) {$($prefix)*}
            )
            ()
            (()()())
            ($($generics)* ,)
        }
    }
}


#[doc(hidden)]
#[macro_export]
macro_rules! __psg_inner {
    (
        (
            ($($path:tt)*) {$($prefix:tt)*}
        )
        $in_order:tt
        $by_kind:tt
        ($(,)*)
    ) => {
        $($path)* !{$($prefix)* $in_order $by_kind}
    };
    (
        $other:tt
        ($($in_order:tt)*)
        (($($lt:tt)*) $types:tt $consts:tt)
        ($lifetime:lifetime $(: $($bound:lifetime $(+)? )*)? , $($rem:tt)*)
    ) => {
        $crate::__psg_inner!{
            $other
            ($($in_order)* ( $lifetime :( $( $($bound +)* )? ) ) )
            (($($lt)* $lifetime:( $( $($bound +)*)? ), ) $types $consts)
            ($($rem)*)
        }
    };
    (
        $other:tt
        ($($in_order:tt)*)
        ($lifetimes:tt ($($types:tt)*) $consts:tt)
        ( $type:ident $(= $default:ty)? , $($rem:tt)* )
    ) => {
        $crate::__psg_inner!{
            $other
            ($($in_order)* (type $type :() $(= $default)? ,) )
            ($lifetimes ($($types)* $type :() $(= $default)? , ) $consts)
            ($($rem)*)
        }
    };
    (
        $other:tt
        $in_order:tt
        $by_kind:tt
        ( $type:ident : $($rem:tt)* )
    ) => {
        $crate::__psg_type_param_bounds!{
            (
                $other
                $type
                $in_order
                $by_kind
            )
            ()
            ( + $($rem)*)
        }
    };
    (
        $other:tt
        ($($in_order:tt)*  )
        ($lifetimes:tt $types:tt ($($consts:tt)*))
        ( const $constp:ident : $constty:ty $(= $default:expr)? , $($rem:tt)* )
    ) => {
        $crate::__psg_inner!{
            $other
            ($($in_order)* (const $constp: $constty $(= $default)?, ) )
            ($lifetimes $types ($($consts)* $constp: $constty $(= $default)? , ) )
            ($($rem)*)
        }
    };
    (
        $other:tt
        $in_order:tt
        $by_kind:tt
        ( $($rem:tt)* )
    ) => {
        compile_error!{concat!(
            "Cannot parse these generics:\n\t",
            $(stringify!($rem),)*
        )}
    };
}


#[doc(hidden)]
#[macro_export]
macro_rules! __psg_type_param_bounds {
    (
        (
            $other:tt
            $type:ident
            ($($in_order:tt)*)
            ($lifetimes:tt ($($types:tt)*) $consts:tt)
        )
        ($($bounds:tt)*)
        ( $(= $default:ty)? , $($rem:tt)*)
    ) => {
        $crate::__psg_inner!{
            $other
            ($($in_order)* (type $type :( $($bounds)* ) $(= $default)? ,) )
            ($lifetimes ($($types)* $type :( $($bounds)* ) $(= $default)?,) $consts)
            ($($rem)*)
        }
    };
    (
        $fixed:tt
        ($($boundts:tt)*)
        ( + $lt:lifetime $($rem:tt)* )
    ) => {
        $crate::__psg_type_param_bounds!{
            $fixed
            ($($boundts)* $lt + )
            ($($rem)*)
        }
    };
    (
        $fixed:tt
        ($($boundts:tt)*)
        ( + ($($parenthesized:tt)*) $($rem:tt)* )
    ) => {
        $crate::__psg_type_param_bounds!{
            $fixed
            ($($boundts)* ($($parenthesized)*) + )
            ($($rem)*)
        }
    };
    (
        $fixed:tt
        $prev_bounds:tt
        ( + $rem_bounds:ty $(= $default:ty)? , $($rem:tt)* )
    ) => {
        $crate::__::__priv_unwrap_bound!{
            $rem_bounds

            $crate::__psg_type_param_finish!{
                $fixed
                $prev_bounds
                ( ($($default)?) $($rem)* )
            }
        }
    };
    (
        $fixed:tt
        ($($boundts:tt)*)
        ( $($rem:tt)* )
    ) => {
        compile_error!{concat!(
            "Cannot parse bounds at the start of these tokens,\n\
             you need to wrap them in parentheses:\n\t",
            $(stringify!($rem),)*
        )}
    };
}




#[doc(hidden)]
#[macro_export]
macro_rules! __psg_type_param_finish {
    (
        (
            $other:tt
            $type:ident
            ($($in_order:tt)*)
            ($lifetimes:tt ($($types:tt)*) $consts:tt)
        )
        ($($bounds:tt)*)
        ( ($($($default:tt)+)?) $($rem:tt)* )
        ($($rem_bounds:tt)*)
    ) => {
        $crate::__psg_inner!{
            $other
            ($($in_order)* (type $type :( $($bounds)* $($rem_bounds)* ) $(= $($default)+ )? ,) )
            (
                $lifetimes
                ($($types)* $type :( $($bounds)* $($rem_bounds)* ) $(= $($default)+ )? ,)
                $consts
            )
            ($($rem)*)
        }
    };
}

