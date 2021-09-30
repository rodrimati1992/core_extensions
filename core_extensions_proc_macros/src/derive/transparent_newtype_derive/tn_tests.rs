use crate::test_utils::{TestStrExt, remove_whitespaces};

use super::derive_for_tests as dft;

use alloc::format;

macro_rules! single {
    ($attrs:expr) => (
        concat!($attrs, "#[repr(transparent)] struct Foo<T>(T);")
    )
}


#[test]
fn test_repr_attr() {
    assert!(
        dft("struct Foo(u8);")
            .unwrap_err()
            .consecutive_unspace(&["compile_error", "#[repr(transparent)]"])
    );
    assert!(
        dft("#[repr(C)] struct Foo(u8);")
            .unwrap_err()
            .consecutive_unspace(&["compile_error", "#[repr(transparent)]"])
    );
    assert!(
        dft("#[repr(transparent, align(8))] struct Foo(u8);")
            .unwrap_err()
            .consecutive_unspace(&["compile_error", "#[repr(transparent)]"])
    );
    assert!(
        dft("#[repr(align(8))] #[repr(transparent)] struct Foo(u8);")
            .unwrap_err()
            .consecutive_unspace(&["compile_error", "#[repr(transparent)]"])
    );
    assert!(
        dft("#[repr(transparent)] #[repr(align(8))] struct Foo(u8);")
            .unwrap_err()
            .consecutive_unspace(&["compile_error", "#[repr(transparent)]"])
    );
    assert!(
        dft("#[repr(transparent)] struct Foo(u8);")
            .unwrap()
            .consecutive_unspace(&["impl", "TransparentNewtype for Foo"])
    );
}

#[test]
fn test_crate_attr() {
    #[track_caller]
    fn assert_renamed(in_: &str, expected: &str) {
        let ret = dft(&format!(single!("{}"), in_)).unwrap();
        assert!(ret.consecutive_unspace(&[expected]));
        assert_eq!(ret.matches("use").count(), 1);
    }
    assert_renamed("", "use ::core_extensions");
    assert_renamed("#[twrap(crate = foo::bar)]", "use foo::bar");
    assert_renamed("#[twrap(crate = ::foo::bar)]", "use ::foo::bar");
}

#[test]
fn test_where_attr() {
    {
        let left  = dft(single!("")).unwrap();
        let right = dft(single!("#[twrap(where)]")).unwrap();
        assert_eq!(remove_whitespaces(&left), remove_whitespaces(&right));
    }
    {
        let aaa = dft(single!("#[twrap(where Foo: Bar<{baz}>)] ")).unwrap();
        let bbb = dft(single!("#[twrap(where Foo: Bar<{baz}>,)]")).unwrap();
        assert!(aaa.consecutive_unspace(&["Foo: Bar<{baz}>,"]), "{}", aaa);
        assert!(bbb.consecutive_unspace(&["Foo: Bar<{baz}>,"]));
    }
    {
        let aaa = dft(single!("#[twrap(where Foo: Bar, Baz: Qux)]")).unwrap();
        let bbb = dft(single!("#[twrap(where Foo: Bar, Baz: Qux,)]")).unwrap();
        assert!(aaa.consecutive_unspace(&["Foo: Bar, Baz: Qux,"]));
        assert!(bbb.consecutive_unspace(&["Foo: Bar, Baz: Qux,"]));
    }
}

#[test]
fn test_require_twrap_attribute() {
    macro_rules! stru {
        ($fields:expr) => (
            concat!("#[repr(transparent)] struct Foo{", $fields, "}")
        )
    }

    {
        let ret  = dft(stru!("")).unwrap_err();
        assert!(ret.consecutive_unspace(&["expected", "#[twrap]"]));
    }
    {
        let ret  = dft(stru!("bar: u32")).unwrap();
        assert!(ret.consecutive_unspace(&["impl", "TransparentNewtype for Foo"]));
    }
    {
        let ret  = dft(stru!("bar: u32, baz: ()")).unwrap_err();
        assert!(ret.consecutive_unspace(&["expected", "#[twrap]"]));
    }
    {
        let ret  = dft(stru!("#[twrap] bar: u32, #[twrap] baz: ()")).unwrap_err();
        assert!(ret.consecutive_unspace(&["#[twrap]", "multiple"]));
    }
    {
        let ret  = dft(stru!("#[twrap(delegate)] bar: u32, #[twrap] baz: ()")).unwrap_err();
        assert!(ret.consecutive_unspace(&["#[twrap]", "multiple"]));
    }
    {
        let ret  = dft(stru!("#[twrap] bar: u32, #[twrap(delegate)] baz: ()")).unwrap_err();
        assert!(ret.consecutive_unspace(&["#[twrap]", "multiple"]));
    }
    {
        let ret  = dft(stru!("bar: u32, baz: (), qux: ()")).unwrap_err();
        assert!(ret.consecutive_unspace(&["expected", "#[twrap]"]));
    }
}

