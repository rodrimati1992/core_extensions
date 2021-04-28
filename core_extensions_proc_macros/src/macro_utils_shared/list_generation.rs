use crate::{
    used_proc_macro::{
        token_stream::IntoIter,
        Delimiter, Group, TokenStream, TokenTree,
    },
    macro_utils::{
        GenIdentRange,
        gen_ident_range_just_idents,
    },
    macro_utils_shared::{
        RangeB, RepeatTimes, Spans,
        expect_no_tokens,
        match_token,
        parse_check_punct, parse_count_param,
        parse_parentheses, parse_range_param, parse_unbounded_range_param,
        usize_tt,
    },
    mmatches, try_,
};

use core::{
    iter::{Chain, Cycle, Peekable},
    marker::PhantomData,
    ops::RangeFrom,
};

use alloc::{
    boxed::Box,
    string::ToString,
    format,
};

// All the finite lists should be `List::List`
pub(crate) enum List {
    List(TokenStream, Spans),
    RangeFrom(usize, Spans),
    GenIdentRange(GenIdentRange),
    Chain{
        bounded: TokenStream,
        spans: Spans,
        unbounded: Box<List>,
    },
    Cycle(TokenStream, Spans),
}


impl List {
    #[allow(dead_code)]
    pub(crate) fn spans(&self) -> Spans {
        match self {
            Self::List(_, x) | Self::RangeFrom(_, x) => *x,
            Self::GenIdentRange(gir) => {
                let s = gir.span();
                Spans{start: s, end: s}
            },
            Self::Chain{spans, ..} => *spans,
            Self::Cycle(_, x) => *x
        }
    }
    pub(crate) fn is_finite(&self) -> bool {
        mmatches!(self, Self::List{..})
    }
}


////////////////////////////////////////////////////////////////////////////////

struct ParseManyLists<C>{
    iter: Peekable<IntoIter>,
    _marker: PhantomData<C>,
}

impl<C> Iterator for ParseManyLists<C> 
where 
    C: Constructors
{
    type Item = crate::Result<C::This>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(_) = self.iter.peek() {
            Some(parse_impl::<_, C>(&mut self.iter))
        } else {
            None
        }
    }
}


////////////////////////////////////////////////////////////////////////////////

fn parse_impl<I, C>(iter: &mut I) -> crate::Result<C::This>
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

                    let paren_res = parse_parentheses(iter);

                    match &keyword[..] {
                        $fname => {
                            let $group = try_!(paren_res);
                            let $stream_iter = $group.stream().into_iter();

                            $fblock
                        }
                        $(
                            $name => {
                                let $group = try_!(paren_res);
                                #[allow(unused_mut)]
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
        "cycle" => {
            let mut args = stream.into_iter();
            let tokens = try_!(parse_bounded(&mut args));

            try_!(expect_no_tokens(args));
            C::make_cycle(tokens.stream(), Spans::new(ident.span(), group.span()))
        }
        "repeat" => {
            let mut args = stream.into_iter();

            let times = try_!(parse_count_param(&mut args)).0;

            try_!(parse_check_punct(&mut args, ','));

            let repeated = try_!(parse_bounded(&mut args)).stream().into_iter();

            let tokens = if times == 0 {
                TokenStream::new()
            } else {
                RepeatTimes::new(times, repeated).collect()
            };

            try_!(expect_no_tokens(args));
            Ok(C::make_group(tokens, Spans::new(ident.span(), group.span())))
        }
        "take" => {
            let mut args = stream.into_iter();

            let count = try_!(parse_count_param(&mut args)).0;

            try_!(parse_check_punct(&mut args, ','));

            let tokens = try_!(parse_unbounded(&mut args)).into_iter().take(count).collect();

            try_!(expect_no_tokens(args));
            Ok(C::make_group(tokens, Spans::new(ident.span(), group.span())))
        }
        "chain" => {
            let iter = ParseManyLists{
                iter: stream.into_iter().peekable(),
                _marker: PhantomData,
            };
            
            C::make_chain(iter, Spans::new(ident.span(), group.span()))
        }
        "gen_ident_range" => {
            let range = try_!(gen_ident_range_just_idents(
                &mut stream.peekable(),
                parse_unbounded_range_param,
            ));

            C::make_gen_idents_range(range, Spans::new(ident.span(), group.span()))
        }
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


trait Constructors: Sized {
    type This;

    fn make_cycle(ts: TokenStream, span: Spans) -> crate::Result<Self::This>;
    
    fn make_chain(_: ParseManyLists<Self>, span: Spans) -> crate::Result<Self::This>;
    
    fn make_gen_idents_range(range: GenIdentRange, span: Spans) -> crate::Result<Self::This>;

    fn make_group(ts: TokenStream, span: Spans) -> Self::This;

    fn make_range_start(rangeb: RangeB) -> crate::Result<Self::This>;
}

struct Unbounded;

impl Constructors for Unbounded {
    type This = List;
    
    fn make_cycle(ts: TokenStream, spans: Spans) -> crate::Result<Self::This> {
        Ok(List::Cycle(ts, spans))
    }

    fn make_chain(iter: ParseManyLists<Self>, mut spans: Spans) -> crate::Result<Self::This> {
        let mut bounded = TokenStream::new();
        let mut unbounded = None::<Box<List>>;

        for elem in iter {
            let elem = try_!(elem);

            spans.end = elem.spans().end;

            if let None = unbounded {
                if let List::List(list, _) = elem {
                    bounded.extend(list);
                } else {
                    unbounded = Some(Box::new(elem))
                }
            }
        }

        if let Some(unbounded) = unbounded {
            Ok(List::Chain{bounded, spans, unbounded})
        } else {
            Ok(List::List(bounded, spans))
        }
    }

    fn make_gen_idents_range(range: GenIdentRange, spans: Spans) -> crate::Result<Self::This> {
        if range.is_unbounded() {
            Ok(List::GenIdentRange(range))
        } else {
            Ok(Self::make_group(range.collect(), spans))
        }
    }

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

    fn make_cycle(_: TokenStream, spans: Spans) -> crate::Result<Self::This> {
        Err(crate::Error::with_spans(spans, "cannot use `cycle` here"))
    }

    fn make_chain(iter: ParseManyLists<Self>, span: Spans) -> crate::Result<Self::This> {
        let mut tokens = TokenStream::new();

        for elem in iter {
            let elem = try_!(elem);
            tokens.extend(elem.stream());
        }

        let mut group = Group::new(Delimiter::Parenthesis, tokens);
        group.set_span(span.start);
        Ok(group)
    }
    
    fn make_gen_idents_range(range: GenIdentRange, spans: Spans) -> crate::Result<Self::This> {
        if range.is_unbounded() {
            Err(crate::Error::with_spans(spans, "expected bounded range"))
        } else {
            Ok(Self::make_group(range.collect(), spans))
        }
    }

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
    GenIdentRange(GenIdentRange),
    Chain(Chain<IntoIter, Box<ListIter>>),
    Cycle(Cycle<IntoIter>),
}


impl IntoIterator for List {
    type Item = TokenTree;
    type IntoIter = ListIter;

    fn into_iter(self) -> ListIter {
        match self {
            Self::List(ts, _) => ListIter::List(ts.into_iter()),
            Self::RangeFrom(start, span) => ListIter::RangeFrom(start.., span),
            Self::GenIdentRange(gir) => ListIter::GenIdentRange(gir),
            Self::Chain{bounded, unbounded, ..} =>
                ListIter::Chain(bounded.into_iter().chain(Box::new(unbounded.into_iter()))),
            Self::Cycle(x, _) => ListIter::Cycle(x.into_iter().cycle()),
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
            Self::GenIdentRange(x) => x.next(),
            Self::Chain(x) => x.next(),
            Self::Cycle(x) => x.next(),
        }
    }
}







