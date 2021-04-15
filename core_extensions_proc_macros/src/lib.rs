#![no_std]

extern crate proc_macro;

#[cfg(not(test))]
use proc_macro as used_proc_macro;

#[cfg(test)]
extern crate proc_macro2;

#[cfg(test)]
use proc_macro2 as used_proc_macro;

extern crate alloc;

// #[cfg(test)]
extern crate std;

use crate::used_proc_macro::{Delimiter, Group, Punct, Spacing, Span, TokenStream, TokenTree};

use core::iter::once;

#[cfg(test)]
mod tests;

mod parsing_shared;

mod splitting_generics;

#[cfg(feature = "macro_utils")]
mod macro_utils;

#[cfg(feature = "macro_utils")]
mod macro_utils_shared;

#[cfg(feature = "item_parsing")]
mod item_parsing;


#[cfg(feature = "macro_utils")]
use crate::macro_utils_shared::Error;

#[cfg(feature = "macro_utils")]
type Result<T> = core::result::Result<T, Error>;


#[doc(hidden)]
#[proc_macro]
pub fn __priv_unwrap_bound(
    input_tokens: proc_macro::TokenStream
) -> proc_macro::TokenStream {
    let input_tokens: TokenStream = input_tokens.into();

    let mut iter = input_tokens.into_iter();

    let ty_tt = iter.next().expect("__priv_unwrap_bound expected more tokens");
    
    let group = match &ty_tt {
        TokenTree::Group(group) if group.delimiter() == Delimiter::None => group,
        x => panic!("Expected a none-delimited group, found:\n{}", x)
    };

    let mut last_is_plus = true;

    let mut ty = group.stream()
        .into_iter()
        .inspect(|tt|{
             last_is_plus = mmatches!(tt, TokenTree::Punct(punc) if punc.as_char() == '+');
        })
        .collect::<TokenStream>();

    if !last_is_plus {
        ty.extend(once(TokenTree::Punct(Punct::new('+', Spacing::Alone))))
    }

    let args = TokenStream::new();

    parsing_shared::parse_path_and_args("__priv_unwrap_bound", &mut iter, args, |args| {
        args.extend(once(TokenTree::Group(Group::new(Delimiter::Parenthesis, ty))));
    }).into()
}


#[cfg(feature = "macro_utils")]
#[doc(hidden)]
#[proc_macro]
pub fn __priv_rewrap_macro_parameters(input_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_tokens: TokenStream = input_tokens.into();
    //std::println!("\n----------------------------\n\n{:?}", input_tokens);
    let out = macro_utils::rewrap_macro_parameters(input_tokens);
    //std::println!("\n\n{:?}", out);
    out.into()
}

#[cfg(feature = "macro_utils")]
#[proc_macro]
pub fn count_tts(input_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_tokens: TokenStream = input_tokens.into();
    let out = macro_utils::count_tts(input_tokens).unwrap_or_else(Error::into_compile_error); 
    out.into()
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

    struct UnparsedPostGenerics {
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

    SplitGenerics::new(input).split_generics(TokenStream::new(), UnparsedPostGenerics{
        output: TokenStream::new(),
        output_span: Span::call_site(),
    })
}

#[cfg(feature = "item_parsing")]
#[doc(hidden)]
#[proc_macro]
pub fn __priv_split_impl(input_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::item_parsing::split_impl(input_tokens.into()).into()
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
