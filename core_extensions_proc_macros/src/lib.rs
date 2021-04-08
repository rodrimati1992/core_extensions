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

use core::{
    iter::once,
    mem,
};

use alloc::string::ToString;

#[cfg(test)]
mod tests;

mod parsing_shared;

mod splitting_generics;

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

    parsing_shared::parse_path_and_args("__priv_remove_non_delimiter", &mut iter, |args| {
        args.extend(once(TokenTree::Group(Group::new(Delimiter::Parenthesis, ty))));
    }).into()
}


#[doc(hidden)]
#[proc_macro]
pub fn __priv_split_generics(input_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    split_generics(input_tokens.into()).into()
}


fn split_generics(input: TokenStream) -> TokenStream {
    use crate::{
        parsing_shared::out_parenthesized,
        splitting_generics::{PostGenericsParser, SplitGenerics}
    };

    pub(crate) struct UnparsedPostGenerics {
        output: TokenStream,
        output_span: Span,
    }

    impl PostGenericsParser for UnparsedPostGenerics {
        fn consume_token(&mut self, sg: &SplitGenerics, tt: TokenTree) {
            self.output_span = sg.last_span();
            self.output.extend(once(tt));
        }
        fn write_tokens(self, ts: &mut TokenStream) {
            out_parenthesized(self.output, self.output_span, ts)
        }
    }

    SplitGenerics::new(input).split_generics(UnparsedPostGenerics{
        output: TokenStream::new(),
        output_span: Span::call_site(),
    })
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
