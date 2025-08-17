// For comparing token streams in a Comparable way

#[allow(unused_imports)]
use crate::{
    used_proc_macro::{Delimiter, Punct, TokenStream, TokenTree},
    mmatches,
};

use core::cmp::PartialEq;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};


pub(crate) enum Found {
    Yes,
    No,
}


pub(crate) enum ComparableTT {
    Ident(String),
    Punct(Punct),
    Literal(String),
    Group(ComparableGroup),
}



impl ComparableTT {
    pub(crate) fn new(tt: TokenTree) -> Self {
        match tt {
            TokenTree::Ident(ident) => ComparableTT::Ident(ident.to_string()),
            TokenTree::Punct(x) => ComparableTT::Punct(x),
            TokenTree::Literal(x) => ComparableTT::Literal(x.to_string()),
            TokenTree::Group(group) => {
                let this = ComparableGroup{
                    stream:  group.stream().into_iter().map(ComparableTT::new).collect(),
                    delimiter: group.delimiter(),
                };
                ComparableTT::Group(this)
            },
        }
    }

    pub(crate) fn many<I>(iter: I) -> Vec<ComparableTT>
    where
        I: IntoIterator<Item = TokenTree>
    {
        iter.into_iter().map(ComparableTT::new).collect()
    }
}

impl PartialEq<ComparableTT> for TokenTree {
    fn eq(&self, other: &ComparableTT) -> bool {
        match (self, other) {
            (TokenTree::Ident(l), ComparableTT::Ident(r)) => l.to_string() == *r,
            (TokenTree::Punct(l), ComparableTT::Punct(r)) => 
                l.as_char() == r.as_char(),
            (TokenTree::Literal(l), ComparableTT::Literal(r)) => l.to_string() == *r,
            (TokenTree::Group(l), ComparableTT::Group(r)) => {
                l.stream().into_iter().eq(r.stream.iter()) &&
                l.delimiter() == r.delimiter
            }
            _ => false,
        }
    }
}

impl PartialEq<&ComparableTT> for TokenTree {
    fn eq(&self, other: &&ComparableTT) -> bool {
        *self == **other
    }
}


pub(crate) struct ComparableGroup {
    pub(crate) stream: Vec<ComparableTT>,
    pub(crate) delimiter: Delimiter,
}


pub(crate) fn skip_until_match<I>(mut iter: I, s_tokens: &[ComparableTT]) -> (TokenStream, Found) 
where 
    I: Iterator<Item = TokenTree>
{
    let mut cmp_iter = s_tokens.iter();
    
    let mut out = Vec::new();

    while let Some(next) = cmp_iter.next() {
        match iter.next() {
            Some(tt) => {
                if tt != *next {
                    cmp_iter = s_tokens.iter();
                    if tt == s_tokens[0] {
                        cmp_iter.next();
                    }
                }
                out.push(tt);
            }
            None => return (out.into_iter().collect(), Found::No),
        }
    }

    let found = if s_tokens.is_empty() { Found::No } else { Found::Yes };

    out.truncate(out.len() - s_tokens.len());
    (out.into_iter().collect(), found)
}










