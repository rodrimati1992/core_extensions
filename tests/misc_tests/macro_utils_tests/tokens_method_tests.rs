use crate::misc_tests::utils_tests::remove_whitespace;

mod default {
    pub const X: &str = "";
}

macro_rules! declare_const {
    (hello $($tt:tt)*) => {pub const X: &str = stringify!($($tt)*); };
}

macro_rules! declare_const_exprs {
    (   hello 
        $(( $(( $($e:expr)? ))* ))*
    ) => {
        pub const X: &str = stringify!( $(( $(( $($e)? ))* ))* ); 
    };
}

macro_rules! assert_with_conster {
    (
        $conster:ident,
        $expected:expr,
        $($args:tt)*
    ) => {{
        mod fooo {
            pub use super::default::*;
            krate::tokens_method!{
                $conster!{hello}
                $($args)*
            }
        }

        assert_eq!(remove_whitespace(fooo::X), remove_whitespace($expected));
    }};
}

macro_rules! assert_tm {
    ( $($args:tt)* ) => { assert_with_conster!{declare_const, $($args)*} };
}

macro_rules! assert_tm_exprs {
    ( $($args:tt)* ) => { assert_with_conster!{declare_const_exprs, $($args)*} };
}


#[test]
fn first_test() {
    assert_tm!{"()", first: ()}
    assert_tm!{"(3)", first: (3)}
    assert_tm!{"(3)", first: (3 5)}
    assert_tm!{"((3 5))", first: ((3 5) 8 13)}
}


#[test]
fn last_test() {
    assert_tm!{"()", last: ()}
    assert_tm!{"(3)", last: (3)}
    assert_tm!{"(5)", last: (3 5)}
    assert_tm!{"((8 13))", last: (3 5 (8 13))}
}

#[test]
fn split_first_test() {
    assert_tm!{"() ()", split_first: ()}
    assert_tm!{"(3) ()", split_first: (3)}
    assert_tm!{"(3) (5)", split_first: (3 5)}
    assert_tm!{"((3 5)) (8 13)", split_first: ((3 5) 8 13)}
}

#[test]
fn split_last_test() {
    assert_tm!{"() ()", split_last: ()}
    assert_tm!{"() (3)", split_last: (3)}
    assert_tm!{"(3) (5)", split_last: (3 5)}
    assert_tm!{"(3 5) ((8 13))", split_last: (3 5 (8 13))}
}

#[test]
fn split_last_n_test() {
    assert_tm!{"() ()", split_last_n(0): ()}
    assert_tm!{"(3) ()", split_last_n(0): (3)}
    assert_tm!{"(3 5) ()", split_last_n(0): (3 5)}
    assert_tm!{"(3 5 (8 13)) ()", split_last_n(0): (3 5 (8 13))}
    assert_tm!{"(3 5 (8 13) {21 34}) ()", split_last_n(0): (3 5 (8 13) {21 34})}

    assert_tm!{"() ()", split_last_n(1): ()}
    assert_tm!{"() (3)", split_last_n(1): (3)}
    assert_tm!{"(3) (5)", split_last_n(1): (3 5)}
    assert_tm!{"(3 5) ((8 13))", split_last_n(1): (3 5 (8 13))}

    assert_tm!{"() ()", split_last_n(2): ()}
    assert_tm!{"() (3)", split_last_n(2): (3)}
    assert_tm!{"() (3 5)", split_last_n(2): (3 5)}
    assert_tm!{"(3) (5 (8 13))", split_last_n(2): (3 5 (8 13))}
    assert_tm!{"(3 5) ((8 13) {21 34})", split_last_n(2): (3 5 (8 13) {21 34})}
}

#[test]
fn split_at_test() {
    assert_tm!{"() ()", split_at(0): ()}
    assert_tm!{"() (3)", split_at(0): (3)}
    assert_tm!{"() (3 5)", split_at(0): (3 5)}
    assert_tm!{"() (3 5 (8 13))", split_at(0): (3 5 (8 13))}
    assert_tm!{"() (3 5 (8 13) {21 34})", split_at(0): (3 5 (8 13) {21 34})}

    assert_tm!{"() ()", split_at(1): ()}
    assert_tm!{"(3) ()", split_at(1): (3)}
    assert_tm!{"(3) (5)", split_at(1): (3 5)}
    assert_tm!{"(3) (5 (8 13))", split_at(1): (3 5 (8 13))}

    assert_tm!{"() ()", split_at(2): ()}
    assert_tm!{"(3) ()", split_at(2): (3)}
    assert_tm!{"(3 5) ()", split_at(2): (3 5)}
    assert_tm!{"(3 5) ((8 13))", split_at(2): (3 5 (8 13))}
    assert_tm!{"(3 5) ((8 13) {21 34})", split_at(2): (3 5 (8 13) {21 34})}
}

#[test]
fn get_test() {
    assert_tm!{"(3)", get(1): range(2..)}
    assert_tm!{"(3 4 5)", get(1..=3): range(2..)}
    assert_tm!{"(2 3 4 5)", get(..=3): range(2..)}

    assert_tm!{"(3)", get(0): (3 5 (8 13) {21 34} 55)}
    assert_tm!{"(5)", get(1): (3 5 (8 13) {21 34} 55)}
    assert_tm!{"((8 13))", get(2): (3 5 (8 13) {21 34} 55)}
    assert_tm!{"({21 34})", get(3): (3 5 (8 13) {21 34} 55)}
    assert_tm!{"(55)", get(4): (3 5 (8 13) {21 34} 55)}

    assert_tm!{"(3 5 (8 13) {21 34} 55)", get(..): (3 5 (8 13) {21 34} 55)}
    assert_tm!{"(3 5)", get(0..2): (3 5 (8 13) {21 34} 55)}
    assert_tm!{"(3 5)", get(..2): (3 5 (8 13) {21 34} 55)}
    assert_tm!{"((8 13) {21 34})", get(2..4): (3 5 (8 13) {21 34} 55)}
    assert_tm!{"((8 13) {21 34} 55)", get(2..): (3 5 (8 13) {21 34} 55)}
    assert_tm!{"((8 13) {21 34} 55)", get(2..100000): (3 5 (8 13) {21 34} 55)}
    assert_tm!{"((8 13) {21 34} 55)", get(2..=100000): (3 5 (8 13) {21 34} 55)}
    assert_tm!{"(3 5)", get(0..=1): (3 5 (8 13) {21 34} 55)}
    assert_tm!{"(3 5)", get(..=1): (3 5 (8 13) {21 34} 55)}
}

#[test]
fn split_test() {
    assert_tm!{"() (1+1) (2+2) (3+3)", split(=): ( = 1 + 1 = 2 + 2 = 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3) ()", split(=): (1 + 1 = 2 + 2 = 3 + 3 =)}
    assert_tm!{"(1+1) (2+2) (3+3)", split(=): (1 + 1 = 2 + 2 = 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split(==): (1 + 1 == 2 + 2 == 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split(foo): (1 + 1 foo 2 + 2 foo 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split(foo bar): (1 + 1 foo bar 2 + 2 foo bar 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split((foo)): (1 + 1 (foo) 2 + 2 (foo) 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split("hello"): (1 + 1 "hello" 2 + 2 "hello" 3 + 3)}
}

#[test]
fn split_terminator_test() {
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator(=): (1 + 1 = 2 + 2 = 3 + 3 =)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator(=): (1 + 1 = 2 + 2 = 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator(==): (1 + 1 == 2 + 2 == 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator(foo): (1 + 1 foo 2 + 2 foo 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator((foo)): (1 + 1 (foo) 2 + 2 (foo) 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator(foo bar): (1 + 1 foo bar 2 + 2 foo bar 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator("hello"): (1 + 1 "hello" 2 + 2 "hello" 3 + 3)}
}

#[test]
fn split_starter_test() {
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter(=): (= 1 + 1 = 2 + 2 = 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter(=): (1 + 1 = 2 + 2 = 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter(==): (1 + 1 == 2 + 2 == 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter(foo): (1 + 1 foo 2 + 2 foo 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter((foo)): (1 + 1 (foo) 2 + 2 (foo) 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter(foo bar): (1 + 1 foo bar 2 + 2 foo bar 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter("hello"): (1 + 1 "hello" 2 + 2 "hello" 3 + 3)}
}

macro_rules! test_zip_fn {
    (
        $zip_fn:ident ($(( $($e:expr),* ))*)
        expected($expected:expr)
    ) => {{
        assert_tm_exprs!{
            $expected,
            $zip_fn: $(( $($e)* ))*
        }
    }};
}

#[test]
fn zip_shortest_test() {
    assert_tm!{
        "((A)) ((B)) ((C)) ((D)) ((E)) ((F))",
        zip_shortest: (A B C D E F)
    }
    assert_tm!{
        "
            ((fooA) (barA))
            ((fooB) (barB))
            ((fooC) (barC))
            ((fooD) (barD))
            ((fooE) (barE))
        ",
        zip_shortest:
            (fooA fooB fooC fooD fooE)
            (barA barB barC barD barE barF)
        
    }
    assert_tm!{
        "
            ((fooA) (barA))
            ((fooB) (barB))
            ((fooC) (barC))
            ((fooD) (barD))
            ((fooE) (barE))
            ((fooF) (barF))
        ",
        zip_shortest:
            (fooA fooB fooC fooD fooE fooF)
            (barA barB barC barD barE barF)
        
    }
    assert_tm!{
        "
            ((fooA) (barA) (bazA))
            ((fooB) (barB) (bazB))
            ((fooC) (barC) (bazC))
            ((fooD) (barD) (bazD))
            ((fooE) (barE) (bazE))
        ",
        zip_shortest:
            (fooA fooB fooC fooD fooE)
            (barA barB barC barD barE barF)
            (bazA bazB bazC bazD bazE bazF)
        
    }
    assert_tm!{
        "
            ((fooA) (barA) (bazA))
            ((fooB) (barB) (bazB))
            ((fooC) (barC) (bazC))
            ((fooD) (barD) (bazD))
            ((fooE) (barE) (bazE))
            ((fooF) (barF) (bazF))
        ",
        zip_shortest:
            (fooA fooB fooC fooD fooE fooF)
            (barA barB barC barD barE barF)
            (bazA bazB bazC bazD bazE bazF)
    }


    test_zip_fn!{
        zip_shortest(
            (foo(), bar + baz, aaa * bbb / ccc)
            (fff().ggg(), hhh())
        )
        expected("
            ((foo()) (fff().ggg()))
            ((bar + baz) (hhh()))
        ")
    }
}

#[test]
fn zip_longest_test() {
    assert_tm!{
        "((A)) ((B)) ((C)) ((D)) ((E)) ((F))",
        zip_longest: (A B C D E F)
    }
    assert_tm!{
        "
            ((fooA) (barA))
            ((fooB) (barB))
            ((fooC) (barC))
            ((fooD) (barD))
            ((fooE) (barE))
            (() (barF))
        ",
        zip_longest: 
            (fooA fooB fooC fooD fooE)
            (barA barB barC barD barE barF)
        
    }
    assert_tm!{
        "
            ((fooA) (barA))
            ((fooB) (barB))
            ((fooC) (barC))
            ((fooD) (barD))
            ((fooE) (barE))
            ((fooF) (barF))
        ",
        zip_longest: 
            (fooA fooB fooC fooD fooE fooF)
            (barA barB barC barD barE barF)
        
    }
    assert_tm!{
        "
            ((fooA) (barA) (bazA))
            ((fooB) (barB) (bazB))
            ((fooC) (barC) (bazC))
            ((fooD) (barD) (bazD))
            ((fooE) (barE) (bazE))
            (() (barF) (bazF))
            (() (barG) (bazG))
        ",
        zip_longest: 
            (fooA fooB fooC fooD fooE)
            (barA barB barC barD barE barF barG)
            (bazA bazB bazC bazD bazE bazF bazG)
        
    }
    assert_tm!{
        "
            ((fooA) (barA) (bazA))
            ((fooB) (barB) (bazB))
            ((fooC) (barC) (bazC))
            ((fooD) (barD) (bazD))
            ((fooE) (barE) (bazE))
            ((fooF) (barF) (bazF))
            ((fooG) (barG) (bazG))
        ",
        zip_longest: 
            (fooA fooB fooC fooD fooE fooF fooG)
            (barA barB barC barD barE barF barG)
            (bazA bazB bazC bazD bazE bazF bazG)
    }

    test_zip_fn!{
        zip_longest(
            (foo(), bar + baz, aaa * bbb / ccc)
            (fff().ggg(), hhh())
        )
        expected("
            ((foo()) (fff().ggg()))
            ((bar + baz) (hhh()))
            ((aaa * bbb / ccc) ())
        ")
    }
}


#[test]
fn cycle_iter_test() {
    assert_tm!{
        "
            ((fooA) (0))
            ((fooB) (1))
            ((fooC) (2))
            ((fooD) (0))
            ((fooE) (1))
            ((fooF) (2))
            ((fooG) (0))
            ((fooH) (1))
        ",
        zip_shortest: 
            (fooA fooB fooC fooD fooE fooF fooG fooH)
            cycle(range(0..3))
    }
    assert_tm!{
        "
            ((fooA) (foo))
            ((fooB) (bar))
            ((fooC) (qux))
            ((fooD) (foo))
            ((fooE) (bar))
            ((fooF) (qux))
            ((fooG) (foo))
            ((fooH) (bar))
        ",
        zip_longest: 
            (fooA fooB fooC fooD fooE fooF fooG fooH)
            cycle((foo bar qux))
    }
}


#[test]
fn repeat_iter_test() {
    assert_tm!{
        "
            ((fooA) (0))
            ((fooB) (1))
            ((fooC) (2))
            ((fooD) (0))
            ((fooE) (1))
            ((fooF) (2))
            ((fooG) (0))
            ((fooH) (1))
            ((fooI) (2))
            ((fooJ) ())
            ((fooK) ())
        ",
        zip_longest: 
            (fooA fooB fooC fooD fooE fooF fooG fooH fooI fooJ fooK)
            repeat(3, (0 1 2))
    }
    assert_tm!{
        "(0 1 2 0 1 2 0 1 2)",
        iterate: repeat(3, (0 1 2))
    }
}


#[test]
fn take_iter_test() {
    assert_tm!{
        "
            (a b c d 5 6 7 8 9 10)
        ",
        iterate: 
            take(10, chain((a b c d) range(5..)))
    }
    assert_tm!{
        "
            ((a) (5))
            ((b) (6))
            ((c) (7))
            ((d) (8))
            ((e) (9))
            ((f) (10))
            ((g) ())
            ((h) ())
        ",
        zip_longest: 
            (a b c d e f g h)
            take(6, range(5..))
    }
}

#[test]
fn skip_iter_test() {
    assert_tm!{
        "(c d 5 6 7 8 9 10)",
        iterate: skip(2, chain((a b c d) range(5..=10)))
    }
    assert_tm!{
        "(6 7 8 9 10)",
        iterate: skip(5, chain((a b c d) range(5..=10)))
    }
    assert_tm!{
        "
            ((a) (11))
            ((b) (12))
            ((c) (13))
            ((d) (14))
            ((e) (15))
            ((f) (16))
            ((g) (17))
            ((h) (18))
        ",
        zip_longest: 
            (a b c d e f g h)
            skip(6, range(5..))
    }
}


#[test]
fn chain_iter_test() {
    assert_tm!{
        "(a b c d 5 6)",
        iterate: chain((a b c d) range(5..7))
    }
    assert_tm!{
        "
            ((a) (0))
            ((b) (1))
            ((c) (2))
            ((d) (3))
            ((e) (6))
            ((f) (7))
            ((g) (8))
            ((h) (foo))
            (() (bar))
            (() (baz))
        ",
        zip_longest: 
            (a b c d e f g h)
            chain(range(0..=3) range(6..=8) (foo bar baz))
    }
    assert_tm!{
        "
            ((a) (0))
            ((b) (1))
            ((c) (2))
            ((d) (3))
            ((e) (6))
            ((f) (7))
            ((g) (8))
            ((h) (9))
        ",
        zip_longest: 
            (a b c d e f g h)
            chain(range(0..=3) range(6..))
    }
}

#[test]
fn gen_ident_range_iter_test() {
    assert_tm!{
        "(f0 f1 f2 f3)",
        iterate: gen_ident_range(for f* in 0..=3)
    }
    assert_tm!{
        "
            ((a) (f10))
            ((b) (f11))
            ((c) (f12))
            ((d) (f13))
            ((e) (f14))
            ((f) (f15))
            (() (f16))
            (() (f17))
            (() (f18))
        ",
        zip_longest: 
            (a b c d e f)
            gen_ident_range(for f* in 10..=18)
    }
    assert_tm!{
        "
            ((a) (f10))
            ((b) (f11))
            ((c) (f12))
            ((d) (f13))
            ((e) (f14))
            ((f) (f15))
        ",
        zip_longest: 
            (a b c d e f)
            gen_ident_range(for f* in 10..)
    }
}












