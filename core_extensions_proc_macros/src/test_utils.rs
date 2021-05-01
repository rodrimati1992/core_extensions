use crate::used_proc_macro::TokenStream;

use alloc::string::{String, ToString};

pub(crate) fn remove_whitespaces(x: &str) -> String {
    x.chars()
        .filter(|x| !x.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}


pub(crate) fn test_proc(
    results: &[(&str, &str)],
    function: &dyn Fn(TokenStream) -> TokenStream,
) {
    for (input, expected) in results {
        let found = remove_whitespaces(&function(input.parse().unwrap()).to_string());
        let expected = remove_whitespaces(expected);

        assert_eq!(found, expected, "\ninput   : {}\nexpected: {}", input, expected);
    }
}

pub(crate) fn test_try_proc(
    results: &mut dyn Iterator<Item = (&str, Result<&str, &str>)>,
    function: &dyn Fn(TokenStream) -> Result<TokenStream, TokenStream>,
) {
    for (input, expected) in results {
        let ret = function(input.parse().unwrap());
        let found = map_both(ret, |x| remove_whitespaces(&x.to_string()));
        let expected = expected.map(remove_whitespaces).map_err(remove_whitespaces);

        let cond = match &expected {
            Ok(x) => found.as_ref().unwrap() == x,
            Err(x) => found.as_ref().unwrap_err().contains(&*x),
        };

        assert!(
            cond,
            "\ninput   : {}\nexpected: {}\nfound: {}",
            input,
            format_res(&expected),
            format_res(&found),
        );
    }
}

fn format_res(res: &Result<String, String>) -> String {
    match res {
        Ok(x) => alloc::format!("Ok : {}", x),
        Err(x) => alloc::format!("Err: {}", x),
    }
}

fn map_both<T, U, F>(res: Result<T, T>, func: F) -> Result<U, U>
where
    F: Fn(T) -> U
{
    match res {
        Ok(x) => Ok(func(x)),
        Err(x) => Err(func(x)),
    }
}

