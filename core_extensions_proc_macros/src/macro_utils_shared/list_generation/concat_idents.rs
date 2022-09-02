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
        CountAnd, parse_count_and,
        expect_no_tokens,
        match_token,
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