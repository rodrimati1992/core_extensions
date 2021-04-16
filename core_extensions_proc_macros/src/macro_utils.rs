use crate::{
    used_proc_macro::{Delimiter, Ident, Group, Literal, TokenStream, TokenTree},
    macro_utils_shared::{
        parse_ident, parse_keyword, parse_check_punct,
        parse_parentheses, parse_range_param, parse_macro_invocation,
    },
    mmatches,
};

use core::{
    iter::once,
    mem,
};

use alloc::{
    string::ToString,
    format,
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

enum ExpandedInto{
    Macro,
    Expr,
}

pub(crate) fn count_tts(tokens: TokenStream) -> crate::Result<TokenStream> {
    let mut iter = tokens.into_iter().peekable();

    fn output_counted(counted: Group, ei: ExpandedInto, out: &mut TokenStream) {
        let count = counted.stream().into_iter().count();
        let mut lit = match ei {
            ExpandedInto::Macro => Literal::usize_unsuffixed(count),
            ExpandedInto::Expr => Literal::usize_suffixed(count),
        };
        lit.set_span(counted.span());
        out.extend(once(TokenTree::Literal(lit)));
    }

    // If no callback macro was passed
    if mmatches!{
        iter.peek(), Some(TokenTree::Group(group))
        if mmatches!(group.delimiter(), Delimiter::Parenthesis)
    } {
        let mut out = TokenStream::new();

        output_counted(parse_parentheses(&mut iter)?, ExpandedInto::Expr, &mut out);

        Ok(out)
    } else {
        let mut macro_ = parse_macro_invocation(&mut iter)?;

        output_counted(parse_parentheses(&mut iter)?, ExpandedInto::Macro, &mut macro_.args);

        Ok(macro_.into_token_stream())
    }
}


pub(crate) fn gen_idents(tokens: TokenStream) -> crate::Result<TokenStream> {
    let mut iter = tokens.into_iter().peekable();
    
    let mut macro_ = parse_macro_invocation(&mut iter)?;

    parse_keyword(&mut iter, "for")?;

    let prefix = parse_ident(&mut iter)?;
    let sprefix = prefix.to_string();

    parse_check_punct(&mut iter, '*')?;

    parse_keyword(&mut iter, "in")?;

    let range = parse_range_param(&mut iter)?;

    let mut idents = TokenStream::new();

    for n in range {
        let ident = Ident::new(&format!("{}{}", sprefix, n), prefix.span());
        idents.extend(once(TokenTree::Ident(ident)))
    }

    let paren = Group::new(Delimiter::Parenthesis, idents);

    macro_.args.extend(once(TokenTree::Group(paren)));

    Ok(macro_.into_token_stream())
}

