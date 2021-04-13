use crate::{
    used_proc_macro::{
        token_stream::IntoIter,
        Delimiter, Punct, Spacing, Span, TokenStream, TokenTree
    },
    parsing_shared::{out_parenthesized, parse_paren_args, parse_path_and_args},
    mmatches,
};

use core::iter::{Peekable, once};

use alloc::string::ToString;




pub(crate) trait PostGenericsParser {
    fn consume_token(&mut self, _: &SplitGenerics, tt: TokenTree);
    fn write_tokens(self, ts: &mut TokenStream);
}



pub(crate) struct SplitGenerics {
    // All of the tokens passed to this
    input_tokens: IntoIter,
    // The parsed tokens from the generic parameter list to after the where clause
    parsing: Peekable<IntoIter>,
    curr_is_joint: bool,
    prev_is_joint: bool,
    curr_token_kind: TokenKind,
    prev_token_kind: TokenKind,
    location: ParseLocation,
    depth: u32,
    last_span: Span,
    generics: TokenStream,
    generics_span: Span,
    where_clause: TokenStream,
    where_clause_span: Span,
    after_where: TokenStream,
    after_where_span: Span,
}


impl SplitGenerics {
    pub(crate) fn new(input_tokens: TokenStream) -> Self {
        let mut input_tokens = input_tokens.into_iter();

        let parsed_tt = input_tokens.next().expect("skip_generics expected more tokens");

        let parsing = parse_paren_args(&parsed_tt);

        Self::some_consumed(input_tokens, parsing)
    }

    pub(crate) fn some_consumed(input_tokens: IntoIter, parsing: Peekable<IntoIter>) -> Self {
        Self {
            input_tokens,
            parsing,
            curr_is_joint: false,
            prev_is_joint: false,
            curr_token_kind: TokenKind::Other,
            prev_token_kind: TokenKind::Other,
            depth: 0,
            location: ParseLocation::InGenerics,
            last_span: Span::call_site(),
            generics: TokenStream::new(),
            generics_span: Span::call_site(),
            where_clause: TokenStream::new(),
            where_clause_span: Span::call_site(),
            after_where: TokenStream::new(),
            after_where_span: Span::call_site(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn curr_is_joint(&self) -> bool {
        self.curr_is_joint
    }

    #[allow(dead_code)]
    pub(crate) fn prev_is_joint(&self) -> bool {
        self.prev_is_joint
    }

    #[allow(dead_code)]
    pub(crate) fn depth(&self) -> u32 {
        self.depth
    }

    #[allow(dead_code)]
    pub(crate) fn last_span(&self) -> Span {
        self.last_span
    }
}

macro_rules! match_process_gen {
    ($res:expr, $tt:ident) => {
        match $res {
            Some(tt)=> $tt = tt,
            None => break,
        }
    };
}

impl SplitGenerics {
    pub(crate) fn split_generics<P>(mut self, args: TokenStream,mut parsing_pgen: P) -> TokenStream
    where
        P: PostGenericsParser
    {
        self.process_generics();

        self.location = ParseLocation::AfterGenerics;

        if self.depth == 0 {
            while let Some(mut tt) = self.parsing.next() {
                match_process_gen!(self.process_generic_list(tt), tt);

                if self.depth == 0 {
                    match_process_gen!(self.process_after_generics(tt), tt);
                }

                parsing_pgen.consume_token(&self, tt);
            }
        }

        self.process_from_where_clause();

        let Self{
            mut input_tokens, 
            generics, generics_span,
            where_clause, where_clause_span,
            after_where, after_where_span,
            ..
        } = self;

        parse_path_and_args("__priv_split_generics", &mut input_tokens, args, |args| {

            out_parenthesized(generics, generics_span, args);
            
            parsing_pgen.write_tokens(args);

            out_parenthesized(where_clause, where_clause_span, args);
            out_parenthesized(after_where, after_where_span, args);
        })
    }

    // Processes the generic parameters that start the token stream,
    // those declare the generic parmeters
    fn process_generics(&mut self) {
        if mmatches!(
            self.parsing.peek(),
            Some(TokenTree::Punct(punct)) if punct.as_char() == '<' 
        ) {
            drop(self.parsing.next());
            while let Some(mut tt) = self.parsing.next() {
                match_process_gen!(self.process_generic_list(tt), tt);
                self.generics.extend(once(tt));
            }
            self.generics_span = self.last_span;
        }
    }

    fn process_from_where_clause(&mut self) {
        if self.depth == 0 && mmatches!(self.location, ParseLocation::InWhere) {
            while let Some(mut tt) = self.parsing.next() {
                match_process_gen!(self.process_generic_list(tt), tt);

                if self.depth == 0 {
                    match_process_gen!(self.process_after_generics(tt), tt);
                }

                self.where_clause.extend(once(tt));
            }
        }

        self.where_clause_span = self.last_span;


        for tt in &mut self.parsing {
            self.last_span = tt.span();
            self.after_where.extend(once(tt));
        }
        self.after_where_span = self.last_span;
    }

    fn process_after_generics(&mut self, tt: TokenTree) -> Option<TokenTree> {
        match &tt {
            TokenTree::Ident(ident) if 
                mmatches!(self.location, ParseLocation::AfterGenerics) &&
                ident.to_string() == "where" 
            => {
                self.curr_token_kind = TokenKind::Where;
                self.location = ParseLocation::InWhere;
                None
            }
            TokenTree::Punct(punct) if {
                let c = punct.as_char();
                c == ';' || c == '=' && punct.spacing() == Spacing::Alone
            } => {
                self.where_clause.extend(self.get_trailing_comma());
                
                self.after_where.extend(once(tt));
                self.location = ParseLocation::AfterWhere;
                
                None
            }
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                self.where_clause.extend(self.get_trailing_comma());
                
                self.after_where.extend(once(tt));
                self.location = ParseLocation::AfterWhere;
                
                None
            }
            _ => Some(tt),
        }
    }

    fn get_trailing_comma(&self) -> Option<TokenTree> {
        if let (ParseLocation::InWhere, TokenKind::Other) = (self.location, self.prev_token_kind) {
            let mut p = Punct::new(',', Spacing::Alone);
            p.set_span(self.last_span);
            Some(TokenTree::Punct(p))
        } else {
            None
        }
    }

    // Processes any pair of `<` and `>`
    fn process_generic_list(&mut self, tt: TokenTree) -> Option<TokenTree> {
        self.last_span = tt.span();
        self.prev_is_joint = self.curr_is_joint;
        self.curr_is_joint = false;

        self.prev_token_kind = self.curr_token_kind;
        self.curr_token_kind = TokenKind::Other;

        if let TokenTree::Punct(punct) = &tt {
            let char = punct.as_char();
            self.curr_is_joint = char == '-' ||
                punct.spacing() == Spacing::Joint && char != '>' && char != '<';

            if char == ',' {
                self.curr_token_kind = TokenKind::Comma;
            }

            if char == '<' {
                self.depth += 1;
            } if !self.prev_is_joint && char == '>' {
                if self.depth == 0 {
                    if mmatches!(self.location, ParseLocation::InGenerics) {
                        return None;
                    } 
                } else {
                    self.depth -= 1;
                }
            }
        }

        Some(tt)
    }
}


#[derive(Copy, Clone)]
enum ParseLocation {
    InGenerics,
    AfterGenerics,
    InWhere,
    AfterWhere,
}


#[derive(Copy, Clone)]
enum TokenKind{
    Where,
    Comma,
    Other,
}



