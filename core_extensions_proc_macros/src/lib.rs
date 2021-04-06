#![no_std]

#[cfg(not(test))]
extern crate proc_macro as used_proc_macro;

#[cfg(test)]
extern crate proc_macro2 as used_proc_macro;

extern crate alloc;

#[cfg(test)]
extern crate std;

use used_proc_macro::{Delimiter, Group, Spacing, TokenStream, TokenTree};

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


    let mut out = TokenStream::new();

    loop{
        match iter.next().expect("__priv_remove_non_delimiter expected more tokens") {
            TokenTree::Group(group) => {
                assert_ne!(group.delimiter(), Delimiter::None);

                let mut args = group.stream();

                args.extend(once(TokenTree::Group(Group::new(Delimiter::Parenthesis, ty))));

                out.extend(once(TokenTree::Group(Group::new(group.delimiter(), args))));

                break;
            }
            x => {
                out.extend(once(x));
            }
        }
    }

    out.into()
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

    let mut generics = TokenStream::new();
    let mut after_generics = TokenStream::new();
    let mut where_clause = TokenStream::new();
    let mut after_where = TokenStream::new();

    let mut input = input.into_iter();

    macro_rules! match_tt {
        ($tt:ident, $($e:expr)? , $on_too_many_gt:expr ) => {
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

    while let Some(tt) = input.next() {
        match_tt!{ tt, , break }

        generics.extend(once(tt));
    }

    if depth == 0 {
        let mut output = &mut after_generics;

        while let Some(tt) = input.next() {
            match_tt!{
                tt, match &tt {
                    TokenTree::Ident(ident) if ident.to_string() == "where" => {
                        output = &mut where_clause;
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
    }

    after_where.extend(input);

    let mut out = TokenStream::new();

    loop {
        match iter
            .next()
            .expect("__priv_remove_non_delimiter expected more tokens")
        {
            TokenTree::Group(group) => {
                assert_ne!(group.delimiter(), Delimiter::None);

                let mut args = group.stream();

                args.extend(once(parenthesize_token_stream(generics)));
                args.extend(once(parenthesize_token_stream(after_generics)));
                args.extend(once(parenthesize_token_stream(where_clause)));
                args.extend(once(parenthesize_token_stream(after_where)));

                out.extend(once(TokenTree::Group(Group::new(group.delimiter(), args))));

                break;
            }
            x => {
                out.extend(once(x));
            }
        }
    }

    // panic!("{}", out)
    out
}

fn parenthesize_token_stream(ts: TokenStream) -> TokenTree {
    TokenTree::Group(Group::new(Delimiter::Parenthesis, ts))
}
