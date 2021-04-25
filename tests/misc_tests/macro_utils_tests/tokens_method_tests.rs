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
    assert_tm!{"()", first ()}
    assert_tm!{"(3)", first (3)}
    assert_tm!{"(3)", first (3 5)}
    assert_tm!{"((3 5))", first ((3 5) 8 13)}
}


#[test]
fn last_test() {
    assert_tm!{"()", last ()}
    assert_tm!{"(3)", last (3)}
    assert_tm!{"(5)", last (3 5)}
    assert_tm!{"((8 13))", last (3 5 (8 13))}
}

#[test]
fn split_first_test() {
    assert_tm!{"() ()", split_first ()}
    assert_tm!{"(3) ()", split_first (3)}
    assert_tm!{"(3) (5)", split_first (3 5)}
    assert_tm!{"((3 5)) (8 13)", split_first ((3 5) 8 13)}
}

#[test]
fn split_last_test() {
    assert_tm!{"() ()", split_last ()}
    assert_tm!{"() (3)", split_last (3)}
    assert_tm!{"(3) (5)", split_last (3 5)}
    assert_tm!{"(3 5) ((8 13))", split_last (3 5 (8 13))}
}

#[test]
fn split_last_n_test() {
    assert_tm!{"() ()", split_last_n(0) ()}
    assert_tm!{"(3) ()", split_last_n(0) (3)}
    assert_tm!{"(3 5) ()", split_last_n(0) (3 5)}
    assert_tm!{"(3 5 (8 13)) ()", split_last_n(0) (3 5 (8 13))}
    assert_tm!{"(3 5 (8 13) {21 34}) ()", split_last_n(0) (3 5 (8 13) {21 34})}

    assert_tm!{"() ()", split_last_n(1) ()}
    assert_tm!{"() (3)", split_last_n(1) (3)}
    assert_tm!{"(3) (5)", split_last_n(1) (3 5)}
    assert_tm!{"(3 5) ((8 13))", split_last_n(1) (3 5 (8 13))}

    assert_tm!{"() ()", split_last_n(2) ()}
    assert_tm!{"() (3)", split_last_n(2) (3)}
    assert_tm!{"() (3 5)", split_last_n(2) (3 5)}
    assert_tm!{"(3) (5 (8 13))", split_last_n(2) (3 5 (8 13))}
    assert_tm!{"(3 5) ((8 13) {21 34})", split_last_n(2) (3 5 (8 13) {21 34})}
}

#[test]
fn split_at_test() {
    assert_tm!{"() ()", split_at(0) ()}
    assert_tm!{"() (3)", split_at(0) (3)}
    assert_tm!{"() (3 5)", split_at(0) (3 5)}
    assert_tm!{"() (3 5 (8 13))", split_at(0) (3 5 (8 13))}
    assert_tm!{"() (3 5 (8 13) {21 34})", split_at(0) (3 5 (8 13) {21 34})}

    assert_tm!{"() ()", split_at(1) ()}
    assert_tm!{"(3) ()", split_at(1) (3)}
    assert_tm!{"(3) (5)", split_at(1) (3 5)}
    assert_tm!{"(3) (5 (8 13))", split_at(1) (3 5 (8 13))}

    assert_tm!{"() ()", split_at(2) ()}
    assert_tm!{"(3) ()", split_at(2) (3)}
    assert_tm!{"(3 5) ()", split_at(2) (3 5)}
    assert_tm!{"(3 5) ((8 13))", split_at(2) (3 5 (8 13))}
    assert_tm!{"(3 5) ((8 13) {21 34})", split_at(2) (3 5 (8 13) {21 34})}
}

#[test]
fn get_test() {
    assert_tm!{"(3)", get(0) (3 5 (8 13) {21 34} 55)}
    assert_tm!{"(5)", get(1) (3 5 (8 13) {21 34} 55)}
    assert_tm!{"((8 13))", get(2) (3 5 (8 13) {21 34} 55)}
    assert_tm!{"({21 34})", get(3) (3 5 (8 13) {21 34} 55)}
    assert_tm!{"(55)", get(4) (3 5 (8 13) {21 34} 55)}

    assert_tm!{"(3 5 (8 13) {21 34} 55)", get(..) (3 5 (8 13) {21 34} 55)}
    assert_tm!{"(3 5)", get(0..2) (3 5 (8 13) {21 34} 55)}
    assert_tm!{"(3 5)", get(..2) (3 5 (8 13) {21 34} 55)}
    assert_tm!{"((8 13) {21 34})", get(2..4) (3 5 (8 13) {21 34} 55)}
    assert_tm!{"((8 13) {21 34} 55)", get(2..) (3 5 (8 13) {21 34} 55)}
    assert_tm!{"((8 13) {21 34} 55)", get(2..100000) (3 5 (8 13) {21 34} 55)}
    assert_tm!{"((8 13) {21 34} 55)", get(2..=100000) (3 5 (8 13) {21 34} 55)}
    assert_tm!{"(3 5)", get(0..=1) (3 5 (8 13) {21 34} 55)}
    assert_tm!{"(3 5)", get(..=1) (3 5 (8 13) {21 34} 55)}
}

#[test]
fn split_test() {
    assert_tm!{"() (1+1) (2+2) (3+3)", split(=) ( = 1 + 1 = 2 + 2 = 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3) ()", split(=) (1 + 1 = 2 + 2 = 3 + 3 =)}
    assert_tm!{"(1+1) (2+2) (3+3)", split(=) (1 + 1 = 2 + 2 = 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split(==) (1 + 1 == 2 + 2 == 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split(foo) (1 + 1 foo 2 + 2 foo 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split(foo bar) (1 + 1 foo bar 2 + 2 foo bar 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split((foo)) (1 + 1 (foo) 2 + 2 (foo) 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split("hello") (1 + 1 "hello" 2 + 2 "hello" 3 + 3)}
}

#[test]
fn split_terminator_test() {
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator(=) (1 + 1 = 2 + 2 = 3 + 3 =)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator(=) (1 + 1 = 2 + 2 = 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator(==) (1 + 1 == 2 + 2 == 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator(foo) (1 + 1 foo 2 + 2 foo 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator((foo)) (1 + 1 (foo) 2 + 2 (foo) 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator(foo bar) (1 + 1 foo bar 2 + 2 foo bar 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_terminator("hello") (1 + 1 "hello" 2 + 2 "hello" 3 + 3)}
}

#[test]
fn split_starter_test() {
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter(=) (= 1 + 1 = 2 + 2 = 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter(=) (1 + 1 = 2 + 2 = 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter(==) (1 + 1 == 2 + 2 == 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter(foo) (1 + 1 foo 2 + 2 foo 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter((foo)) (1 + 1 (foo) 2 + 2 (foo) 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter(foo bar) (1 + 1 foo bar 2 + 2 foo bar 3 + 3)}
    assert_tm!{"(1+1) (2+2) (3+3)", split_starter("hello") (1 + 1 "hello" 2 + 2 "hello" 3 + 3)}
}

macro_rules! test_zip_fn {
    (
        $zip_fn:ident ($(( $($e:expr),* ))*)
        expected($expected:expr)
    ) => {{
        assert_tm_exprs!{
            $expected,
            $zip_fn $(( $($e)* ))*
        }
    }};
}

#[test]
fn zip_shortest_test() {
    assert_tm!{
        "((A)) ((B)) ((C)) ((D)) ((E)) ((F))",
        zip_shortest (A B C D E F)
    }
    assert_tm!{
        "
            ((fooA) (barA))
            ((fooB) (barB))
            ((fooC) (barC))
            ((fooD) (barD))
            ((fooE) (barE))
        ",
        zip_shortest 
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
        zip_shortest 
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
        zip_shortest 
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
        zip_shortest 
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
        zip_longest (A B C D E F)
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
        zip_longest 
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
        zip_longest 
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
        zip_longest 
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
        zip_longest 
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