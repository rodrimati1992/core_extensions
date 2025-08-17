use syn::parse::{Parse, ParseBuffer, Peek};

use quote::TokenStreamExt;

pub struct Empty(pub proc_macro2::Span);

impl quote::ToTokens for Empty {
    fn to_tokens(&self, ts: &mut crate::TokenStream2) {
        ts.append_all(quote::quote_spanned!(self.0 => ()));
    }
}



pub(crate) trait ParseBufferExt {
    fn peek_parse<F, X, P>(&self, f: F) -> Result<Option<P>, syn::Error>
    where
        F: FnOnce(X) -> P + Peek,
        P: Parse;

    #[allow(dead_code)]
    fn peek_parse_paren(&self) -> Result<Option<syn::parse::ParseBuffer<'_>>, syn::Error>;
}

impl ParseBufferExt for ParseBuffer<'_> {
    fn peek_parse<F, X, P>(&self, f: F) -> Result<Option<P>, syn::Error>
    where
        F: FnOnce(X) -> P + Peek,
        P: Parse,
    {
        if self.peek(f) {
            self.parse::<P>().map(Some)
        } else {
            Ok(None)
        }
    }

    fn peek_parse_paren(&self) -> Result<Option<syn::parse::ParseBuffer<'_>>, syn::Error> {
        if self.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in self);
            Ok(Some(content))
        } else {
            Ok(None)
        }
    }
}


pub(crate) trait SynResultExt {
    fn combine_err<T>(&mut self, res: syn::Result<T>);
}

impl<T> SynResultExt for syn::Result<T>{
    fn combine_err<T2>(&mut self, res: syn::Result<T2>) {
        if let Err(err) = res {
            match self {
                this @ Ok(_) => *this = Err(err),
                Err(e) => e.combine(err),
            }
        }
    }
}