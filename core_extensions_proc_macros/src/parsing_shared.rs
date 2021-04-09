use crate::used_proc_macro::{
    token_stream::IntoIter,
    Delimiter, Ident, Group, Spacing, Span, TokenStream, TokenTree
};


use core::iter::once;

use alloc::string::ToString;


pub(crate) fn parenthesize_ts(ts: TokenStream, span: Span) -> TokenTree {
    let mut group = Group::new(Delimiter::Parenthesis, ts);
    group.set_span(span);
    TokenTree::Group(group)
}

pub(crate) fn out_parenthesized(ts: TokenStream, span: Span, out: &mut TokenStream) {
    out.extend(once(parenthesize_ts(ts, span)));
}

pub(crate) fn out_ident(value: &str, span: Span, out: &mut TokenStream) {
    let ident = Ident::new(value, span);
    out.extend(once(TokenTree::Ident(ident)));
}


pub(crate) fn parse_path_and_args<F>(
    macro_name: &str,
    iter: &mut IntoIter,
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
                let mut args = group.stream();

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
