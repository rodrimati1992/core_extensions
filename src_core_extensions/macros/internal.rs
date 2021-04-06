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
macro_rules! rc_shared_docs {
    ($(#[$attr:meta])* => ($($before_1_46:tt)*)  ($($since_1_46:tt)*)  ) => {
        $(#[$attr])*
        $($before_1_46)*
    };
}

#[allow(unused_macros)]
#[cfg(feature = "rust_1_46")]
macro_rules! rc_shared_docs {
    ($(#[$attr:meta])* => ($($before_1_46:tt)*)  ($($since_1_46:tt)*)  ) => {
        $(#[$attr])*
        $($since_1_46)*
    };
}
