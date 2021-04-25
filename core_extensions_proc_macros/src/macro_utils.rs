use crate::{
    used_proc_macro::{
        token_stream::IntoIter,
        Delimiter, Ident, Group, Literal, Punct, Spacing, TokenStream, TokenTree,
    },
    macro_utils_shared::{
        cmp_ts::{self, ComparableTT, Found},
        parse_count_param, parse_ident, parse_int_or_range_param,
        parse_keyword, parse_check_punct,
        parse_parentheses, parse_range_param, parse_macro_invocation,
        macro_span, out_parenthesized_tt,
    },
    parsing_shared::out_parenthesized,
    mmatches,
};

use core::{
    iter::once,
    mem,
};

use alloc::{
    string::ToString,
    vec::Vec,
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


pub(crate) fn gen_ident_range(tokens: TokenStream) -> crate::Result<TokenStream> {
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


pub(crate) fn macro_attr(attr: TokenStream, item: TokenStream) -> crate::Result<TokenStream> {
    let mut attr = attr.into_iter();

    let mut macro_ = crate::macro_utils_shared::parse_path_and_span(&mut attr)?;

    let (bang, more_tokens) = match macro_.terminator {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '!' => 
            (punct, true),
        Some(tt) => 
            return Err(crate::Error::one_tt(tt.span(), "expected a `!`")),
        None => {
            let mut bang = Punct::new('!', Spacing::Alone);
            bang.set_span(macro_.start_span);
            (bang, false)
        }
    };
    
    macro_.path.extend(once(TokenTree::Punct(bang)));

    let (args, bspan) = if more_tokens {
        let group = crate::macro_utils_shared::parse_group(&mut attr)?;
        let mut args = group.stream();
        args.extend(item);
        
        (args, group.span())
    } else {
        (item, macro_.end_span)
    };

    let mut args = Group::new(Delimiter::Brace, args);
    args.set_span(bspan);
    macro_.path.extend(once(TokenTree::Group(args)));
    
    Ok(macro_.path)
}


pub(crate) fn tokens_method(tokens: TokenStream) -> crate::Result<TokenStream> {
    let mut iter = tokens.into_iter();

    let mut macro_ = parse_macro_invocation(&mut iter)?;
    let args = &mut macro_.args;

    macro_rules! declare_methods {
        (
            $fname:literal => $fblock:block
            $( $name:literal => $block:block )* 
        ) => {
            const ERR_MSG: &str = concat!(
                "expected one of ",
                "`", $fname, "`",
                $(", `", $name, "`",)*
                "."
            );

            match parse_ident(&mut iter) {
                Ok(ident) => {
                    let keyword = ident.to_string();

                    match &keyword[..] {
                        $fname => $fblock
                        $($name => $block)*
                        other => {
                            let err = format!("{}\nFound {}", ERR_MSG, other);
                            return Err(crate::Error::one_tt(ident.span(), &err));
                        }
                    }
                }
                Err(e) => {
                    return Err(crate::Error::one_tt(e.start_span(), &ERR_MSG));
                }
            }
        };
    }

    declare_methods!{
        "first" => {
            let group = parse_parentheses(iter)?;
            
            let last_token: TokenStream = group.stream().into_iter().take(1).collect();

            out_parenthesized(last_token, group.span(), args);
        }
        "last" => {
            let group = parse_parentheses(iter)?;
            
            let last_token: TokenStream = 
                group.stream().into_iter().last().into_iter().collect();

            out_parenthesized(last_token, group.span(), args);
        }
        "split_first" => {
            let group = parse_parentheses(iter)?;
            
            let mut iter = group.stream().into_iter();
            let first: TokenStream = (&mut iter).take(1).collect();
            let rest: TokenStream = iter.collect();

            out_parenthesized(first, group.span(), args);
            out_parenthesized(rest, group.span(), args);
        }
        "split_last" => {
            let group = parse_parentheses(iter)?;
            
            let mut iter = group.stream().into_iter();
            
            let mut first = TokenStream::new();
            let mut last = iter.next();
            for tt in iter {
                first.extend(last);
                last = Some(tt);
            }
            let last = last.into_iter().collect::<TokenStream>();

            out_parenthesized(first, group.span(), args);
            out_parenthesized(last, group.span(), args);
        }
        "split_last_n" => {
            let mut params = parse_parentheses(&mut iter)?.stream().into_iter();
            let last_count = parse_count_param(&mut params)? as usize;
            crate::macro_utils_shared::expect_no_tokens(params)?;

            let group = parse_parentheses(iter)?;
            
            let elems = group.stream().into_iter().collect::<Vec<TokenTree>>();
            
            let taken = elems.len().saturating_sub(last_count);
            let mut iter = elems.into_iter();
            let first = (&mut iter).take(taken).collect::<TokenStream>();
            let last = iter.collect::<TokenStream>();

            out_parenthesized(first, group.span(), args);
            out_parenthesized(last, group.span(), args);
        }
        "split_at" => {
            let mut params = parse_parentheses(&mut iter)?.stream().into_iter();
            let split_at = parse_count_param(&mut params)? as usize;
            crate::macro_utils_shared::expect_no_tokens(params)?;

            let group = parse_parentheses(&mut iter)?;
            
            let mut iter = group.stream().into_iter();

            let start: TokenStream = (&mut iter).take(split_at).collect();
            let rest: TokenStream = iter.collect();

            out_parenthesized(start, group.span(), args);
            out_parenthesized(rest, group.span(), args);
        }
        "get" => {
            let params = parse_parentheses(&mut iter)?;
            let mut params = params.stream().into_iter().peekable();
            let range = parse_int_or_range_param(&mut params)?;
            crate::macro_utils_shared::expect_no_tokens(params)?;

            let group = parse_parentheses(&mut iter)?;

            let middle: TokenStream = group.stream()
                .into_iter()
                .take(range.end as usize)
                .skip(range.start as usize)
                .collect();

            out_parenthesized(middle, group.span(), args);
        }
        "split" => {
            let (needle, group, mut iter) = split_shared(&mut iter)?;
            loop {
                let (tokens, found) = cmp_ts::skip_until_match(&mut iter, &needle);
                out_parenthesized(tokens, group.span(), args);
                if let Found::No = found { break }
            }
        }
        "split_terminator" => {
            let (needle, group, mut iter) = split_shared(&mut iter)?;
            loop {
                let (tokens, found) = cmp_ts::skip_until_match(&mut iter, &needle);
                if mmatches!(found, Found::Yes) || !tokens.is_empty() {
                    out_parenthesized(tokens, group.span(), args);
                }
                if let Found::No = found { break }
            }
        }
        "split_starter" => {
            let (needle, group, mut iter) = split_shared(&mut iter)?;

            let mut start = true;
            loop {
                let (tokens, found) = cmp_ts::skip_until_match(&mut iter, &needle);
                if !start || ( start && (!tokens.is_empty() || mmatches!(found, Found::No))) {
                    out_parenthesized(tokens, group.span(), args);
                }
                if let Found::No = found { break }
                start = false;
            }
        }
        "zip_shortest" => {
            let mut iters = iter_many_parentheses(iter)?;
            let outer_span = macro_span();

            'outer: loop {
                let mut zipped = TokenStream::new();
                for tt_iter in &mut iters {
                    if let Some(tt) = tt_iter.next() {
                        out_parenthesized_tt(tt, &mut zipped);
                    } else {
                        break 'outer;
                    }
                }
                out_parenthesized(zipped, outer_span, args)
            }
        }
        "zip_longest" => {
            let mut iters = iter_many_parentheses(iter)?;
            let outer_span = macro_span();

            loop {
                let mut zipped = TokenStream::new();

                let mut none_count = 0;
                for tt_iter in &mut iters {
                    if let Some(tt) = tt_iter.next() {
                        out_parenthesized_tt(tt, &mut zipped);
                    } else {
                        none_count+=1;
                        out_parenthesized(TokenStream::new(), outer_span, &mut zipped)
                    }
                }
                if none_count == iters.len() { break }

                out_parenthesized(zipped, outer_span, args)
            }
        }
    }

    Ok(macro_.into_token_stream())
}


fn split_shared(iter: &mut IntoIter) -> crate::Result<(Vec<ComparableTT>, Group, IntoIter)> {
    let params = parse_parentheses(&mut *iter)?;
    let needle = ComparableTT::many(params.stream().into_iter());

    let group = parse_parentheses(&mut *iter)?;
    let iter = group.stream().into_iter();
    
    Ok((needle, group, iter))
}

fn iter_many_parentheses(iter: IntoIter) -> crate::Result<Vec<IntoIter>> {
    let mut out = Vec::new();
    let mut iter = iter.peekable();
    
    while iter.peek().is_some() {
        let group = parse_parentheses(&mut iter)?;
        out.push(group.stream().into_iter());
    }

    Ok(out)
}





