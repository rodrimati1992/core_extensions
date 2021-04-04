use super::StringExt;

use std_::mem;
use std_::str::CharIndices;

#[inline(always)]
fn next_split<'a, P, T: Eq + Clone>(
    pred: &mut P,
    s: &mut &'a str,
    last: &mut T,
) -> Option<KeyStr<'a, T>>
where
    P: FnMut(char) -> T,
{
    let mut next = last.clone();
    if s.is_empty() {
        return None;
    }
    let end = s
        .find(|c| {
            next = pred(c);
            *last != next
        })
        .map_or(s.len(), |v| v);
    let (ret, new_s) = s.split_at(end);
    *s = new_s;
    let key = mem::replace(last, next);
    Some(KeyStr { str: ret, key })
}

#[inline(always)]
fn next_rsplit<'a, P, T: Eq + Clone>(
    pred: &mut P,
    s: &mut &'a str,
    last: &mut T,
) -> Option<KeyStr<'a, T>>
where
    P: FnMut(char) -> T,
{
    let mut next = last.clone();
    if s.is_empty() {
        return None;
    }
    let left = s
        .rfind(|c| {
            next = pred(c);
            *last != next
        })
        .map_or(0, |v| s.next_char_boundary(v));
    let (new_s, ret) = s.split_at(left);
    *s = new_s;
    let key = mem::replace(last, next);
    Some(KeyStr { str: ret, key })
}

//-------------------------------------------------------------------------------------------

/// A pair of (string slice, key) returned by the 
/// [RSplitWhile](struct.RSplitWhile.html)/
/// [SplitWhile](struct.SplitWhile.html) iterators.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct KeyStr<'a, T> {
    /// `str_slice` is a string slice for which all characters were mapped to `key` by a closure.
    pub str: &'a str,
    /// The value that all the chars in the string slice were mapped to.
    pub key: T,
}

impl<'a, T> KeyStr<'a, T> {
    /// Converts this into a key, string slice pair
    pub fn into_pair(self)->(T,&'a str){
        (self.key,self.str)
    }
}

//-------------------------------------------------------------------------------------------

/// Iterator over string slices,
/// in which all the chars in each string were mapped to the same key by a closure.
///
/// Look [here](trait.StringExt.html#method.split_while) for examples.
#[derive(Debug, Clone)]
pub struct SplitWhile<'a, P, T> {
    pub(super) mapper: P,
    pub(super) s: &'a str,
    pub(super) last_left: T,
    pub(super) last_right: T,
}

impl<'a, P, T: Eq + Clone> Iterator for SplitWhile<'a, P, T>
where
    P: FnMut(char) -> T,
{
    type Item = KeyStr<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        next_split(&mut self.mapper, &mut self.s, &mut self.last_left)
    }
}

impl<'a, P, T: Eq + Clone> DoubleEndedIterator for SplitWhile<'a, P, T>
where
    P: FnMut(char) -> T,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        next_rsplit(&mut self.mapper, &mut self.s, &mut self.last_right)
    }
}

//-------------------------------------------------------------------------------------------

/// Iterator over string slices,
/// in which all the chars in each string were mapped to the same key by a closure,
/// iterating from the end.
///
/// Look [here](trait.StringExt.html#method.rsplit_while) for examples.
#[derive(Debug, Clone)]
pub struct RSplitWhile<'a, P, T> {
    pub(super) mapper: P,
    pub(super) s: &'a str,
    pub(super) last_left: T,
    pub(super) last_right: T,
}

impl<'a, P, T: Eq + Clone> Iterator for RSplitWhile<'a, P, T>
where
    P: FnMut(char) -> T,
{
    type Item = KeyStr<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        next_rsplit(&mut self.mapper, &mut self.s, &mut self.last_right)
    }
}

impl<'a, P, T: Eq + Clone> DoubleEndedIterator for RSplitWhile<'a, P, T>
where
    P: FnMut(char) -> T,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        next_split(&mut self.mapper, &mut self.s, &mut self.last_left)
    }
}

//-------------------------------------------------------------------------------------------

/// Like [`CharIndices`], which starts from an offset.
///
/// Look [here](trait.StringExt.html#method.char_indices_from) for examples.
///
/// # Motivation
/// 
/// The reason to use this instead of `string[from..].char_indices()` is that 
/// this gives you the position of the characters in `string`, 
/// while [`CharIndices`] gives you the position of the characters in `string[from..]`.
/// 
/// [`CharIndices`]: https://doc.rust-lang.org/std/str/struct.CharIndices.html
#[derive(Clone, Debug)]
pub struct CharIndicesFrom<'a> {
    pub(super) offset: usize,
    pub(super) iter: CharIndices<'a>,
}

impl<'a> Iterator for CharIndicesFrom<'a> {
    type Item = (usize, char);

    #[inline]
    fn next(&mut self) -> Option<(usize, char)> {
        self.iter.next().map(|(i, c)| (i + self.offset, c))
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(mut self) -> Option<(usize, char)> {
        self.next_back()
    }
}

impl<'a> DoubleEndedIterator for CharIndicesFrom<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<(usize, char)> {
        self.iter.next_back().map(|(i, c)| (i + self.offset, c))
    }
}

impl<'a> CharIndicesFrom<'a> {
    /// Returns the rest of the slice to be iterated over.
    pub fn as_str(&self) -> &'a str {
        self.iter.as_str()
    }
}
