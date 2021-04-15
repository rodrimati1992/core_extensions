use crate::{
    used_proc_macro::{Delimiter, Group, Literal, TokenStream, TokenTree},
    macro_utils_shared::{parse_parentheses, parse_macro_invocation},
    mmatches,
};

use core::{
    iter::once,
    mem,
};

pub fn rewrap_macro_parameters(tokens: TokenStream) -> TokenStream {
    let mut prev_tilde;
    let mut curr_tilde = false;
    let mut out = TokenStream::new();

    for tt in tokens {
        prev_tilde = mem::replace(&mut curr_tilde, false);

        let tt_out = match tt {
            TokenTree::Group(group) => {
                let out = rewrap_macro_parameters(group.stream());
                let span = group.span();

                let delim = if prev_tilde && group.delimiter() == Delimiter::None {
                    Delimiter::Parenthesis
                } else {
                    group.delimiter()
                };

                let mut group = Group::new(delim, out);
                group.set_span(span);
                TokenTree::Group(group)
            }
            TokenTree::Punct(punct) => {
                curr_tilde = punct.as_char() == '~';
                if !prev_tilde && curr_tilde {
                    continue;
                } else {
                    curr_tilde = false;
                    TokenTree::Punct(punct)
                }
            },
            tt => tt,
        };

        out.extend(once(tt_out));
    }
    out
}


pub(crate) fn count_tts(tokens: TokenStream) -> crate::Result<TokenStream> {
    let mut iter = tokens.into_iter().peekable();

    fn output_counted(counted: Group, out: &mut TokenStream) {
        let mut lit = Literal::u32_unsuffixed(counted.stream().into_iter().count() as u32);
        lit.set_span(counted.span());
        out.extend(once(TokenTree::Literal(lit)));
    }

    // If no callback macro was passed
    if mmatches!{
        iter.peek(), Some(TokenTree::Group(group))
        if mmatches!(group.delimiter(), Delimiter::Parenthesis)
    } {
        let mut out = TokenStream::new();

        output_counted(parse_parentheses(&mut iter)?, &mut out);

        Ok(out)
    } else {
        let mut macro_ = parse_macro_invocation(&mut iter)?;

        output_counted(parse_parentheses(&mut iter)?, &mut macro_.args);

        Ok(macro_.into_token_stream())
    }
}


