use crate::{
    used_proc_macro::{
        token_stream::IntoIter,
        Delimiter, Group, Spacing, Span, TokenStream, TokenTree
    },
    parsing_shared::{out_ident, out_parenthesized, parse_paren_args, parse_path_and_args},
    splitting_generics::{PostGenericsParser, SplitGenerics},
    mmatches,
};

use core::{
    iter::once,
    mem,
};

use alloc::string::ToString;


struct ImplHeader {
    type_: TokenStream,
    type_span: Span,
    trait_: Option<TokenStream>,
    trait_span: Span,
    location: ParseLocation,
}

impl PostGenericsParser for ImplHeader {
    fn consume_token(&mut self, sg: &SplitGenerics, tt: TokenTree) {
        match self.location {
            ParseLocation::BeforeStart => {
                self.location = if mmatches!(&tt, TokenTree::Ident(i) if i.to_string() == "dyn" ) {
                    ParseLocation::IgnoreFor
                } else {
                    ParseLocation::Started
                };
            },
            ParseLocation::Started => {
                if mmatches!(&tt, TokenTree::Ident(i) if i.to_string() == "for" ) {
                    self.trait_span = self.type_span;
                    self.trait_ = Some(mem::take(&mut self.type_));
                    self.location = ParseLocation::IgnoreFor;
                    return;
                }
            }
            ParseLocation::IgnoreFor =>  {}
        }

        self.type_span = sg.last_span();
        self.type_.extend(once(tt));
    }
    fn write_tokens(self, ts: &mut TokenStream) {
        if let Some(trait_) = self.trait_ {
            out_ident("trait", self.trait_span, ts);
            out_parenthesized(trait_, self.trait_span, ts);
        }
        out_ident("type", self.type_span, ts);
        out_parenthesized(self.type_, self.type_span, ts);
    }
}

pub(crate) fn split_impl(ts: TokenStream) -> TokenStream {
    let mut ts = ts.into_iter();

    let parsed_tt = ts.next().expect("skip_generics expected more tokens");

    let mut parsing = parse_paren_args(&parsed_tt);

    let mut out = TokenStream::new();

    let mut attrs = TokenStream::new();
    let mut attrs_span = Span::call_site();
    let mut qualifiers = TokenStream::new();
    let mut qualifiers_span = Span::call_site();
    let mut which_one = &mut attrs;
    let mut which_span = &mut attrs_span;

    while let Some(tt) = parsing.peek() {
        if let TokenTree::Ident(ident) = tt {
            if ident.to_string() == "impl" {
                parsing.next();
                break
            } else {
                which_one = &mut qualifiers;
                which_span = &mut qualifiers_span;
            }
        }

        *which_span = tt.span();
        which_one.extend(parsing.next());
    }

    out_parenthesized(attrs, attrs_span, &mut out);
    out_parenthesized(qualifiers, qualifiers_span, &mut out);

    SplitGenerics::some_consumed(ts, parsing).split_generics(out, ImplHeader{
        type_: TokenStream::new(),
        type_span: Span::call_site(),
        trait_: None,
        trait_span: Span::call_site(),
        location: ParseLocation::BeforeStart,
    })
}

enum ParseLocation {
    BeforeStart,
    Started,
    IgnoreFor,
}



