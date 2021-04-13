use crate::used_proc_macro::{Delimiter, Group, TokenStream, TokenTree};

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

