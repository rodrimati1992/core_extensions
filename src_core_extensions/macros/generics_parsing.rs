/// For writing macros that parse item definitions, while treating generics opaquely.
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


/// For writing macros that parse item definitions.
/// This also parses generics for using in all syntactic locations they're usable in.
/// 
/// # Examples
/// 
/// ### Basic
/// 
/// Basic example the syntax this macro expects and passes to a callback macro.
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
///         ('a, T: Foo = $default_ty:ty, const N: $const_ty0:ty,)
///
///         // generics for use in `impl<...>`, and function`declarations
///         ('a, T: Foo, const N: $const_ty1:ty,)
///
///         // generics for use in generic arguments
///         ('a, T, N,)
///
///         // `PhantomData` type that uses all lifetimes and types
///         ($phantom:ty)
///
///         ((Foo, Bar, Baz))            // before the where clause
///         (T: Bar)                     // inside the where clause
///         ( ; )                        // after the where clause
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

            $crate::__psg_unparsed_generics!{
                ($(:: $(@$leading@)? )? $first $(:: $trailing)*) ! $prefix
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __psg_unparsed_generics {
    (
        $path:tt! $params:tt
        ($($generics:tt)*)
        $after_generics:tt
        $where_clause:tt
        $after_where:tt
    ) => {
        $crate::parse_generics!{
            $crate::__psg_parsed_generics!{
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
macro_rules! __psg_parsed_generics {
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



/// Parses a list of generic parameters, and passes them to a macro.
/// 
/// This would be used by macros when they take generic parameters inside `()`, `[]`, or`{}`.
/// 
/// # Examples
/// 
/// ### Basic
/// 
/// Basic example the syntax this macro expects and passes to a callback macro.
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
///         ('a, T: Foo = $default_ty:ty, const N: $const_ty0:ty,)
///
///         // generics for use in `impl<...>`, and function`declarations
///         ('a, T: Foo, const N: $const_ty1:ty,)
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
            ($path:path) {$($prefix:tt)*}
        )
        $struct_params:tt
        $impl_params:tt
        $impl_args:tt
        $phantoms:tt
        ($(,)*)
    ) => {
        $path! {
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
        $crate::__::__priv_remove_non_delimiter!{
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

