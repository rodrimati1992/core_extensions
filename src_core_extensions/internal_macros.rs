// Used because this crate supports Rust down to 1.20.0 
// when `?` for Option was stabilized in Rust 1.22.0
macro_rules! try_opt {
    ($expr:expr) => {
        match $expr {
            Some(v) => v,
            None => return None,
        }
    }
}