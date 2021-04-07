#![no_std]

extern crate proc_macro;

#[cfg(not(test))]
use proc_macro as used_proc_macro;

#[cfg(test)]
extern crate proc_macro2;

#[cfg(test)]
use proc_macro2 as used_proc_macro;

extern crate alloc;

#[cfg(test)]
extern crate std;

use crate::used_proc_macro::{Delimiter, Group, Spacing, Span, TokenStream, TokenTree};

use crate::used_proc_macro::token_stream::IntoIter;

use core::iter::once;

use alloc::string::ToString;

#[cfg(test)]
mod tests;


#[doc(hidden)]
#[proc_macro]
pub fn __priv_remove_non_delimiter(
    input_tokens: proc_macro::TokenStream
) -> proc_macro::TokenStream {
    let input_tokens: TokenStream = input_tokens.into();

    let mut iter = input_tokens.into_iter();

    let ty_tt = iter.next().expect("__priv_remove_non_delimiter expected more tokens");
    
    let ty = match &ty_tt {
        TokenTree::Group(group) if group.delimiter() == Delimiter::None => 
            group.stream(),
        x => panic!("Expected a none-delimited group, found:\n{}", x)
    };

    parse_path_and_args("__priv_remove_non_delimiter", &mut iter, |args| {
        args.extend(once(TokenTree::Group(Group::new(Delimiter::Parenthesis, ty))));
    }).into()
}


#[doc(hidden)]
#[proc_macro]
pub fn __priv_split_generics(input_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    split_generics(input_tokens.into()).into()
}

#[doc(hidden)]
fn split_generics(input_tokens: TokenStream) -> TokenStream {
    let mut iter = input_tokens.into_iter();

    let ty_tt = iter.next().expect("skip_generics expected more tokens");

    let input = match &ty_tt {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => group.stream(),
        x => panic!("Expected a parentheses-delimited group, found:\n{}", x),
    };

    let mut curr_joint = false;
    let mut prev_joint;
    let mut depth = 0;
    let mut last_span = Span::call_site();

    let mut generics = TokenStream::new();
    let mut generics_span = Span::call_site();

    let mut after_generics = TokenStream::new();
    let mut after_generics_span = Span::call_site();

    let mut where_clause = TokenStream::new();
    let mut where_clause_span = Span::call_site();
    
    let mut after_where = TokenStream::new();
    let after_where_span;

    let mut input = input.into_iter().peekable();

    macro_rules! match_tt {
        ($tt:ident, $($e:expr)? , $on_too_many_gt:expr ) => {
            last_span = $tt.span();
            prev_joint = curr_joint;
            curr_joint = false;
            if let TokenTree::Punct(punct) = &$tt {
                let char = punct.as_char();
                curr_joint = char == '-' ||
                    punct.spacing() == Spacing::Joint && char != '>' && char != '<';

                if char == '<' {
                    depth += 1;
                } if !prev_joint && char == '>' {
                    if depth == 0 {
                        $on_too_many_gt;
                    } else {
                        depth -= 1;
                    }
                }
            }

            $(
                if depth == 0 {
                    $e
                }
            )?

        };
    }

    if mmatches!(input.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == '<' ) {
        drop(input.next());
        while let Some(tt) = input.next() {
            match_tt!{ tt, , break }

            generics.extend(once(tt));
        }
        generics_span = last_span;
    }
    

    if depth == 0 {
        let mut output = &mut after_generics;

        while let Some(tt) = input.next() {
            match_tt!{
                tt, match &tt {
                    TokenTree::Ident(ident) if ident.to_string() == "where" => {
                        output = &mut where_clause;
                        after_generics_span = last_span;
                        continue;
                    }
                    TokenTree::Punct(punct) if {
                        let c = punct.as_char();
                        c == ';' || c == '=' && punct.spacing() == Spacing::Alone
                    } => {
                        after_where.extend(once(tt));
                        break
                    }
                    TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                        after_where.extend(once(tt));
                        break
                    }
                    _ => {}
                },
                {}
            }
            output.extend(once(tt));
        }

        where_clause_span = last_span;
    }

    for tt in input {
        last_span = tt.span();
        after_where.extend(once(tt));
    }
    after_where_span = last_span;


    parse_path_and_args("__priv_split_generics", &mut iter, |args| {
        args.extend(once(parenthesize_ts(generics, generics_span)));
        args.extend(once(parenthesize_ts(after_generics, after_generics_span)));
        args.extend(once(parenthesize_ts(where_clause, where_clause_span)));
        args.extend(once(parenthesize_ts(after_where, after_where_span)));
    })
}

fn parenthesize_ts(ts: TokenStream, span: Span) -> TokenTree {
    let mut group = Group::new(Delimiter::Parenthesis, ts);
    group.set_span(span);
    TokenTree::Group(group)
}



fn parse_path_and_args<F>(
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



// MSRV is 1.41.0, matches was stabilized in 1.42.0
macro_rules! mmatches {
    ( $expr:expr, $pat:pat $(if $cond:expr)?)=>{
        match $expr {
            $pat  $(if $cond)? =>true,
            _=>false
        }
    };
} use mmatches;
