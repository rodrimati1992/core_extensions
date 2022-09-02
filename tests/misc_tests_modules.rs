// Leaving this until I switch to edition = 2018
extern crate core;
extern crate alloc;

// Renamed it to krate to test that macros work through reexports
extern crate core_extensions as krate;
extern crate static_assertions;

mod misc_tests {
    #[macro_use]
    mod utils_tests;
    
    #[cfg(all(feature = "derive", feature = "const_default"))]
    mod const_default_derive;

    #[cfg(feature = "const_val")]
    mod quasiconst_tests;

    #[cfg(feature = "type_identity")]
    mod type_identity_tests;

    #[cfg(feature = "generics_parsing")]
    mod parse_generics_tests;

    #[cfg(feature = "item_parsing")]
    mod item_parsing_tests;

    #[cfg(feature = "macro_utils")]
    mod macro_utils_tests;

    #[cfg(feature = "transparent_newtype")]
    mod transparent_newtype_tests;
    
    #[cfg(feature = "option_result")]
    mod result_option_extension_tests;
    

}
