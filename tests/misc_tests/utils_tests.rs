use core_extensions::utils::{transmute_ignore_size, transmute_vec};



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
fn transmute_vec_test() {
    unsafe {
        assert_eq!(transmute_vec::<u8, u8>(vec![3]), vec![3]);
        assert_eq!(transmute_vec::<u64, i64>(vec![!0]), vec![-1]);

        let arrays = vec![[!0, !0, !0, !0], [0, 0, 0, 0]];
        assert_eq!(transmute_vec::<[u8; 4], u32>(arrays), vec![!0u32, 0]);
    }
}