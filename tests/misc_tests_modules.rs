// Leaving this until I switch to edition = 2018
extern crate core;
extern crate alloc;

// Renamed it to krate to test that macros work through reexports
extern crate core_extensions as krate;


mod misc_tests {
    #[cfg(feature = "const_val")]
    mod quasiconst_tests;

    #[cfg(feature = "type_identity")]
    mod type_identity_tests;

    #[cfg(feature = "generics_parsing")]
    mod generics_parsing_tests;

    #[cfg(feature = "transparent_newtype")]
    mod transparent_newtype_tests;
    
    #[cfg(feature = "option_result")]
    mod result_option_extension_tests;
    
    mod utils_tests;

}
