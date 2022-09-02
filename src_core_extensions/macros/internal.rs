#[allow(unused_macros)]
macro_rules! cfg_if {
    (($meta:meta) $then:block else $else:block) => {{
        #[cfg($meta)] 
        let ret = $then;

        #[cfg(not($meta))] 
        let ret = $else;

        ret
    }};
}

#[allow(unused_macros)]
#[cfg(not(feature = "rust_1_46"))]
macro_rules! if_rust_1_46 {
    ($(#[$attr:meta])* => ($($before_1_46:tt)*)  ($($since_1_46:tt)*)  ) => {
        $(#[$attr])*
        $($before_1_46)*
    };
}

#[allow(unused_macros)]
#[cfg(feature = "rust_1_46")]
macro_rules! if_rust_1_46 {
    ($(#[$attr:meta])* => ($($before_1_46:tt)*)  ($($since_1_46:tt)*)  ) => {
        $(#[$attr])*
        $($since_1_46)*
    };
}



#[doc(hidden)]
#[macro_export]
macro_rules! __coerce_item {
    ($item:item) => {
        $item
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __validate_macro_then_parentheses {
    (
        (
            $(::)? $($path:ident)::* ! $prefix:tt
            ($($tokens:tt)*)
        )
        $($expansion:tt)*
    ) => {
        $($expansion)*
    };
    (
        ($($anything:tt)*)
        $($expansion:tt)*
    ) => {
        $crate::__::compile_error!{$crate::__::concat!{
            "expected arguments to be a macro invocation followed by `()`-delimited arguments"
        }}
    };
}
