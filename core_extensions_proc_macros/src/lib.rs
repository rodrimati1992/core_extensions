#![no_std]

extern crate proc_macro;

use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

use core::iter::once;


#[doc(hidden)]
#[proc_macro]
pub fn __priv_remove_non_delimiter(input_tokens: TokenStream) -> TokenStream {
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

    out
}
