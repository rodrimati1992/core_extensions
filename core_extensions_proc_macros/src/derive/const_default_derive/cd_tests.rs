use crate::test_utils::{TestStrExt, remove_whitespaces};

use super::derive_for_tests as dft;

use alloc::format;


#[test]
fn test_crate_attr() {
    #[cfg_attr(feature = "rust_1_46", track_caller)]
    fn assert_renamed(in_: &str, expected: &str) {
        let ret = dft(&format!("{} struct Foo;", in_)).unwrap();
        assert!(ret.consecutive_unspace(&[expected]));
        assert_eq!(ret.matches("use").count(), 1);
    }
    assert_renamed("", "use ::core_extensions");
    assert_renamed("#[cdef(crate = foo::bar)]", "use foo::bar");
    assert_renamed("#[cdef(crate = ::foo::bar)]", "use ::foo::bar");
}

#[test]
fn test_bound_attr() {
    {
        let ret = dft("#[cdef(bound(U: ))] struct Foo<T>(T);").unwrap_err();
        assert!(ret.consecutive_unspace(&["expected", "type parameter"]));
    }
    {
        let ret = dft("#[cdef(bound(T: ))] struct Foo<T>(T);").unwrap();
        assert!(ret.consecutive_unspace(&["T:,"]));
    }
    {
        let ret = dft("#[cdef(bound(T: Foo + Bar))] struct Foo<T>(T);").unwrap();
        assert!(ret.consecutive_unspace(&["T: Foo + Bar,"]));
    }
}

#[test]
fn test_where_attr() {
    {
        let left  = dft("struct Foo<T>(T);").unwrap();
        let right = dft("#[cdef(where)] struct Foo<T>(T);").unwrap();
        assert_eq!(remove_whitespaces(&left), remove_whitespaces(&right));
    }
    {
        let aaa = dft("#[cdef(where Foo: Bar<{baz}>)] struct Foo<T>(T);").unwrap();
        let bbb = dft("#[cdef(where Foo: Bar<{baz}>,)] struct Foo<T>(T);").unwrap();
        assert!(aaa.consecutive_unspace(&["Foo: Bar<{baz}>,"]), "{}", aaa);
        assert!(bbb.consecutive_unspace(&["Foo: Bar<{baz}>,"]));
    }
    {
        let aaa = dft("#[cdef(where Foo: Bar, Baz: Qux)] struct Foo<T>(T);").unwrap();
        let bbb = dft("#[cdef(where Foo: Bar, Baz: Qux,)] struct Foo<T>(T);").unwrap();
        assert!(aaa.consecutive_unspace(&["Foo: Bar, Baz: Qux,"]));
        assert!(bbb.consecutive_unspace(&["Foo: Bar, Baz: Qux,"]));
    }
}



#[test]
fn test_default_variant_attr() {
    {
        let ret  = dft("enum Foo{Bar, Baz}").unwrap_err();
        assert!(ret.consecutive_unspace(&["expected", "#[cdef(default)]"]));
    }
    {
        let ret = dft("enum Foo{#[cdef(default)] Bar, Baz}").unwrap();
        assert!(ret.consecutive_unspace(&["impl", "ConstDefault", "for Foo","::Bar"]));
    }
    {
        let ret = dft("enum Foo{Bar, #[cdef(default)] Baz}").unwrap();
        assert!(ret.consecutive_unspace(&["impl", "ConstDefault", "for Foo","::Baz"]));
    }
}

