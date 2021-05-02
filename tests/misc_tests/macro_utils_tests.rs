use krate::{count_tts, gen_ident_range, rewrap_macro_parameters};


mod tokens_method_tests;


#[test]
fn different_delimiters() {
    macro_rules! assertion_1 {
        (
            (hello world)
            [foo bar]
            {aaa bbb ccc}
            (1 + 1) 
            (fn foo(){})
            (Vec<u32>)
            (didnt)
        ) => {
            pub const S: &'static str = "matched";
        };
        ($($tt:tt)*) => { compile_error!{stringify!($($tt)*)} };
    }

    macro_rules! assertion_2 {
        (($($t:tt)*)) => { pub const S: &'static str = "no match"; };
        (1 + 1) => {
            pub const S: &'static str = "matched";
        };
    }

    macro_rules! somemacro {
        (
            $hello:expr,
            $item:item,
            $ty:ty,
            $ident:ident,
        ) => {
            mod foo {
                use super::*;
                rewrap_macro_parameters!{
                    assertion_1 ! {
                        ~(hello world) ~[foo bar] ~{aaa bbb ccc} 
                        ~$hello 
                        ~$item
                        ~$ty
                        ~$ident
                    }
                }

            }
            assert_eq!(foo::S, "matched");
            
            mod bar{
                use super::*;
                rewrap_macro_parameters!{assertion_2!{$hello}}
            }
            assert_eq!(bar::S, "matched");
        };
    }

    somemacro!{
        1 + 1,
        fn foo() {},
        Vec<u32>,
        didnt,
    }

}


#[test]
fn count_tts_test() {
    macro_rules! assert_count {
        (3 3) => {};
        (4 4) => {};
        (5 5) => {};
        (6 6) => {};
        (7 7) => {};
        (8 8) => {};
        (9 9) => {};
    }

    macro_rules! count_tests {
        (
            $expected_expr:tt ($($t_expr:expr),*),
            $expected_ty:tt ($($t_ty:ty),*),
            $expected_path:tt ($($t_path:path),*),
            $expected_item:tt ($($t_item:item)*),
            $expected_tt:tt ($($t_tt:tt)*),
        ) => {
            mod __{
                use super::*;
                count_tts!{assert_count!{$expected_expr} ($($t_expr)*)}
                count_tts!{assert_count!{$expected_ty} ($($t_ty)*)}
                count_tts!{assert_count!{$expected_path} ($($t_path)*)}
                count_tts!{assert_count!{$expected_item} ($($t_item)*)}
                count_tts!{assert_count!{$expected_tt} ($($t_tt)*)}
            }
            const _: [(); $expected_expr] = [(); count_tts!(($($t_expr)*))];
            const _: [(); $expected_ty] = [(); count_tts!(($($t_ty)*))];
            const _: [(); $expected_path] = [(); count_tts!(($($t_path)*))];
            const _: [(); $expected_item] = [(); count_tts!(($($t_item)*))];
            const _: [(); $expected_tt] = [(); count_tts!(($($t_tt)*))];

        };
    }

    count_tests!{
        3 (1 + 1, f(10) - 1, a * b / c),
        4 (Vec<T>, HashMap<i32, i32>, u64, dyn A + B),
        5 (::a::b, c, crate::d::<e>::f, g::h, ::i),
        6 (
            const _: () = ();
            impl Foo {}
            fn bar() -> u32 {}
            type X = Y;
            struct Bar;
            enum Baz {}
        ),
        7 (_1 _2 _3 _4 _5 _6 _7),
    }

    // Making sure that the constant is a usize
    fn type_name_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }

    assert!(type_name_of(count_tts!(())).contains("usize"));
    assert!(type_name_of(count_tts!((_))).contains("usize"));

}


#[test]
fn gen_idents_test() {}

mod gen_idents_test {
    use super::*;

    macro_rules! assert_idents {
        ($expected:tt $found:tt) => {
            macro_rules! assertion {
                ($expected) => {};
            }
            assertion!($found);
        };
    }

    macro_rules! ident_test {
        (
            ($count_expr:literal) $expected_expr:tt ($($t_expr:expr),*),
            $count_ty:tt $expected_ty:tt ($($t_ty:ty),*),
            ($count_path:expr) $expected_path:tt ($($t_path:path),*),
            $count_item:tt $expected_item:tt ($($t_item:item)*),
            $count_tt:tt $expected_tt:tt ($($t_tt:tt)*),
        ) => {
            gen_ident_range!{assert_idents!{$expected_expr} for f* in 0..$count_expr}
            gen_ident_range!{assert_idents!{$expected_expr} for f* in 0..count($($t_expr)*)}
            
            gen_ident_range!{assert_idents!{$expected_ty} for f* in 0..$count_ty}
            gen_ident_range!{assert_idents!{$expected_ty} for f* in 0..count($($t_ty)*)}
            
            gen_ident_range!{assert_idents!{$expected_path} for f* in 0..$count_path}
            gen_ident_range!{assert_idents!{$expected_path} for f* in 0..count($($t_path)*)}
            
            gen_ident_range!{assert_idents!{$expected_item} for f* in 0..$count_item}
            gen_ident_range!{assert_idents!{$expected_item} for f* in 0..count($($t_item)*)}
            
            gen_ident_range!{assert_idents!{$expected_tt} for f* in 0..$count_tt}
            gen_ident_range!{assert_idents!{$expected_tt} for f* in 0..count($($t_tt)*)}
        };
    }

    ident_test!{
        (3) (f0 f1 f2) (1 + 1, f(10) - 1, a * b / c),
        4 (f0 f1 f2 f3) (Vec<T>, HashMap<i32, i32>, u64, dyn A + B),
        (5) (f0 f1 f2 f3 f4) (::a::b, c, crate::d::<e>::f, g::h, ::i),
        6 (f0 f1 f2 f3 f4 f5) (
            const _: () = ();
            impl Foo {}
            fn bar() -> u32 {}
            type X = Y;
            struct Bar;
            enum Baz {}
        ),
        7 (f0 f1 f2 f3 f4 f5 f6) (_1 _2 _3 _4 _5 _6 _7),
    }
    gen_ident_range!{assert_idents!{(a0 a1 a2 a3)} for a* in 0..=3}
    gen_ident_range!{assert_idents!{(b2 b3 b4)} for b* in 2..=count(_ _ _ _)}
    gen_ident_range!{assert_idents!{(c2 c3 c4)} for c* in count(_ _)..=4}
    gen_ident_range!{assert_idents!{(d2 d3 d4)} for d* in count(_ _)..=count(_ _ _ _)}
}
