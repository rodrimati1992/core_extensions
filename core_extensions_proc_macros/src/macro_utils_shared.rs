use crate::{
    used_proc_macro::{
        token_stream::IntoIter,
        Delimiter, Group, Ident, Punct, Literal, TokenStream, TokenTree, Spacing, Span,
    },
    parsing_shared::{parenthesize_ts, out_ident},
    mmatches, try_,
};

use core::{
    iter::{Peekable, once},
    ops::Range,
};

use alloc::{
    string::{String, ToString},
    format,
};



pub(crate) mod cmp_ts;
pub(crate) mod list_generation;



pub(crate) fn macro_span() -> Span {
    #[cfg(not(feature = "rust_1_45"))]
    let span = Span::call_site();

    #[cfg(feature = "rust_1_45")]
    let span = Span::mixed_site();

    span
}


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
} pub(crate) use match_token;


#[allow(dead_code)]
pub(crate) fn parse_integer<I>(mut input: I) -> crate::Result<usize>
where
    I: Iterator<Item = TokenTree>
{
    match_token!{"expected a decimal integer", input.next() => 
        Some(TokenTree::Literal(lit)) => {
            lit.to_string().parse::<usize>()
                .map_err(|_| crate::Error::one_tt(lit.span(), "expected a decimal integer") )
        }
    }
}

pub(crate) fn parse_count_param<I>(input: I) -> crate::Result<(usize, Span)> 
where
    I: IntoIterator<Item = TokenTree>,
{
    const MSG: &str = "\
        expected either `count(....)` or an integer literal\
    ";

    let sident;

    let mut input = input.into_iter();

    match_token!{MSG, input.next() =>
        Some(TokenTree::Ident(ident)) if {
            sident = ident.to_string();
            sident == "count"
        } => {
            match_token!{"expected parentheses", input.next() =>
                Some(TokenTree::Group(group)) => {
                    Ok((group.stream().into_iter().count() as usize, group.span()))
                }
            }
        }
        Some(TokenTree::Group(group)) if mmatches!(group.delimiter(), Delimiter::None) => {
            let mut iter = group.stream().into_iter();
            let res = parse_count_param(&mut iter);

            if let Some(tt) = iter.next() {
                return Err(Error::one_tt(tt.span(), "Expected no more tokens after integer"));
            }

            res
        }
        Some(TokenTree::Literal(lit)) => {
            const IL_MSG: &str = "could not parse integer literal";

            let int = try_!(lit.to_string().parse::<usize>(),
                map_err = |_| crate::Error::one_tt(lit.span(), IL_MSG) 
            );

            Ok((int, lit.span()))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct RangeB {
    pub(crate) start: usize,
    pub(crate) end: Option<usize>,
    pub(crate) spans: Spans,
}

pub(crate) fn parse_start_bound(input: &mut Peekable<IntoIter>) -> crate::Result<(usize, Span)> {
    match input.peek() {
        Some(TokenTree::Punct(p)) if p.as_char() == '.' => Ok((0, p.span())),
        _=> parse_count_param(&mut *input)
    }
}

pub(crate) fn parse_range_param(input: &mut Peekable<IntoIter>) -> crate::Result<RangeB> {
    let (start, start_span) = try_!(parse_start_bound(&mut *input));
    let (end, end_span);

    let range_ty = try_!(parse_range_operator(&mut *input));
    
    match range_ty {
        RangeType::Inclusive|RangeType::Exclusive=> {
            let (end_, end_span_) = try_!(parse_count_param(input));
            end = if let RangeType::Inclusive = range_ty {
                Some(end_.saturating_add(1))
            } else {
                Some(end_)
            };
            end_span = end_span_;
        }
        RangeType::RangeStart => {
            end = None;
            end_span = start_span;
        }
    }

    let spans = Spans {start: start_span, end: end_span};
    Ok(RangeB{start, end, spans})
}

pub(crate) fn parse_bounded_range_param(
    input: &mut Peekable<IntoIter>,
) -> crate::Result<Range<usize>> {
    let RangeB{start, end, spans} = try_!(parse_range_param(input));
    const ERR_MSG: &str =  "Expected a finite range";
    let end = match end {
        Some(x) => x,
        None => return Err(crate::Error::with_spans(spans, ERR_MSG)),
    };
    Ok(start .. end)
}

// Implicitly unbounded
pub(crate) fn parse_int_or_range_param(
    input: &mut Peekable<IntoIter>,
) -> crate::Result<Range<usize>> {
    let (start, _) = try_!(parse_start_bound(&mut *input));
    
    let end = match try_!(parse_range_operator_opt(&mut *input)) {
        Some(RangeType::RangeStart) => !0,
        Some(range_ty) => {
            let (end_, _) = try_!(parse_count_param(input));
            if let RangeType::Inclusive = range_ty {
                end_.saturating_add(1)
            } else {
                end_
            }
        },
        None => start.saturating_add(1),
    };

    Ok(start .. end)
}



pub(crate)  enum RangeType {
    Inclusive,
    Exclusive,
    RangeStart,
}

fn parse_range_operator_inner(input: &mut Peekable<IntoIter>) -> crate::Result<RangeType> { 
    match_token!{"expected a range", input.next() =>
        Some(TokenTree::Punct(_)) => {}
    }

    match input.peek() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {
            input.next();
            Ok(RangeType::Inclusive)
        },
        Some(_) => Ok(RangeType::Exclusive),
        None=> Ok(RangeType::RangeStart),
    }
}

fn parse_range_operator(input: &mut Peekable<IntoIter>) -> crate::Result<RangeType> {
    match_token!{"expected a range", input.next() => 
        Some(TokenTree::Punct(punct)) if
            punct.as_char() == '.' && mmatches!(punct.spacing(), Spacing::Joint) 
        => {
            parse_range_operator_inner(input)
        }
    }
}

fn parse_range_operator_opt(input: &mut Peekable<IntoIter>) -> crate::Result<Option<RangeType>> {
    match input.next() {
        Some(TokenTree::Punct(punct)) if
            punct.as_char() == '.' && mmatches!(punct.spacing(), Spacing::Joint) 
        => {
            parse_range_operator_inner(input).map(Some)
        }
        Some(tt) => Err(crate::Error::one_tt(tt.span(), "expected a range")),
        None => Ok(None)
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

#[allow(dead_code)]
pub(crate) fn assert_parentheses(tt: TokenTree) -> crate::Result<Group> {
    match tt {
        TokenTree::Group(group) if mmatches!(group.delimiter(), Delimiter::Parenthesis) => 
            Ok(group),
        tt => Err(crate::Error::one_tt(tt.span(), "expected parentheses"))
    }
}

pub(crate) fn parse_group<I>(mut input: I) -> crate::Result<Group>
where
    I: Iterator<Item = TokenTree>
{
    match_token!{"expected `(`, `{`, or `[`", input.next() => 
        Some(TokenTree::Group(group)) if !mmatches!(group.delimiter(), Delimiter::None) => {
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
        Some(TokenTree::Group(group)) if mmatches!(group.delimiter(), Delimiter::None) => {
            parse_ident(group.stream().into_iter())
        }
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
    iter: I
) -> crate::Result<MacroInvocation> 
where
    I: IntoIterator<Item = TokenTree>
{
    let mut path_bang = TokenStream::new();

    let mut iter = iter.into_iter();

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


pub(crate) struct PathAndSpan {
    pub(crate) path: TokenStream,
    pub(crate) start_span: Span,
    pub(crate) end_span: Span,
    pub(crate) terminator: Option<TokenTree>,
}

pub(crate) fn parse_path_and_span<I>(
    iter: I
) -> crate::Result<PathAndSpan> 
where
    I: IntoIterator<Item = TokenTree>
{
    let mut this = PathAndSpan{
        path: TokenStream::new(),
        start_span: Span::call_site(),
        end_span: Span::call_site(),
        terminator: None,
    };

    let mut start = true;

    for tt in iter {
        if start {
            this.start_span = tt.span();
            start = false;
        } else {
            this.end_span = tt.span();
        };

        macro_rules! return_ {
            ($tt:expr) => ({
                this.terminator = Some($tt);
                return Ok(this);
            });
        }

        match tt {
            TokenTree::Group(group) => {
                if mmatches!(group.delimiter(), Delimiter::None) {
                    this.path.extend(group.stream());
                } else {
                    return_!(TokenTree::Group(group))
                }
            }
            TokenTree::Punct(punct) => {
                if punct.as_char() == ':' {
                    this.path.extend(once(TokenTree::Punct(punct)));
                } else {
                    return_!(TokenTree::Punct(punct))
                }
            },
            tt @ TokenTree::Literal(_) => return_!(tt),
            x @ TokenTree::Ident(_) => {
                this.path.extend(once(x));
            }
        }
    }

    Ok(this)
}


////////////////////////////////////////////////////////////////////////////////


pub(crate) fn expect_no_tokens<I>(iter: I) -> Result<(), crate::Error>
where
    I: IntoIterator<Item = TokenTree>
{
    if let Some(tt) = iter.into_iter().next() {
        let msg = "expected no more tokens, starting from this one";
        Err(crate::Error::one_tt(tt.span(), msg))
    } else {
        Ok(())
    }
}


////////////////////////////////////////////////////////////////////////////////

pub(crate) fn usize_tt(n: usize, span: Span) -> TokenTree {
    let mut lit = Literal::usize_unsuffixed(n);
    lit.set_span(span);
    TokenTree::Literal(lit)
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn out_parenthesized_tt(tt: TokenTree, out: &mut TokenStream) {
    let span = tt.span();
    out.extend(once(parenthesize_ts(tt.into(), span)));
}

pub(crate) fn braced_ts(ts: TokenStream, span: Span) -> TokenTree {
    let mut group = Group::new(Delimiter::Brace, ts);
    group.set_span(span);
    TokenTree::Group(group)
}

pub(crate) fn out_braced_tt(tt: TokenTree, out: &mut TokenStream) {
    let span = tt.span();
    out.extend(once(braced_ts(tt.into(), span)));
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub(crate) struct Spans {
    pub(crate) start: Span,
    pub(crate) end: Span,
}

impl Spans {
    #[inline(always)]
    pub(crate) fn new(start: Span, end: Span) -> Self {
        Self{start, end}
    }
}

////////////////////////////////////////////////////////////////////////////////


pub(crate) struct Error {
    spans: Spans,
    message: String,
}

impl Error {
    #[allow(dead_code)]
    pub(crate) fn new(start_span: Span, end_span: Span, message: &str) -> Self {
        Self {
            spans: Spans::new(start_span, end_span),
            message: message.into(),
        }
    }

    pub(crate) fn with_spans(spans: Spans, message: &str) -> Self {
        Self {
            spans,
            message: message.into(),
        }
    }

    pub(crate) fn one_tt(span: Span, message: &str) -> Self {
        Self {
            spans: Spans::new(span, span),
            message: message.into(),
        }
    }

    pub(crate) fn end(message_: &str) -> Self {
        let mut message = "tokens ended before parsing finished, ".to_string();
        message.push_str(message_);

        Self {
            spans: Spans::new(Span::call_site(), Span::call_site()),
            message,
        }
    }

    pub(crate) fn start_span(&self) -> Span {
        self.spans.start
    }
    #[allow(dead_code)]
    pub(crate) fn end_span(&self) -> Span {
        self.spans.end
    }

    pub(crate) fn into_compile_error(self) -> TokenStream {
        self.to_compile_error()
    }
    pub(crate) fn to_compile_error(&self) -> TokenStream {
        let Error { ref message, spans: Spans{start: start_span, end: end_span} } = *self;

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

impl From<Error> for TokenStream {
    fn from(err: Error) -> TokenStream {
        err.into_compile_error()
    }
}



