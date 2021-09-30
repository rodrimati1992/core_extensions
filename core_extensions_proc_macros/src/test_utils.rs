use crate::used_proc_macro::TokenStream;

use alloc::string::{String, ToString};

use alloc::vec::Vec;

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



pub trait TestStrExt {
    fn as_str(&self) -> &str;

    /// Checks that these needles exist consequtively in self.
    ///
    /// Example: `"hello world".consecutive_in_set(&["he", "wor"])` returns `true`.
    /// Example: `"hello world".consecutive_in_set(&["wor", "he"])` returns `false`.
    fn consecutive_in_self<S: AsRef<str>>(&self, needles: &[S]) -> bool {
        let mut rem = self.as_str();
        for needle in needles {
            let needle: &str = needle.as_ref();
            rem = match rem.find(needle) {
                Some(next) => &rem[next + needle.len()..],
                None => return false,
            };
        }
        true
    }

    fn consecutive_unspace(&self, needles: &[&str]) -> bool {
        let rem = remove_whitespaces(self.as_str());
        let needles = needles.iter().map(|x| remove_whitespaces(x   )).collect::<Vec<String>>();
        rem.consecutive_in_self(&needles)
    }
}

impl TestStrExt for str {
    #[inline(always)]
    fn as_str(&self) -> &str {
        self
    }
}

impl TestStrExt for alloc::string::String {
    #[inline(always)]
    fn as_str(&self) -> &str {
        self
    }
}
