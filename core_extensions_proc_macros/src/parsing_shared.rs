#[allow(unused_imports)]
use crate::used_proc_macro::{
    token_stream::IntoIter,
    Delimiter, Ident, Group, Punct, Spacing, Span, TokenStream, TokenTree
};

use core::iter::{Peekable, once};


// Parse the arguments that were passed in parenthesized arguments
#[track_caller]
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
    out_punct(':', Spacing::Joint, span, out);
    out_punct(':', Spacing::Alone, span, out);
}

#[allow(dead_code)]
pub(crate) fn out_punct(char: char, spacing: Spacing, span: Span, out: &mut TokenStream) {
    let mut token = TokenTree::Punct(Punct::new(char, spacing));
    token.set_span(span);
    out.extend(once(token));
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





////////////////////////////////////////////////////////////////////////////////


pub(crate) struct MacroInvocation {
    pub(crate) path_bang: TokenStream,
    pub(crate) delimiter: Delimiter,
    pub(crate) delim_span: Span,
    pub(crate) args: TokenStream,
}

impl MacroInvocation {
    pub(crate) fn into_token_stream(mut self) -> TokenStream {
        let mut args = Group::new(self.delimiter, self.args);
        args.set_span(self.delim_span);
        self.path_bang.extend(once(TokenTree::Group(args)));
        self.path_bang
    }

    pub(crate) fn expand_with_extra_args<F>(mut self, f: F) -> TokenStream 
    where
        F: FnOnce(&mut TokenStream)
    {
        f(&mut self.args);
        let mut args = Group::new(self.delimiter, self.args);
        args.set_span(self.delim_span);
        self.path_bang.extend(once(TokenTree::Group(args)));
        self.path_bang
    }
}

const PARSE_MACRO_CALL_ERR: &str = "could not parse last tokens as a macro invocation";

#[cfg(feature = "macro_utils")]
pub(crate) fn parse_macro_invocation<I>(
    iter: I
) -> crate::Result<MacroInvocation> 
where
    I: IntoIterator<Item = TokenTree>
{
    let mut path_bang = TokenStream::new();

    let mut iter = iter.into_iter();

    loop {
        match iter.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::None => {
                path_bang.extend(group.stream());
            }
            Some(TokenTree::Group(group)) => {
                return Ok(MacroInvocation{
                    path_bang,
                    delimiter: group.delimiter(),
                    delim_span: group.span(),
                    args: group.stream(),
                });
            }
            Some(x) => {
                path_bang.extend(once(x));
            }
            None => {
                return Err(crate::Error::end(PARSE_MACRO_CALL_ERR));
            }
        }
    }
}

pub(crate) fn panicking_parse_macro_invocation<I>(
    iter: I
) -> MacroInvocation
where
    I: IntoIterator<Item = TokenTree>
{
    let mut path_bang = TokenStream::new();

    let mut iter = iter.into_iter();

    loop {
        match iter.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::None => {
                path_bang.extend(group.stream());
            }
            Some(TokenTree::Group(group)) => {
                return MacroInvocation{
                    path_bang,
                    delimiter: group.delimiter(),
                    delim_span: group.span(),
                    args: group.stream(),
                };
            }
            Some(x) => {
                path_bang.extend(once(x));
            }
            None => panic!("{}", PARSE_MACRO_CALL_ERR),
        }
    }
}

