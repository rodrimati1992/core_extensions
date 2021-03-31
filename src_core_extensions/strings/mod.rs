//! Extension trait for string types.

use std_::borrow::Borrow;
use std_::cmp;
use std_::fmt;
use std_::str::CharIndices;

#[cfg(feature = "alloc")]
use alloc_::string::String;

mod iterators;

pub use self::iterators::{CharIndicesFrom, KeyStr, RSplitWhile, SplitWhile};

/// Extension trait for strings (any type that borrows as `str`).
pub trait StringExt: Borrow<str> {
    /// Returns the previous character boundary, stopping at 0.
    ///
    /// if `index > self.len()`, returns `self.len()`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "foo速度惊人";
    ///
    /// assert_eq!(word.previous_char_boundary(0), 0);
    /// assert_eq!(word.previous_char_boundary(1), 0);
    /// assert_eq!(word.previous_char_boundary(2), 1);
    /// assert_eq!(word.previous_char_boundary(3), 2);
    ///
    /// // This input index is inside of '速'
    /// assert_eq!(word.previous_char_boundary(4), 3);
    /// assert_eq!(word.previous_char_boundary(5), 3);
    /// assert_eq!(word.previous_char_boundary(6), 3);
    ///
    /// // This input index is inside of '度'
    /// assert_eq!(word.previous_char_boundary(7), 6);
    ///
    /// assert_eq!(word.previous_char_boundary(10000), word.len());
    ///
    /// ```
    ///
    fn previous_char_boundary(&self, mut index: usize) -> usize {
        let this = self.borrow();
        if index > this.len() {
            return this.len();
        }
        index = index.saturating_sub(1);
        while !this.is_char_boundary(index) {
            index -= 1;
        }
        index
    }
    /// Returns the next character boundary.
    ///
    /// If `index > self.len()`, returns `self.len()`
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "foo誰もが";
    ///
    /// assert_eq!(word.next_char_boundary(0), 1);
    /// assert_eq!(word.next_char_boundary(1), 2);
    /// assert_eq!(word.next_char_boundary(2), 3);
    ///
    /// // This input index is inside of '誰'
    /// assert_eq!(word.next_char_boundary(3), 6);
    /// assert_eq!(word.next_char_boundary(4), 6);
    /// assert_eq!(word.next_char_boundary(5), 6);
    ///
    /// // This input index is inside of 'も'
    /// assert_eq!(word.next_char_boundary(6), 9);
    ///
    /// assert_eq!(word.next_char_boundary(10000), word.len());
    ///
    /// ```
    ///
    fn next_char_boundary(&self, mut index: usize) -> usize {
        let this = self.borrow();
        if index >= this.len() {
            return this.len();
        }
        index += 1;
        while !this.is_char_boundary(index) {
            index += 1;
        }
        index
    }
    /// Returns the closest characted boundary left of `index`(including `index`).
    ///
    /// if `index > self.len()`, returns `self.len()`
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "barЯзык";
    ///
    /// assert_eq!(word.left_char_boundary(0), 0);
    /// assert_eq!(word.left_char_boundary(1), 1);
    /// assert_eq!(word.left_char_boundary(2), 2);
    ///
    /// // The input index is inside of 'Я'
    /// assert_eq!(word.left_char_boundary(3), 3);
    /// assert_eq!(word.left_char_boundary(4), 3);
    ///
    /// // The input index is inside of 'з'
    /// assert_eq!(word.left_char_boundary(5), 5);
    /// assert_eq!(word.left_char_boundary(6), 5);
    ///
    /// assert_eq!(word.left_char_boundary(10000), word.len());
    ///
    /// ```
    fn left_char_boundary(&self, mut index: usize) -> usize {
        let this = self.borrow();
        if index > this.len() {
            return this.len();
        }
        while !this.is_char_boundary(index) {
            index -= 1;
        }
        index
    }
    /// Returns the closest characted boundary right of `index`(including `index`).
    ///
    /// if `index > self.len()`, returns `self.len()`
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "rápido";
    ///
    /// assert_eq!(word.right_char_boundary(0),0);
    ///
    /// // The input index is inside of 'á'
    /// assert_eq!(word.right_char_boundary(1), 1);
    /// assert_eq!(word.right_char_boundary(2), 3);
    ///
    /// assert_eq!(word.right_char_boundary(3), 3);
    /// assert_eq!(word.right_char_boundary(4), 4);
    ///
    /// assert_eq!(word.right_char_boundary(10000), word.len());
    ///
    /// ```
    fn right_char_boundary(&self, mut index: usize) -> usize {
        let this = self.borrow();
        if index >= this.len() {
            return this.len();
        }
        while !this.is_char_boundary(index) {
            index += 1;
        }
        index
    }
    /// Returns an iterator over substrings whose characters were mapped to
    /// the same key by `mapper`.
    ///
    /// The returned type implements 
    /// `DoubleEndedIterator<Item =`[KeyStr](./struct.KeyStr.html)`<T>>`.
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::{StringExt, KeyStr};
    ///
    /// assert_eq!(
    ///     "Hello, world!".split_while(|c| c.is_alphanumeric()).collect::<Vec<_>>(),
    ///     vec![
    ///         KeyStr{key: true, str: "Hello"},
    ///         KeyStr{key: false, str: ", "},
    ///         KeyStr{key: true, str: "world"},
    ///         KeyStr{key: false, str: "!"},
    ///     ]
    /// );
    /// assert_eq!(
    ///     "aaabbbccc".split_while(|c| c).collect::<Vec<_>>(),
    ///     vec![
    ///         KeyStr{key: 'a', str: "aaa"},
    ///         KeyStr{key: 'b', str: "bbb"},
    ///         KeyStr{key: 'c', str: "ccc"},
    ///     ]
    /// );
    ///
    /// ```
    fn split_while<'a, P, T: Eq + Clone>(&'a self, mut mapper: P) -> SplitWhile<'a, P, T>
    where
        P: FnMut(char) -> T,
    {
        let this = self.borrow();
        let mut c = this.chars();
        SplitWhile {
            last_left: mapper(c.next().unwrap_or(' ')),
            last_right: mapper(c.next_back().unwrap_or(' ')),
            mapper,
            s: this,
        }
    }
    /// A variation of [`split_while`](#method.split_while) that iterates
    /// from the right(the order of substrings is reversed).
    ///
    /// The returned type implements 
    /// `DoubleEndedIterator<Item =`[KeyStr](./struct.KeyStr.html)`<T>>`.
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::{StringExt, KeyStr};
    ///
    /// assert_eq!(
    ///     "Hello, world!".rsplit_while(|c| c.is_alphanumeric()).collect::<Vec<_>>(),
    ///     vec![
    ///         KeyStr{key: false, str: "!"},
    ///         KeyStr{key: true, str: "world"},
    ///         KeyStr{key: false, str: ", "},
    ///         KeyStr{key: true, str: "Hello"},
    ///     ]
    /// );
    /// assert_eq!(
    ///     "aaabbbccc".rsplit_while(|c| c).collect::<Vec<_>>(),
    ///     vec![
    ///         KeyStr{key: 'c', str: "ccc"},
    ///         KeyStr{key: 'b', str: "bbb"},
    ///         KeyStr{key: 'a', str: "aaa"},
    ///     ]
    /// );
    ///
    /// ```
    fn rsplit_while<'a, P, T: Eq + Clone>(&'a self, mut mapper: P) -> RSplitWhile<'a, P, T>
    where
        P: FnMut(char) -> T,
    {
        let this = self.borrow();
        let mut c = this.chars();
        RSplitWhile {
            last_left: mapper(c.next().unwrap_or(' ')),
            last_right: mapper(c.next_back().unwrap_or(' ')),
            mapper,
            s: this,
        }
    }
    /// The byte index of the `nth` character
    ///
    /// If there is no `nth` character, this returns `None`.
    ///
    /// This operation takes `O(n)` time, where `n` is `self.len()`.
    ///
    /// # Example
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "fooпозволяющий";
    ///
    /// assert_eq!(word.get_nth_char_index(0), Some(0));
    /// assert_eq!(word.get_nth_char_index(1), Some(1));
    /// assert_eq!(word.get_nth_char_index(2), Some(2));
    /// assert_eq!(word.get_nth_char_index(3), Some(3));
    /// assert_eq!(word.get_nth_char_index(4), Some(5));
    /// assert_eq!(word.get_nth_char_index(5), Some(7));
    ///
    /// assert_eq!(word.get_nth_char_index(13), Some(23));
    /// assert_eq!(word.get_nth_char_index(14), None);
    /// ```
    fn get_nth_char_index(&self, nth: usize) -> Option<usize> {
        self.borrow().char_indices().nth(nth).map(|(i, _)| i)
    }

    /// The byte index of the `nth` character
    ///
    /// If there is no `nth` character, this returns `self.len()`.
    ///
    /// This operation takes `O(n)` time, where `n` is `self.len()`.
    ///
    /// # Example
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "fooпозволяющий";
    ///
    /// assert_eq!(word.nth_char_index(0), 0);
    /// assert_eq!(word.nth_char_index(1), 1);
    /// assert_eq!(word.nth_char_index(2), 2);
    /// assert_eq!(word.nth_char_index(3), 3);
    /// assert_eq!(word.nth_char_index(4), 5);
    /// assert_eq!(word.nth_char_index(5), 7);
    ///
    /// assert_eq!(word.nth_char_index(13), 23);
    /// assert_eq!(word.nth_char_index(14), word.len());
    /// ```
    fn nth_char_index(&self, nth: usize) -> usize {
        let this = self.borrow();
        this.borrow()
            .char_indices()
            .nth(nth)
            .map_or(this.len(), |(i, _)| i)
    }

    /// Returns the `nth` character in the str.
    ///
    /// This operation takes `O(n)` time, where `n` is `self.len()`.
    ///
    /// # Example
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "débuter";
    ///
    /// assert_eq!(word.nth_char(0), Some('d'));
    /// assert_eq!(word.nth_char(1), Some('é'));
    /// assert_eq!(word.nth_char(2), Some('b'));
    /// assert_eq!(word.nth_char(3), Some('u'));
    /// assert_eq!(word.nth_char(4), Some('t'));
    /// assert_eq!(word.nth_char(5), Some('e'));
    /// assert_eq!(word.nth_char(6), Some('r'));
    /// assert_eq!(word.nth_char(7), None);
    /// ```
    fn nth_char(&self, nth: usize) -> Option<char> {
        self.borrow().chars().nth(nth)
    }

    /// Returns a string containing the first `n` chars.
    ///
    /// if `n` is greater than the amount of chars, this returns the entire string.
    ///
    /// # Example
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "сине";
    ///
    /// assert_eq!(word.first_chars(0),"");
    /// assert_eq!(word.first_chars(1),"с");
    /// assert_eq!(word.first_chars(2),"си");
    /// assert_eq!(word.first_chars(3),"син");
    /// assert_eq!(word.first_chars(4),"сине");
    /// assert_eq!(word.first_chars(5),"сине");
    /// ```
    fn first_chars(&self, n: usize) -> &str {
        let this = self.borrow();
        &this[..this.nth_char_index(n)]
    }
    /// Returns a string containing the last `n` chars
    ///
    /// if `n` is greater than the amount of chars, this returns the entire string.
    ///
    /// # Example
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "сине";
    ///
    /// assert_eq!(word.last_chars(0),"");
    /// assert_eq!(word.last_chars(1),"е");
    /// assert_eq!(word.last_chars(2),"не");
    /// assert_eq!(word.last_chars(3),"ине");
    /// assert_eq!(word.last_chars(4),"сине");
    /// assert_eq!(word.last_chars(5),"сине");
    /// ```
    fn last_chars(&self, n: usize) -> &str {
        let this = self.borrow();
        // keeps the property of this being a slice of the same region of memory
        if n == 0 {
            return &this[..0];
        }
        let index = this
            .char_indices()
            .rev()
            .skip(n - 1)
            .next()
            .map_or(0, |(i, _)| i);
        &this[index..]
    }
    /// Returns the string from the `n`th character
    ///
    /// if `n` is greater than the amount of chars, this returns an empty string.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "υιός";
    ///
    /// assert_eq!(word.from_nth_char(0), "υιός");
    /// assert_eq!(word.from_nth_char(1), "ιός");
    /// assert_eq!(word.from_nth_char(2), "ός");
    /// assert_eq!(word.from_nth_char(3), "ς");
    /// assert_eq!(word.from_nth_char(4), "");
    /// assert_eq!(word.from_nth_char(5), "");
    /// assert_eq!(word.from_nth_char(6), "");
    /// ```
    fn from_nth_char(&self, n: usize) -> &str {
        let this = self.borrow();
        &this[this.nth_char_index(n)..]
    }

    /// Returns the length of the string in utf16
    ///
    /// # Warning
    ///
    /// This is calculated every time the function is called.
    ///
    /// # Example
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// assert_eq!("foo".calc_len_utf16(), 3);
    /// assert_eq!("υιός".calc_len_utf16(), 4);
    /// assert_eq!("👪".calc_len_utf16(), 2);
    ///
    /// ```
    fn calc_len_utf16(&self) -> usize {
        self.borrow()
            .chars()
            .fold(0, |accum, c| accum + c.len_utf16())
    }
    /// Returns the character at the `at_byte` index inside of the string,
    /// returning `None` if the index is outside the string.
    ///
    /// If the index is between char boundaries,
    /// this returns the char at the previous char boundary.
    ///
    /// # Example
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "foo 効 门";
    ///
    /// assert_eq!(word.get_char_at(0), Some('f'));
    /// assert_eq!(word.get_char_at(1), Some('o'));
    /// assert_eq!(word.get_char_at(2), Some('o'));
    /// assert_eq!(word.get_char_at(3), Some(' '));
    /// assert_eq!(word.get_char_at(4), Some('効'));
    /// assert_eq!(word.get_char_at(5), Some('効'));
    /// assert_eq!(word.get_char_at(6), Some('効'));
    /// assert_eq!(word.get_char_at(7), Some(' '));
    /// assert_eq!(word.get_char_at(8), Some('门'));
    /// assert_eq!(word.get_char_at(9), Some('门'));
    /// assert_eq!(word.get_char_at(10), Some('门'));
    /// assert_eq!(word.get_char_at(11), None);
    ///
    /// ```
    ///
    fn get_char_at(&self, at_byte: usize) -> Option<char> {
        let this = self.borrow();
        if at_byte >= this.len() {
            return None;
        }
        let start = this.left_char_boundary(at_byte);
        this[start..].chars().nth(0)
    }

    /// Returns an iterator over (index,char) pairs up to 
    /// (but not including) the char at the `to` byte.
    ///
    /// if `index > self.len()`, returns an iterator over the entire string.
    ///
    /// # Example
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "foo 効 ";
    ///
    /// assert_eq!(word.char_indices_to(0).collect::<Vec<_>>(), vec![]);
    /// assert_eq!(word.char_indices_to(1).collect::<Vec<_>>(), vec![(0, 'f')]);
    /// 
    /// let expected_a = vec![(0, 'f'), (1, 'o'), (2, 'o'), (3, ' ')];
    /// assert_eq!(word.char_indices_to(4).collect::<Vec<_>>(), expected_a);
    /// assert_eq!(word.char_indices_to(5).collect::<Vec<_>>(), expected_a);
    /// assert_eq!(word.char_indices_to(6).collect::<Vec<_>>(), expected_a);
    /// 
    /// let expected_b = vec![(0, 'f'), (1, 'o'), (2, 'o'), (3, ' '), (4, '効')];
    /// assert_eq!(word.char_indices_to(7).collect::<Vec<_>>(), expected_b);
    ///     
    /// let expected_c = vec![(0, 'f'), (1, 'o'), (2, 'o'), (3, ' '), (4, '効'), (7, ' ')];
    /// assert_eq!(word.char_indices_to(8).collect::<Vec<_>>(), expected_c);
    /// assert_eq!(word.char_indices_to(100).collect::<Vec<_>>(), expected_c);
    /// ```
    ///
    fn char_indices_to(&self, to: usize) -> CharIndices {
        let this = self.borrow();
        let to = this.left_char_boundary(to);
        this[..to].char_indices()
    }

    /// Returns an iterator over (index, char) pairs, starting from the `from` byte.
    ///
    /// If the index is between char boundaries,
    /// this starts from the char at the previous char boundary.
    ///
    /// if `index > self.len()`, returns an empty iterator.
    ///
    /// # Example
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// let word = "foo 効 ";
    ///
    /// let expected_a = vec![(0, 'f'), (1, 'o'), (2, 'o'), (3, ' '), (4, '効'), (7, ' ')];
    /// assert_eq!(word.char_indices_from(0).collect::<Vec<_>>(), expected_a);
    ///
    /// let expected_b = vec![(1, 'o'), (2, 'o'), (3, ' '), (4, '効'), (7, ' ')];
    /// assert_eq!(word.char_indices_from(1).collect::<Vec<_>>(), expected_b);
    ///
    /// let expected_c = vec![(3, ' '), (4, '効'), (7, ' ')];
    /// assert_eq!(word.char_indices_from(3).collect::<Vec<_>>(), expected_c);
    ///
    /// let expected_c = vec![(4, '効'), (7, ' ')];
    /// assert_eq!(word.char_indices_from(4).collect::<Vec<_>>(), expected_c);
    /// assert_eq!(word.char_indices_from(5).collect::<Vec<_>>(), expected_c);
    /// assert_eq!(word.char_indices_from(6).collect::<Vec<_>>(), expected_c);
    /// 
    /// assert_eq!(word.char_indices_from(7).collect::<Vec<_>>(), vec![(7, ' ')]);
    ///
    /// assert_eq!(word.char_indices_from(8).collect::<Vec<_>>(), vec![]);
    ///
    /// assert_eq!(word.char_indices_from(9).collect::<Vec<_>>(), vec![]);
    ///
    /// ```
    fn char_indices_from(&self, from: usize) -> CharIndicesFrom {
        let this = self.borrow();
        let from = this.left_char_boundary(from);
        CharIndicesFrom {
            offset: from,
            iter: this[from..].char_indices(),
        }
    }

    /// Pads the string on the left with `how_much` additional spaces.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// assert_eq!(
    ///     "what\n  the\n    hall".left_pad(4),
    ///     "    what\n      the\n        hall"
    /// );
    /// ```
    ///
    #[cfg(feature = "alloc")]
    fn left_pad(&self, how_much: usize) -> String {
        use alloc_::string::ToString;
        self.left_padder(how_much).to_string()
    }
    /// Returns a value that pads the string on the left with `how_much` additional
    /// spaces in its `Display` impl.
    ///
    /// Use this to avoid allocating an extra string.
    ///
    /// # Example
    ///
    #[cfg_attr(not(feature = "alloc"), doc = " ```ignore")]
    #[cfg_attr(feature = "alloc", doc = " ```rust")]
    /// use core_extensions::StringExt;
    ///
    /// assert_eq!(
    ///     "what\n  the\n    hall".left_pad(4).to_string(),
    ///     "    what\n      the\n        hall"
    /// );
    /// ```
    ///
    fn left_padder<'a>(&'a self, how_much: usize) -> LeftPadder<'a> {
        LeftPadder::new(self.borrow(), how_much)
    }
    /// The indentation of the first line.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// assert_eq!("".line_indentation(), 0);
    /// assert_eq!("    ".line_indentation(), 4);
    /// assert_eq!("    \n      word".line_indentation(), 4);
    /// assert_eq!("    \nword".line_indentation(), 4);
    ///
    /// ```
    ///
    #[cfg(any(core_str_methods, feature = "alloc"))]
    fn line_indentation(&self) -> usize {
        let this = self.borrow().lines().next().unwrap_or("");
        this.len() - this.trim_start().len()
    }

    /// The minimum indentation of the string, ignoring lines that only contain whitespace.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// assert_eq!("".min_indentation(), 0);
    /// assert_eq!("    ".min_indentation(), 0);
    /// assert_eq!("    \nf".min_indentation(), 0);
    /// assert_eq!("    \n      word".min_indentation(), 6);
    /// assert_eq!("    \n word".min_indentation(), 1);
    /// assert_eq!("    \n\nword".min_indentation(), 0);
    ///
    /// ```
    ///
    #[cfg(any(core_str_methods, feature = "alloc"))]
    fn min_indentation(&self) -> usize {
        self.borrow()
            .lines()
            .filter(|l| !l.trim_start().is_empty())
            .map(|v| v.line_indentation())
            .min()
            .unwrap_or(0)
    }
    /// The maximum indentation of the string, ignoring lines that only contain whitespace.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::StringExt;
    ///
    /// assert_eq!("".max_indentation(), 0);
    /// assert_eq!("    ".max_indentation(), 0);
    /// assert_eq!("    \n      word".max_indentation(), 6);
    /// assert_eq!("    \n  word".max_indentation(), 2);
    ///
    /// ```
    ///
    #[cfg(any(core_str_methods, feature = "alloc"))]
    fn max_indentation(&self) -> usize {
        self.borrow()
            .lines()
            .filter(|l| !l.trim_start().is_empty())
            .map(|v| v.line_indentation())
            .max()
            .unwrap_or(0)
    }
}

impl<T: ?Sized> StringExt for T where T: Borrow<str> {}

//----------------------------------------------------------------------------------------

/// Add padding to a string in its `Display` impl.
/// 
/// # Example
/// 
/// ```rust
/// use core_extensions::strings::LeftPadder;
/// 
/// assert_eq!(LeftPadder::new("foo\n bar", 0).to_string(), "foo\n bar");
/// assert_eq!(LeftPadder::new("foo\n bar", 1).to_string(), " foo\n  bar");
/// assert_eq!(LeftPadder::new("foo\n bar", 2).to_string(), "  foo\n   bar");
/// assert_eq!(LeftPadder::new("foo\n bar", 4).to_string(), "    foo\n     bar");
/// 
/// 
/// ```
#[derive(Clone, Copy, Debug)]
pub struct LeftPadder<'a> {
    string: &'a str,
    padding: usize,
}

impl<'a> LeftPadder<'a> {
    /// Constructs a LeftPadder
    pub fn new(string: &'a str, padding: usize) -> Self {
        Self { string, padding }
    }
}

impl<'a> fmt::Display for LeftPadder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        use std_::fmt::Write;
        for line in self.string.lines() {
            if !first {
                f.write_char('\n')?;
            }
            const SPACES: &str = "                                ";

            let has_non_whitespace = line.contains(|c: char| !c.is_whitespace());
            let mut pad = if has_non_whitespace { self.padding } else { 0 };
            
            while let Some(next) = pad.checked_sub(SPACES.len()) {
                f.write_str(SPACES)?;
                pad = next;
            }
            f.write_str(&SPACES[..pad])?;

            fmt::Display::fmt(line, f)?;
            first = false;
        }
        Ok(())
    }
}

//---------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_left_pad() {
        let s = "what\n  the\n    hall";
        assert_eq!(s.left_pad(0), s);

        assert_eq!(
            "what\n  the\n    hall".left_pad(4),
            "    what\n      the\n        hall"
        );
        
        assert_eq!("\n\nfoo".left_pad(4), "\n\n    foo");
    }

    #[test]
    fn test_right_char_boundary() {
        let word = "niño";
        assert_eq!(word.right_char_boundary(0), 0);
        assert_eq!(word.right_char_boundary(1), 1);
        assert_eq!(word.right_char_boundary(2), 2);
        // This index is inside of 'ñ'
        assert_eq!(word.right_char_boundary(3), 4);
        assert_eq!(word.right_char_boundary(4), 4);
        assert_eq!(word.right_char_boundary(5), 5);
        assert_eq!(word.right_char_boundary(6), 5);
        assert_eq!(word.right_char_boundary(7), 5);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_char_indices_to() {
        let word = "niño";
        assert_eq!(
            word.char_indices_to(0).map(|(_, c)| c).collect::<String>(),
            ""
        );
        assert_eq!(
            word.char_indices_to(1).map(|(_, c)| c).collect::<String>(),
            "n"
        );
        assert_eq!(
            word.char_indices_to(2).map(|(_, c)| c).collect::<String>(),
            "ni"
        );
        assert_eq!(
            word.char_indices_to(3).map(|(_, c)| c).collect::<String>(),
            "ni"
        );
        assert_eq!(
            word.char_indices_to(4).map(|(_, c)| c).collect::<String>(),
            "niñ"
        );
        assert_eq!(
            word.char_indices_to(5).map(|(_, c)| c).collect::<String>(),
            "niño"
        );
        assert_eq!(
            word.char_indices_to(6).map(|(_, c)| c).collect::<String>(),
            "niño"
        );
        assert_eq!(
            word.char_indices_to(7).map(|(_, c)| c).collect::<String>(),
            "niño"
        );
    }
}
