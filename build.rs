extern crate rustc_version;
use rustc_version::{version, Version};

fn main() {
    let rver = version().unwrap();

    if Version::new(1, 22, 0) <= rver {
        println!("cargo:rustc-cfg=rust_1_22");
    }
    if Version::new(1, 24, 0) <= rver {
        println!("cargo:rustc-cfg=rust_1_24");
    }
    if Version::new(1, 25, 0) <= rver {
        println!("cargo:rustc-cfg=core_duration");
        println!("cargo:rustc-cfg=rust_1_25");
    }
    if Version::new(1, 26, 0) <= rver {
        println!("cargo:rustc-cfg=enable_128");
        println!("cargo:rustc-cfg=rust_1_26");
        println!("cargo:rustc-cfg=enable_copy_closures");
    }
    if Version::new(1, 27, 0) <= rver {
        println!("cargo:rustc-cfg=core_str_methods");
    }
    if Version::new(1, 29, 0) <= rver {
        println!("cargo:rustc-cfg=rust_1_29");
    }
    if Version::new(1, 30, 0) <= rver {
        println!("cargo:rustc-cfg=trim_left_right_method_deprecation");
    }
    if Version::new(1, 32, 0) <= rver {
        println!("cargo:rustc-cfg=rust_1_32");
    }
    if Version::new(1, 34, 0) <= rver {
        println!("cargo:rustc-cfg=rust_1_34");
    }
    if Version::new(1, 36, 0) <= rver {
        println!("cargo:rustc-cfg=rust_1_36");
    }
    if Version::new(1, 39, 0) <= rver {
        println!("cargo:rustc-cfg=rust_1_39");
    }
    if Version::new(1, 42, 0) <= rver {
        println!("cargo:rustc-cfg=rust_1_42");
    }
}
