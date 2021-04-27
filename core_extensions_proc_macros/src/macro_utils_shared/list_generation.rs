use crate::{
    used_proc_macro::{
        token_stream::IntoIter,
        Delimiter, Group, TokenStream, TokenTree,
    },
    macro_utils_shared::{
        RangeB, Spans,
        match_token,
        parse_parentheses, parse_range_param,
        usize_tt,
    },
    mmatches, try_,
};

use core::ops::RangeFrom;

use alloc::{
    string::ToString,
    format,
};

pub(crate) enum List {
    List(TokenStream, Spans),
    RangeFrom(usize, Spans),
}


impl List {
    #[allow(dead_code)]
    pub(crate) fn spans(&self) -> Spans {
        match self {
            Self::List(_, x) | Self::RangeFrom(_, x) => *x,
        }
    }
    pub(crate) fn is_finite(&self) -> bool {
        mmatches!(self, Self::List{..})
    }
}



fn parse_impl<I, C>(mut iter: I) -> crate::Result<C::This>
where
    I: Iterator<Item = TokenTree>,
    C: Constructors,
{
    macro_rules! list_functions {
        (
            $ident:ident,
            $group:ident,
            $stream_iter:ident,
            $fname:literal => $fblock:block
            $( $name:literal => $block:block )* 
        ) => {
            macro_rules! method_names {
                () => {
                    concat!(
                        "one of ",
                        "`", $fname, "`",
                        $(", `", $name, "`",)*
                    )
                };
            }

            const PARAM_MSG: &str = concat!("expected ", method_names!(),", or parentheses.");

            match_token!{PARAM_MSG, iter.next() =>
                Some(TokenTree::Ident($ident)) => {
                    let keyword = $ident.to_string();
                    
                    const IDENT_ERR: &str =
                        concat!("expected ", method_names!(),", or parentheses.");

                    let paren_res = parse_parentheses(&mut iter);

                    match &keyword[..] {
                        $fname => {
                            let $group = try_!(paren_res);
                            let $stream_iter = $group.stream().into_iter();

                            $fblock
                        }
                        $(
                            $name => {
                                let $group = try_!(paren_res);
                                let mut $stream_iter = $group.stream().into_iter();

                                $block
                            }
                        )*
                        other => {
                            let err = format!("{}\nFound {}", IDENT_ERR, other);
                            return Err(crate::Error::one_tt($ident.span(), &err));
                        }
                    }
                }
                Some(TokenTree::Group(group))  => {
                    Ok(C::make_group(group.stream(), Spans::new(group.span(), group.span())))
                }
            }
        };
    }


    list_functions!{
        ident,
        group,
        stream,
        "range" => {
            let mut stream = stream.peekable();
            let rangeb = try_!(parse_range_param(&mut stream));

            if let Some(rend) = rangeb.end {
                let tokens = (rangeb.start..rend)
                    .map(|i| usize_tt(i, rangeb.spans.start) )
                    .collect::<TokenStream>();
                Ok(C::make_group(tokens, Spans::new(ident.span(), group.span())))
            } else {
                C::make_range_start(rangeb)
            }
        }
    }
}


trait Constructors {
    type This;

    fn make_group(ts: TokenStream, span: Spans) -> Self::This;

    fn make_range_start(rangeb: RangeB) -> crate::Result<Self::This>;
}

struct Unbounded;

impl Constructors for Unbounded {
    type This = List;

    fn make_group(ts: TokenStream, spans: Spans) -> Self::This {
        List::List(ts, spans)
    }

    fn make_range_start(r: RangeB) -> crate::Result<Self::This> {
        Ok(List::RangeFrom(r.start, r.spans))
    }
}


struct Bounded;

impl Constructors for Bounded {
    type This = Group;

    fn make_group(ts: TokenStream, span: Spans) -> Self::This {
        let mut group = Group::new(Delimiter::Parenthesis, ts);
        group.set_span(span.start);
        group
    }

    fn make_range_start(r: RangeB) -> crate::Result<Self::This> {
        Err(crate::Error::with_spans(r.spans, "Expected a bounded range"))
    }
}


pub(crate) fn parse_unbounded<I>(iter: &mut I) -> crate::Result<List>
where
    I: Iterator<Item = TokenTree>
{
    parse_impl::<_, Unbounded>(iter)
}

pub(crate) fn parse_bounded<I>(iter: &mut I) -> crate::Result<Group>
where
    I: Iterator<Item = TokenTree>
{
    parse_impl::<_, Bounded>(iter)
}


pub(crate) enum ListIter {
    List(IntoIter),
    RangeFrom(RangeFrom<usize>, Spans),
}


impl IntoIterator for List {
    type Item = TokenTree;
    type IntoIter = ListIter;

    fn into_iter(self) -> ListIter {
        match self {
            Self::List(ts, _) => ListIter::List(ts.into_iter()),
            Self::RangeFrom(start, span) => ListIter::RangeFrom(start.., span),
        }
    }
}

impl ListIter {
    #[allow(dead_code)]
    pub(crate) fn is_finite(&self) -> bool {
        mmatches!(self, Self::List{..})
    }
}

impl Iterator for ListIter{
    type Item = TokenTree;

    fn next(&mut self) -> Option<TokenTree> {
        match self {
            Self::List(x) => x.next(),
            Self::RangeFrom(x, span) => x.next().map(|x| usize_tt(x, span.start) ),
        }
    }
}







