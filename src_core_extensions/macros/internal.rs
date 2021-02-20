macro_rules! cfg_if {
    (($meta:meta) $then:block else $else:block) => {{
        #[cfg($meta)] 
        let ret = $then;

        #[cfg(not($meta))] 
        let ret = $else;

        ret
    }};
}