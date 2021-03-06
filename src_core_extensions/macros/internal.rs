macro_rules! cfg_if {
    (($meta:meta) $then:block else $else:block) => {{
        #[cfg($meta)] 
        let ret = $then;

        #[cfg(not($meta))] 
        let ret = $else;

        ret
    }};
}


#[cfg(all(feature = "alloc", not(feature = "rust_1_46")))]
macro_rules! rc_shared_docs {
    ($(#[$attr:meta])* => $before_1_46:item  $since_1_46:item  ) => {
        $(#[$attr])*
        $before_1_46
    };
}

#[cfg(all(feature = "alloc", feature = "rust_1_46"))]
macro_rules! rc_shared_docs {
    ($(#[$attr:meta])* => $before_1_46:item  $since_1_46:item  ) => {
        $(#[$attr])*
        $since_1_46
    };
}
