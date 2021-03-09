// Leaving this until I switch to edition = 2018
extern crate core;
extern crate alloc;
extern crate core_extensions;


mod misc_tests {
    #[cfg(feature = "type_identity")]
    mod type_identity_tests;

    #[cfg(feature = "transparent_newtype")]
    mod transparent_newtype_tests;
    
    #[cfg(feature = "option_result")]
    mod result_option_extension_tests;
    
    mod utils_tests;

}
