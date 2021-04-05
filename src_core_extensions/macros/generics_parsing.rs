/// Parses a list of generic parameters, and passes them to a macro.
#[macro_export]
macro_rules! parse_generics {
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* ! {$($prefix:tt)*}

        ($($generics:tt)*)

        $($suffix:tt)*
    )=>{
        $crate::__pg_inner!{
            (
                ($(:: $(@$leading@)? )? $first $(:: $trailing)*) {$($prefix)*}
                $($suffix:tt)*
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
            
            $($suffix:tt)*
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

            $($suffix)*
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

