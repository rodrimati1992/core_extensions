use crate::{
    used_proc_macro::{
        token_stream::IntoIter,
        Delimiter, Group, Ident, Punct, Literal, TokenStream, TokenTree, Spacing, Span,
    },
    parsing_shared::out_ident,
    mmatches,
};

use core::{
    iter::{Peekable, once},
    ops::Range,
};

use alloc::{
    string::{String, ToString},
    format,
};

macro_rules! match_token {
    ($msg:expr, $matched:expr => $($branches:tt)* ) => {
        match $matched {
            $($branches)*
            Some(tt) => {
                // let mut msg = format!("{}, found: {:?}", $msg, tt);

                return Err(crate::Error::one_tt(tt.span(), &$msg));
            }
            None =>{
                return Err(crate::Error::end($msg))
            }
        }
    };
}



pub(crate) fn parse_count_param(input: &mut Peekable<IntoIter>) -> crate::Result<u32> {
    const MSG: &str = "\
        expected either `count(....)` or an integer literal\
    ";

    let sident;

    match_token!{MSG, input.next() =>
        Some(TokenTree::Ident(ident)) if {
            sident = ident.to_string();
            sident == "count"
        } => {
            match_token!{"expected parentheses", input.next() =>
                Some(TokenTree::Group(group)) => {
                    Ok(group.stream().into_iter().count() as u32)
                }
            }
        }
        Some(TokenTree::Group(group)) if mmatches!(group.delimiter(), Delimiter::None) => {
            let mut iter = group.stream().into_iter().peekable();
            let res = parse_count_param(&mut iter);

            if let Some(tt) = iter.next() {
                return Err(Error::one_tt(tt.span(), "Expected no more tokens after integer"));
            }

            res
        }
        Some(TokenTree::Literal(lit)) => {
            const IL_MSG: &str = "could not parse integer literal";

            lit.to_string().parse::<u32>()
                .map_err(|_| crate::Error::one_tt(lit.span(), IL_MSG) )
        }
    }
}

pub(crate) fn parse_range_param(input: &mut Peekable<IntoIter>) -> crate::Result<Range<u32>> {
    let first_int = parse_count_param(input)?;
    
    let range_ty = parse_range_operator(&mut *input)?;
    
    let second_int = parse_count_param(input)?;

    match range_ty {
        RangeType::Inclusive=>Ok(first_int..second_int.saturating_add(1)),
        RangeType::Exclusive=>Ok(first_int..second_int),
    }
}

pub(crate) fn parse_integer<I>(mut input: I) -> crate::Result<u32>
where
    I: Iterator<Item = TokenTree>
{
    match_token!{"expected a decimal integer", input.next() => 
        Some(TokenTree::Literal(lit)) => {
            lit.to_string().parse::<u32>()
                .map_err(|_| crate::Error::one_tt(lit.span(), "expected a decimal integer") )
        }
    }
}


pub(crate)  enum RangeType {
    Inclusive,
    Exclusive,
}

fn parse_range_operator(input: &mut Peekable<IntoIter>) -> crate::Result<RangeType> {
    const S: &str = "expected a range";

    match_token!{S, input.next() => 
        Some(TokenTree::Punct(punct)) if
            punct.as_char() == '.' && mmatches!(punct.spacing(), Spacing::Joint) 
        => {
            match_token!{"expected a range", input.next() =>
                Some(TokenTree::Punct(_)) => {}
            }

            match input.peek() {
                Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {
                    input.next();
                    Ok(RangeType::Inclusive)
                }
                Some(_) => {
                    Ok(RangeType::Exclusive)
                }
                None=> Err(crate::Error::end("expected end of range")),
            }
        }
    }
}


////////////////////////////////////////////////////////////////////////////////

pub(crate) fn parse_parentheses<I>(mut input: I) -> crate::Result<Group>
where
    I: Iterator<Item = TokenTree>
{
    match_token!{"expected parentheses", input.next() => 
        Some(TokenTree::Group(group)) if mmatches!(group.delimiter(), Delimiter::Parenthesis) => {
            Ok(group)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn parse_ident<I>(mut input: I) -> crate::Result<Ident>
where
    I: Iterator<Item = TokenTree>
{
    match_token!{"expected identifier", input.next() => 
        Some(TokenTree::Ident(ident)) => {
            Ok(ident)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn parse_keyword<I>(mut input: I, keyword: &str) -> crate::Result<Ident>
where
    I: Iterator<Item = TokenTree>
{
    match_token!{&format!("expected {:?}", keyword), input.next() => 
        Some(TokenTree::Ident(ident)) if ident.to_string() == keyword => {
            Ok(ident)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn parse_check_punct<I>(mut input: I, punct: char) -> crate::Result<Punct>
where
    I: Iterator<Item = TokenTree>
{
    match_token!{&format!("expected {:?}", punct), input.next() => 
        Some(TokenTree::Punct(p)) if p.as_char() == punct => {
            Ok(p)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////


pub(crate) struct MacroInvocation {
    pub(crate) path_bang: TokenStream,
    pub(crate) delimiter: Delimiter,
    pub(crate) delim_span: Span,
    pub(crate) args: TokenStream,
}

impl MacroInvocation {
    pub(crate) fn into_token_stream(mut self) -> TokenStream {
        let mut args = Group::new(self.delimiter, self.args);
        args.set_span(self.delim_span);
        self.path_bang.extend(once(TokenTree::Group(args)));
        self.path_bang
    }
}


pub(crate) fn parse_macro_invocation<I>(
    mut iter: I
) -> crate::Result<MacroInvocation> 
where
    I: Iterator<Item = TokenTree>
{
    let mut path_bang = TokenStream::new();

    loop {
        match iter.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::None => {
                path_bang.extend(group.stream());
            }
            Some(TokenTree::Group(group)) => {
                return Ok(MacroInvocation{
                    path_bang,
                    delimiter: group.delimiter(),
                    delim_span: group.span(),
                    args: group.stream(),
                });
            }
            Some(x) => {
                path_bang.extend(once(x));
            }
            None => {
                const M: &str =  "expected `{}` `()` or `[]` after the path to a macro";
                return Err(crate::Error::end(M));
            }
        }
    }
}


////////////////////////////////////////////////////////////////////////////////


pub(crate) struct Error {
    start_span: Span,
    end_span: Span,
    message: String,
}

impl Error {
    pub(crate) fn new(start_span: Span, end_span: Span, message: &str) -> Self {
        Self {
            start_span,
            end_span,
            message: message.into(),
        }
    }

    pub(crate) fn one_tt(span: Span, message: &str) -> Self {
        Self {
            start_span: span,
            end_span: span,
            message: message.into(),
        }
    }

    pub(crate) fn end(message_: &str) -> Self {
        let mut message = "tokens ended before parsing finished, ".to_string();
        message.push_str(message_);

        Self {
            start_span: Span::call_site(),
            end_span: Span::call_site(),
            message,
        }
    }

    pub(crate) fn into_compile_error(self) -> TokenStream {
        self.to_compile_error()
    }
    pub(crate) fn to_compile_error(&self) -> TokenStream {
        let Error { ref message, start_span, end_span } = *self;

        let mut out = TokenStream::new();

        out_ident("compile_error", start_span, &mut out);

        let mut bang = Punct::new('!', Spacing::Alone);
        bang.set_span(start_span);
        out.extend(once(TokenTree::Punct(bang)));

        let mut msg = Literal::string(message);
        msg.set_span(end_span);
        let msg = TokenStream::from(TokenTree::from(msg));

        let mut group = Group::new(Delimiter::Brace, msg);
        group.set_span(end_span);
        out.extend(once(TokenTree::Group(group)));

        out
    }
}




