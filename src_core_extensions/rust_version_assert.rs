#![allow(dead_code)]

// Making sure that this feature is enabled in Rust 1.44
#[cfg(feature = "rust_1_46")]
fn assert_rust_1_46(){
    const _: () = if false {};
}