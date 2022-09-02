use krate::utils::transmute_ignore_size;

#[cfg(feature = "alloc")]
use krate::utils::transmute_vec;


///////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

///////////////////////////////////////////////////////////////////////////////

#[cfg_attr(feature = "rust_1_46", track_caller)]
pub(crate) fn assert_macro_input(
    found: &str,
    expected: &str,
) {
    assert_eq!(
        remove_whitespace(expected),
        remove_whitespace(found),
    );
}

///////////////////////////////////////////////////////////////////////////////


macro_rules! assert_is {
    ($macro:ident $prefix:tt $suffix:tt ($($output:tt)*)) => {{
        mod foooo {
            krate::$macro!{
                define_tokens!$prefix
                $suffix
            }
        }

        // Testing with $crate to ensure that this doesn't trigger 
        // https://github.com/rust-lang/rust/issues/101211
        mod baaar {
            krate::$macro!{
                $crate::define_tokens!$prefix
                $suffix
            }
        }

        assert_eq!(foooo::TOKENS, baaar::TOKENS);

        crate::misc_tests::utils_tests::assert_macro_input(
            stringify!($($output)*),
            baaar::TOKENS,
        );
        
    }};
}

///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! stringify_no_whitespace {
    ($($tokens:tt)*) => {
        pub const S: &str = stringify!($($tokens)*);
    };
}

///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! define_tokens {
    ($($tokens:tt)*) => {
        pub(crate) const TOKENS: &str = stringify!($($tokens)*); 
    };
}

///////////////////////////////////////////////////////////////////////////////


#[repr(C)]
#[derive(Debug, PartialEq)]
struct Foo (u32, u64);

#[repr(C)]
struct Bar (u32, u64, u128);

#[test]
fn transmute_ignore_size_test() {
    unsafe{
        assert_eq!(transmute_ignore_size::<u8, u8>(3), 3);
        assert_eq!(transmute_ignore_size::<u64, i64>(!0), -1);
        assert_eq!(transmute_ignore_size::<[u8; 4], u32>([!0, !0, !0, !0]), !0u32);
        
        assert_eq!(transmute_ignore_size::<Bar, Foo>(Bar(3, 5, 8)), Foo(3, 5));
    }
}

#[test]
#[cfg(feature = "alloc")]
fn transmute_vec_test() {
    unsafe {
        assert_eq!(transmute_vec::<u8, u8>(vec![3]), vec![3]);
        assert_eq!(transmute_vec::<u64, i64>(vec![!0]), vec![-1]);
    }
}
