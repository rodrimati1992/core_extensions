use crate::split_generics;

use alloc::string::{String, ToString};


const CASES: &[(&str, &str)] = &[
    (
        r#"(<'a, T: Foo<X=Y>, const X: u32> (x: u32) {}) foo!()"#,
        r#"foo!(('a, T: Foo<X=Y>, const X: u32) ((x: u32)) () ({}))"#,
    ),
    (
        r#"(<T: FnOnce() -> u32 > (x: u32) {}) foo!()"#,
        r#"foo!((T: FnOnce() -> u32) ((x: u32)) () ({}))"#,
    ),
    (
        r#"(<const T: [T; x <  y] > (x: u32) {}) foo!()"#,
        r#"foo!((const T: [T; x <  y]) ((x: u32)) () ({}))"#,
    ),
    (
        r#"(<T: Foo<{x <  y}> > (x: u32) {}) foo!()"#,
        r#"foo!((T: Foo<{x <  y}>) ((x: u32)) () ({}))"#,
    ),
    (
        r#"(<> (x: u32) where T: Foo<{x <  y}> {}) foo!()"#,
        r#"foo!(() ((x: u32)) (T: Foo<{x <  y}>) ({}))"#,
    ),
    (
        r#"(<> (x: u32) where T: FnOnce() -> u32, U: FnOnce() -> u32 {}) foo!()"#,
        r#"foo!(() ((x: u32)) (T: FnOnce() -> u32, U: FnOnce() -> u32) ({}))"#,
    ),
    (
        r#"(<> (x: u32) -> impl I<J = u32> {}) foo!()"#,
        r#"foo!(() ((x: u32) -> impl I<J = u32>) () ({}))"#,
    ),
    (
        r#"(<> (x: u32) where {}) foo!()"#,
        r#"foo!(() ((x: u32)) () ({}))"#,
    ),
    (
        r#"(<> (x: u32) where u32: {}) foo!()"#,
        r#"foo!(() ((x: u32)) (u32:) ({}))"#,
    ),
    (
        r#"(<> (x: u32) -> X<<>> where u32: {}) foo!()"#,
        r#"foo!(() ((x: u32) -> X<<>>) (u32:) ({}))"#,
    ),
    (
        r#"(<> (x: u32) -> X<Y<>> where u32: {}) foo!()"#,
        r#"foo!(() ((x: u32) -> X<Y<>>) (u32:) ({}))"#,
    ),
    (
        r#"(<> (x: u32) -> X<Y<Z<>>> where u32<>: {}) foo!()"#,
        r#"foo!(() ((x: u32) -> X<Y<Z<>>>) (u32<>:) ({}))"#,
    ),
    (
        r#"(<> (x: u32) where u32: Foo<Bar = u32> {}) foo!()"#,
        r#"foo!(() ((x: u32)) (u32: Foo<Bar = u32>) ({}))"#,
    ),
    (
        r#"(<> (x: u32) where u32: Foo<Bar = u32>; a) foo!()"#,
        r#"foo!(() ((x: u32)) (u32: Foo<Bar = u32>) (; a))"#,
    ),
    (
        r#"(<> (x: u32) where u32: Foo<Bar = u32> = 0) foo!()"#,
        r#"foo!(() ((x: u32)) (u32: Foo<Bar = u32>) (= 0))"#,
    ),

    // Making sure that unclosed `<` cause the entire string to be contained in the generics
    (
        r#"(<(x: u32) where u32: Foo<Bar = u32> {}) foo!()"#,
        r#"foo!(((x: u32) where u32: Foo<Bar = u32> {}) () () ())"#,
    ),
    (
        r#"((x: u32) where u32<: Foo<Bar = u32> {}) foo!()"#,
        r#"foo!(() ((x: u32)) (u32<: Foo<Bar = u32> {}) ())"#,
    ),
];

#[test]
fn split_generics_tests() {
    for (input, expected) in CASES {
        let string = remove_whitespaces(&split_generics(input.parse().unwrap()).to_string());
        let expected = remove_whitespaces(expected);

        assert_eq!(string, expected, "\ninput   : {}\nexpected: {}", input, expected);
    }
}



fn remove_whitespaces(x: &str) -> String {
    x.chars().filter(|x| !x.is_whitespace()).collect()
}


