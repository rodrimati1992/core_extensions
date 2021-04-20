#[allow(unused_imports)]
use crate::used_proc_macro::{
    token_stream::IntoIter,
    Delimiter, Ident, Group, Punct, Spacing, Span, TokenStream, TokenTree
};

use core::iter::{Peekable, once};


// Parse the arguments that were passed in parenthesized arguments
pub(crate) fn parse_paren_args(tt: &TokenTree) -> Peekable<IntoIter> {
    match tt {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
            let stream = group.stream();
            let mut iter = stream.clone().into_iter();
            match (iter.next(), iter.next()) {
                (Some(TokenTree::Group(group)), None) if group.delimiter() == Delimiter::None => {
                    group.stream()
                }
                _ => stream,
            }
        }
        x => panic!("Expected a parentheses-delimited group, found:\n{}", x),
    }.into_iter().peekable()
}

pub(crate) fn parenthesize_ts(ts: TokenStream, span: Span) -> TokenTree {
    let mut group = Group::new(Delimiter::Parenthesis, ts);
    group.set_span(span);
    TokenTree::Group(group)
}

pub(crate) fn out_parenthesized(ts: TokenStream, span: Span, out: &mut TokenStream) {
    out.extend(once(parenthesize_ts(ts, span)));
}

#[allow(dead_code)]
pub(crate) fn out_ident(value: &str, span: Span, out: &mut TokenStream) {
    let ident = Ident::new(value, span);
    out.extend(once(TokenTree::Ident(ident)));
}

#[allow(dead_code)]
pub(crate) fn out_colon2(span: Span, out: &mut TokenStream) {
    out.extend(once(TokenTree::Punct(Punct::new(':', Spacing::Joint))));
    out.extend(once(TokenTree::Punct(Punct::new(':', Spacing::Alone))));
}


pub(crate) fn parse_path_and_args<F>(
    macro_name: &str,
    iter: &mut IntoIter,
    mut args: TokenStream,
    f: F,
) -> TokenStream
where
    F: FnOnce(&mut TokenStream)
{
    let mut out = TokenStream::new();
    loop {
        match iter
            .next()
            .unwrap_or_else(|| panic!("{} expected more tokens", macro_name) )
        {
            TokenTree::Group(group) if group.delimiter() == Delimiter::None => {
                out.extend(group.stream());
            }
            TokenTree::Group(group) => {
                let mut pre = group.stream();
                pre.extend(args);
                args = pre;

                f(&mut args);

                let mut args = TokenTree::Group(Group::new(group.delimiter(), args));
                args.set_span(group.span());
                out.extend(once(args));

                return out;
            }
            x => {
                out.extend(once(x));
            }
        }
    }
}

