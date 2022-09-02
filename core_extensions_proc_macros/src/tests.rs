use crate::{
    test_utils::test_proc,
    split_generics,
};

use alloc::string::{String, ToString};


const SPLIT_GENERICS_CASES: &[(&str, &str)] = &[
    (
        r#"foo!() (<'a, T: Foo<X=Y>, const X: u32> (x: u32) {})"#,
        r#"foo!(('a, T: Foo<X=Y>, const X: u32) ((x: u32)) () ({}))"#,
    ),
    (
        r#"foo!() (<T: FnOnce() -> u32 > (x: u32) {})"#,
        r#"foo!((T: FnOnce() -> u32) ((x: u32)) () ({}))"#,
    ),
    (
        r#"foo!() (<const T: [T; x <  y] > (x: u32) {})"#,
        r#"foo!((const T: [T; x <  y]) ((x: u32)) () ({}))"#,
    ),
    (
        r#"foo!() (<T: Foo<{x <  y}> > (x: u32) {})"#,
        r#"foo!((T: Foo<{x <  y}>) ((x: u32)) () ({}))"#,
    ),
    (
        r#"foo!() (<> (x: u32) where T: Foo<{x <  y}> {})"#,
        r#"foo!(() ((x: u32)) (T: Foo<{x <  y}>,) ({}))"#,
    ),
    (
        r#"foo!() (<> (x: u32) where T: FnOnce() -> u32, U: FnOnce() -> u32 {})"#,
        r#"foo!(() ((x: u32)) (T: FnOnce() -> u32, U: FnOnce() -> u32,) ({}))"#,
    ),
    (
        r#"foo!() (<> (x: u32) -> impl I<J = u32> {})"#,
        r#"foo!(() ((x: u32) -> impl I<J = u32>) () ({}))"#,
    ),
    (
        r#"foo!() (<> (x: u32) where {})"#,
        r#"foo!(() ((x: u32)) () ({}))"#,
    ),
    (
        r#"foo!() (<> (x: u32) where u32: {})"#,
        r#"foo!(() ((x: u32)) (u32:,) ({}))"#,
    ),
    (
        r#"foo!() (<> (x: u32) -> X<<>> where u32: {})"#,
        r#"foo!(() ((x: u32) -> X<<>>) (u32:,) ({}))"#,
    ),
    (
        r#"foo!() (<> (x: u32) -> X<Y<>> where u32: {})"#,
        r#"foo!(() ((x: u32) -> X<Y<>>) (u32:,) ({}))"#,
    ),
    (
        r#"foo!() (<> (x: u32) -> X<Y<Z<>>> where u32<>: {})"#,
        r#"foo!(() ((x: u32) -> X<Y<Z<>>>) (u32<>:,) ({}))"#,
    ),
    (
        r#"foo!() (<> (x: u32) where u32: Foo<Bar = u32> {})"#,
        r#"foo!(() ((x: u32)) (u32: Foo<Bar = u32>,) ({}))"#,
    ),
    (
        r#"foo!() (<> (x: u32) where u32: Foo<Bar = u32>; a)"#,
        r#"foo!(() ((x: u32)) (u32: Foo<Bar = u32>,) (; a))"#,
    ),
    (
        r#"foo!() (<> (x: u32) where u32: Foo<Bar = u32> = 0)"#,
        r#"foo!(() ((x: u32)) (u32: Foo<Bar = u32>,) (= 0))"#,
    ),

    // Making sure that unclosed `<` cause the entire string to be contained in the generics
    (
        r#"foo!() (<(x: u32) where u32: Foo<Bar = u32> {})"#,
        r#"foo!(((x: u32) where u32: Foo<Bar = u32> {}) () () ())"#,
    ),
    (
        r#"foo!() ((x: u32) where u32<: Foo<Bar = u32> {})"#,
        r#"foo!(() ((x: u32)) (u32<: Foo<Bar = u32> {}) ())"#,
    ),
];

#[test]
fn split_generics_tests() {
    test_proc(SPLIT_GENERICS_CASES, &|x| split_generics(x));
}


#[cfg(feature = "item_parsing")]
const SPLIT_IMPL_CASES: &[(&str, &str)] = &[
    (
        r#"foo!() (impl dyn for<'a> Trait<'a> {})"#,
        r#"foo!(()()() type(dyn for<'a> Trait<'a>) () ({}) )"#,
    ),
    (
        r#"foo!() (impl for<'a> dyn Trait<'a> {})"#,
        r#"foo!(()()() type(for<'a> dyn Trait<'a>) () ({}) )"#,
    ),
    (
        r#"foo!() (impl for<'a> fn(&'a ()) {})"#,
        r#"foo!(()()() type(for<'a> fn(&'a ())) () ({}) )"#,
    ),
    (
        r#"foo!() (impl Type<'a> {})"#,
        r#"foo!(()()() type(Type<'a>) () ({}) )"#,
    ),
    (
        r#"foo!() (impl Trait<'a> for Foo {})"#,
        r#"foo!(()()() trait(Trait<'a>) type(Foo) () ({}) )"#,
    ),
    (
        r#"foo!() (impl Trait<'a> for for<'a> Foo {})"#,
        r#"foo!(()()() trait(Trait<'a>) type(for<'a> Foo) () ({}) )"#,
    ),
    (
        r#"foo!() (impl for<'a> Trait<'a> for for<'a> Foo {})"#,
        r#"foo!(()()() trait(for<'a> Trait<'a>) type(for<'a> Foo) () ({}) )"#,
    ),
    (
        r#"foo!() (impl for<'a> Trait<'a> for Foo {})"#,
        r#"foo!(()()() trait(for<'a> Trait<'a>) type(Foo) () ({}) )"#,
    ),
];

#[cfg(feature = "item_parsing")]
#[test]
fn split_impl_tests() {
    test_proc(SPLIT_IMPL_CASES, &crate::item_parsing::split_impl);
}



