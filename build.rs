extern crate rustc_version;
use rustc_version::{version, Version};


fn main() {
    let rver=version().unwrap();
    
    if Version::new(1,27,0) <= rver {
        println!("cargo:rustc-cfg=core_str_methods");
    }

    if Version::new(1,25,0) <= rver {
        println!("cargo:rustc-cfg=core_duration");
    }
    if Version::new(1,26,0) <= rver {
        println!("cargo:rustc-cfg=enable_128");
        println!("cargo:rustc-cfg=enable_copy_closures");
    }
}
