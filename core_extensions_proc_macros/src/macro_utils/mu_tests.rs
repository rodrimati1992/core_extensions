use super::tokens_method;

use crate::{
    test_utils::test_try_proc,
    Error,
};

use alloc::string::{String, ToString};


const UNBOUNDED_ERR_CASES: &[(&str, &str)] = &[
    ("f!() last: range(1..)", "Expected a bounded"),
    ("f!() split_first: range(1..)", "Expected a bounded"),
    ("f!() split_last: range(1..)", "Expected a bounded"),
    ("f!() split_last_n(4): range(1..)", "Expected a bounded"),
    ("f!() split_at(5): range(1..)", "Expected a bounded"),
    ("f!() get(..): range(1..)", "Expected a bounded"),
    ("f!() get(5..): range(1..)", "Expected a bounded"),
    ("f!() split(=): range(1..)", "Expected a bounded"),
    ("f!() split_terminator(=): range(1..)", "Expected a bounded"),
    ("f!() split_starter(=): range(1..)", "Expected a bounded"),
    ("f!() zip_shortest: range(1..)", "Expected at least one finite list"),
    ("f!() zip_longest: range(1..)", "Expected at least one finite list"),
    ("f!() iterate: range(1..)", "Expected a bounded"),
    ("f!() iterate: cycle((1)) ", "Expected a bounded"),
    ("f!() iterate: repeat(4, range(1..)) ", "Expected a bounded"),
    ("f!() iterate: skip(10, range(1..)) ", "Expected a bounded"),
    ("f!() iterate: chain(range(1..)) ", "Expected a bounded"),
    ("f!() iterate: chain((a b c d) range(1..)) ", "Expected a bounded"),
    ("f!() iterate: chain((a b c d) range(1..) range(1..)) ", "Expected a bounded"),
    ("f!() iterate: gen_ident_range(for i* in 0..) ", "Expected a bounded"),
];


// Ensures that all methods error when they only have infinite iterators
#[test]
fn unbounded_length_error_test() {
    test_try_proc(
        &mut UNBOUNDED_ERR_CASES.iter().map(|&(x, e)| (x, Err(e))),
        &|x| tokens_method(x).map_err(Error::into_compile_error)
    );
}
