//! Extension trait for string types.

use std_::borrow::Borrow;
use std_::cmp;
use std_::fmt;
use std_::str::CharIndices;

mod iterators;

pub use self::iterators::{CharIndicesFrom, KeyStr, RSplitWhile, SplitWhile};

/// Extension trait for strings (any type that borrows as [str]).
pub trait StringExt: Borrow<str> {
    /// Returns the previous character boundary,stopping at 0.
    ///
    /// if index>self.len() ,returns self.len()
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    ///
    /// let word="niño";
    /// assert_eq!(word.previous_char_boundary(0),0);
    /// assert_eq!(word.previous_char_boundary(1),0);
    /// assert_eq!(word.previous_char_boundary(2),1);
    /// // This index is inside of 'ñ'
    /// assert_eq!(word.previous_char_boundary(3),2);
    /// assert_eq!(word.previous_char_boundary(4),2);
    /// assert_eq!(word.previous_char_boundary(5),4);
    /// assert_eq!(word.previous_char_boundary(6),5);
    /// assert_eq!(word.previous_char_boundary(7),5);
    /// assert_eq!(word.previous_char_boundary(8),5);
    /// ```
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
    /// if index>self.len() ,returns self.len()
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    ///
    /// let word="niño";
    /// assert_eq!(word.next_char_boundary(0),1);
    /// assert_eq!(word.next_char_boundary(1),2);
    /// assert_eq!(word.next_char_boundary(2),4);
    /// // This index is inside of 'ñ'
    /// assert_eq!(word.next_char_boundary(3),4);
    /// assert_eq!(word.next_char_boundary(4),5);
    /// assert_eq!(word.next_char_boundary(5),5);
    /// assert_eq!(word.next_char_boundary(6),5);
    /// ```
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
    /// If index is at a character boundary it returns index,
    /// otherwise it  returns the previous character boundary.
    ///
    /// if index>self.len() ,returns self.len()
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::strings::StringExt;
    ///
    /// let word="niño";
    /// assert_eq!(word.left_char_boundary(0),0);
    /// assert_eq!(word.left_char_boundary(1),1);
    /// assert_eq!(word.left_char_boundary(2),2);
    /// // This index is inside of 'ñ'
    /// assert_eq!(word.left_char_boundary(3),2);
    /// assert_eq!(word.left_char_boundary(4),4);
    /// assert_eq!(word.left_char_boundary(5),5);
    /// assert_eq!(word.left_char_boundary(6),5);
    /// assert_eq!(word.left_char_boundary(7),5);
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
    /// If index is at a character boundary it returns index,
    /// otherwise it  returns the next character boundary.
    ///
    /// if index>self.len() ,returns self.len()
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::strings::StringExt;
    ///
    /// let word="niño";
    /// assert_eq!(word.right_char_boundary(0),0);
    /// assert_eq!(word.right_char_boundary(1),1);
    /// assert_eq!(word.right_char_boundary(2),2);
    /// // This index is inside of 'ñ'
    /// assert_eq!(word.right_char_boundary(3),4);
    /// assert_eq!(word.right_char_boundary(4),4);
    /// assert_eq!(word.right_char_boundary(5),5);
    /// assert_eq!(word.right_char_boundary(6),5);
    /// assert_eq!(word.right_char_boundary(7),5);
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
    /// Returns the offset of a substring inside of the `other` str.
    ///
    /// Returns None if `self` is not a substring of `other`.
    ///
    /// This operation is constant time.Purely done using pointer comparisons.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::strings::StringExt;
    /// let text="What is this?";
    /// assert_eq!(
    ///     vec![(0,"What"),(5,"is"),(8,"this?")],
    ///     text.split_whitespace()
    ///         .filter_map(|w|{
    ///             w.get_offset_inside_of(text)
    ///                 .map(|x| (x,w) )
    ///         })
    ///         .collect::<Vec<_>>()
    /// );
    ///
    /// ```
    fn get_offset_inside_of(&self, parent: &str) -> Option<usize> {
        let self_addr = self.borrow().as_ptr() as usize;
        let offset = self_addr.wrapping_sub(parent.as_ptr() as usize);
        if offset < parent.len() {
            Some(offset)
        } else {
            None
        }
    }
    /// Returns the offset of a substring inside of the `other` str.
    ///
    /// Returns None if `self` is not a substring of `other`.
    ///
    /// This operation is constant time.Purely done using pointer comparisons.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::strings::StringExt;
    /// let text="What is this?";
    /// assert_eq!(
    ///     vec![(0,"What"),(5,"is"),(8,"this?")],
    ///     text.split_whitespace()
    ///         .map(|w| (w.offset_inside_of(text),w) )
    ///         .collect::<Vec<_>>()
    /// );
    ///
    /// ```
    fn offset_inside_of(&self, parent: &str) -> usize {
        let self_addr = self.borrow().as_ptr() as usize;
        let offset = self_addr.wrapping_sub(parent.as_ptr() as usize);
        cmp::min(parent.len(), offset)
    }
    /// Returns whether `self` is a substring (in memory) of `parent`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::strings::StringExt;
    /// use core_extensions::SliceExt;
    /// let text="What is this?";
    /// for i in 0..text.len() {
    ///     let sub=text.slice_lossy(i..text.len(),());
    ///     assert!(sub.is_substring_of(text));
    /// }
    ///
    /// assert!( !"What".to_string().is_substring_of(text));
    /// assert!( !"is"  .to_string().is_substring_of(text));
    /// assert!( !"this".to_string().is_substring_of(text));
    /// assert!( !"".is_substring_of(text));
    /// ```
    fn is_substring_of(&self, parent: &str) -> bool {
        let addr_self = self.borrow().as_ptr() as usize;
        let addr_other = parent.as_ptr() as usize;
        addr_self.wrapping_sub(addr_other) < parent.len()
    }
    /// Returns an iterator over substrings whose chars were mapped to the same key by mapper.
    ///
    /// Returns an impl Iterator\<Item=[KeyStr](./struct.KeyStr.html)\<T\>>
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    ///
    /// fn func<U:Eq+Clone,F>(s:&str,f:F)->Vec<(U,&str)>
    /// where F:FnMut(char)->U
    /// {
    ///     s.split_while(f).map(|v| (v.key,v.str) ).collect()
    /// }
    ///
    /// assert_eq!(
    ///     "Hello, world!"
    ///         .split_while(|c|c.is_alphabetic())
    ///         .filter(|v| v.key==true )
    ///         .map(|v| v.str )
    ///         .collect::<Vec<_>>(),
    ///     vec!["Hello","world"]
    /// );
    /// assert_eq!(
    ///     vec![(true,"Hello"),(false,", "),(true,"world"),(false,"!")] ,
    ///     func("Hello, world!",|c| c.is_alphanumeric())
    /// );
    /// assert_eq!(
    ///     vec![('a',"aaa"),('b',"bbb"),('c',"ccc")] ,
    ///     func("aaabbbccc",|c|c)
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
    /// A variation of split_while that iterates
    /// from the right(the order of substrings is reversed).
    ///
    /// Returns an impl Iterator\<Item=[KeyStr](./struct.KeyStr.html)\<T\>>
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    ///
    /// fn func<U:Eq+Clone,F>(s:&str,f:F)->Vec<(U,&str)>
    /// where F:FnMut(char)->U
    /// {
    ///     s.rsplit_while(f).map(|v| (v.key,v.str) ).collect()
    /// }
    ///
    /// assert_eq!(
    ///     "Hello, world!"
    ///         .rsplit_while(|c|c.is_alphabetic())
    ///         .filter(|v| v.key==true )
    ///         .map(|v| v.str )
    ///         .collect::<Vec<_>>(),
    ///     vec!["world","Hello"]
    /// );
    /// assert_eq!(
    ///     vec![(false,"!"),(true,"world"),(false,", "),(true,"Hello")] ,
    ///     func("Hello, world!",|c| c.is_alphanumeric() )
    /// );
    /// assert_eq!(vec![('c',"ccc"),('b',"bbb"),('a',"aaa")],func("aaabbbccc",|c|c));
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
    /// returns the position of the nth character
    ///
    /// if there is no nth character it returns None
    ///
    /// This operation takes O(n) time,where n is self.len().
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    /// let word="niño";
    ///
    /// assert_eq!(word.get_nth_char_index(0),Some(0));
    /// assert_eq!(word.get_nth_char_index(1),Some(1));
    /// assert_eq!(word.get_nth_char_index(2),Some(2));
    /// assert_eq!(word.get_nth_char_index(3),Some(4));
    /// assert_eq!(word.get_nth_char_index(4),None);
    /// assert_eq!(word.get_nth_char_index(5),None);
    /// assert_eq!(word.get_nth_char_index(6),None);
    /// ```
    fn get_nth_char_index(&self, n: usize) -> Option<usize> {
        self.borrow().char_indices().nth(n).map(|(i, _)| i)
    }

    /// returns the position of the nth character
    ///
    /// if there is no nth character it returns self.len()
    ///
    /// This operation takes O(n) time,where n is self.len().
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    /// let word="niño";
    ///
    /// assert_eq!(word.nth_char_index(0),0);
    /// assert_eq!(word.nth_char_index(1),1);
    /// assert_eq!(word.nth_char_index(2),2);
    /// assert_eq!(word.nth_char_index(3),4);
    /// assert_eq!(word.nth_char_index(4),5);
    /// assert_eq!(word.nth_char_index(5),5);
    /// assert_eq!(word.nth_char_index(6),5);
    /// ```
    fn nth_char_index(&self, n: usize) -> usize {
        let this = self.borrow();
        this.borrow()
            .char_indices()
            .nth(n)
            .map_or(this.len(), |(i, _)| i)
    }

    /// Returns the nth character in the str.
    ///
    /// This operation takes O(n) time,where n is self.len().
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    /// let word="niño";
    ///
    /// assert_eq!(word.nth_char(0),Some('n'));
    /// assert_eq!(word.nth_char(1),Some('i'));
    /// assert_eq!(word.nth_char(2),Some('ñ'));
    /// assert_eq!(word.nth_char(3),Some('o'));
    /// assert_eq!(word.nth_char(4),None);
    /// assert_eq!(word.nth_char(5),None);
    /// assert_eq!(word.nth_char(6),None);
    /// ```
    fn nth_char(&self, n: usize) -> Option<char> {
        self.borrow().chars().nth(n)
    }

    /// returns a string containing the first `n` chars.
    /// if n > self.chars().count() it returns the entire string
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    /// let word="niño";
    ///
    /// assert_eq!(word.first_chars(0),"");
    /// assert_eq!(word.first_chars(1),"n");
    /// assert_eq!(word.first_chars(2),"ni");
    /// assert_eq!(word.first_chars(3),"niñ");
    /// assert_eq!(word.first_chars(4),"niño");
    /// assert_eq!(word.first_chars(5),"niño");
    /// assert_eq!(word.first_chars(6),"niño");
    /// ```
    fn first_chars(&self, n: usize) -> &str {
        let this = self.borrow();
        &this[..this.nth_char_index(n)]
    }
    /// returns a string containing the last `n` chars
    /// if n > self.chars().count() it returns the entire string
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    /// let word="niño";
    ///
    /// assert_eq!(word.last_chars(0),"");
    /// assert_eq!(word.last_chars(1),"o");
    /// assert_eq!(word.last_chars(2),"ño");
    /// assert_eq!(word.last_chars(3),"iño");
    /// assert_eq!(word.last_chars(4),"niño");
    /// assert_eq!(word.last_chars(5),"niño");
    /// assert_eq!(word.last_chars(6),"niño");
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
    /// returns the string from the `n`th character
    /// if n > self.chars().count() it returns an empty string
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    /// let word="niño";
    ///
    /// assert_eq!(word.from_nth_char(0),"niño");
    /// assert_eq!(word.from_nth_char(1),"iño");
    /// assert_eq!(word.from_nth_char(2),"ño");
    /// assert_eq!(word.from_nth_char(3),"o");
    /// assert_eq!(word.from_nth_char(4),"");
    /// assert_eq!(word.from_nth_char(5),"");
    /// assert_eq!(word.from_nth_char(6),"");
    /// ```
    fn from_nth_char(&self, n: usize) -> &str {
        let this = self.borrow();
        &this[this.nth_char_index(n)..]
    }

    /// returns the length of the text in utf16
    ///
    /// # warning
    ///
    /// This is calculated every time the function is called.
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    ///
    /// assert_eq!("niño".calc_len_utf16(),4);
    /// assert_eq!("ññññ".calc_len_utf16(),4);
    ///
    /// ```
    fn calc_len_utf16(&self) -> usize {
        self.borrow()
            .chars()
            .fold(0, |accum, c| accum + c.len_utf16())
    }
    /// Returns the character at `at_byte` if it's an index inside of the str.
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    /// let word="niño";
    ///
    /// assert_eq!(word.get_char_at(0),Some('n'));
    /// assert_eq!(word.get_char_at(1),Some('i'));
    /// assert_eq!(word.get_char_at(2),Some('ñ'));
    /// assert_eq!(word.get_char_at(3),Some('ñ'));
    /// assert_eq!(word.get_char_at(4),Some('o'));
    /// assert_eq!(word.get_char_at(5),None);
    /// assert_eq!(word.get_char_at(6),None);
    /// ```
    fn get_char_at(&self, at_byte: usize) -> Option<char> {
        let this = self.borrow();
        if at_byte >= this.len() {
            return None;
        }
        let start = this.left_char_boundary(at_byte);
        this[start..].chars().nth(0)
    }

    /// Returns an iterator over (index,char) pairs up to (but not including) `to`.
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    /// let word="niño";
    ///
    /// fn func(s:&str,i:usize)->(Vec<usize>,String){
    ///     (
    ///         s.char_indices_to(i).map(|(i,_)|i).collect(),
    ///         s.char_indices_to(i).map(|(_,c)|c).collect(),
    ///     )
    /// }
    /// assert_eq!(func(word,0),(vec![]       ,""    .to_string()));
    /// assert_eq!(func(word,1),(vec![0]      ,"n"   .to_string()));
    /// assert_eq!(func(word,2),(vec![0,1]    ,"ni"  .to_string()));
    /// assert_eq!(func(word,3),(vec![0,1,2]  ,"niñ" .to_string()));
    /// assert_eq!(func(word,4),(vec![0,1,2]  ,"niñ" .to_string()));
    /// assert_eq!(func(word,5),(vec![0,1,2,4],"niño".to_string()));
    /// assert_eq!(func(word,6),(vec![0,1,2,4],"niño".to_string()));
    /// assert_eq!(func(word,7),(vec![0,1,2,4],"niño".to_string()));
    /// ```
    fn char_indices_to(&self, to: usize) -> CharIndices {
        let this = self.borrow();
        let to = this.right_char_boundary(to);
        this[..to].char_indices()
    }

    /// Returns an iterator over (index,char) pairs from `from`.
    ///
    /// # Example
    /// ```
    /// use core_extensions::strings::StringExt;
    /// let word="niño";
    ///
    /// fn func(s:&str,i:usize)->(Vec<usize>,String){
    ///     (
    ///         s.char_indices_from(i).map(|(i,_)|i).collect(),
    ///         s.char_indices_from(i).map(|(_,c)|c).collect(),
    ///     )
    /// }
    /// assert_eq!(func(word,0),(vec![0,1,2,4],"niño".to_string()));
    /// assert_eq!(func(word,1),(vec![1,2,4]  ,"iño" .to_string()));
    /// assert_eq!(func(word,2),(vec![2,4]    ,"ño"  .to_string()));
    /// assert_eq!(func(word,3),(vec![2,4]    ,"ño"  .to_string()));
    /// assert_eq!(func(word,4),(vec![4]      ,"o"   .to_string()));
    /// assert_eq!(func(word,5),(vec![]       ,""    .to_string()));
    /// assert_eq!(func(word,6),(vec![]       ,""    .to_string()));
    /// assert_eq!(func(word,7),(vec![]       ,""    .to_string()));
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
    /// use core_extensions::strings::StringExt;
    /// assert_eq!(
    ///     "what\n  the\n    hell".left_pad(4),
    ///     "    what\n      the\n        hell");
    ///
    /// ```
    ///
    #[cfg(feature = "std")]
    fn left_pad(&self, how_much: usize) -> String {
        self.left_padder(how_much).to_string()
    }
    /// Pads the string on the left with `how_much` additional
    /// spaces in the Display impl of LeftPadder.
    ///
    /// Use this to avoid allocating an extra string.
    fn left_padder<'a>(&'a self, how_much: usize) -> LeftPadder<'a> {
        LeftPadder::new(self.borrow(), how_much)
    }
    /// The indentation of the first line.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::strings::StringExt;
    /// assert_eq!("".line_indentation(),0);
    /// assert_eq!("    ".line_indentation(),4);
    /// assert_eq!("    \n      word".line_indentation(),4);
    /// assert_eq!("    \nword".line_indentation(),4);
    ///
    /// ```
    ///
    #[cfg(any(core_str_methods, feature = "std"))]
    fn line_indentation(&self) -> usize {
        let this = self.borrow().lines().next().unwrap_or("");
        this.len() - this.trim_start_().len()
    }

    /// The minimum indentation of the string.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::strings::StringExt;
    /// assert_eq!("".min_indentation(),0);
    /// assert_eq!("    ".min_indentation(),4);
    /// assert_eq!("    \n      word".min_indentation(),4);
    /// assert_eq!("    \nword".min_indentation(),0);
    ///
    /// ```
    ///
    #[cfg(any(core_str_methods, feature = "std"))]
    fn min_indentation(&self) -> usize {
        self.borrow()
            .lines()
            .map(|v| v.line_indentation())
            .min()
            .unwrap_or(0)
    }
    /// The maximum indentation of the string.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::strings::StringExt;
    /// assert_eq!("".max_indentation(),0);
    /// assert_eq!("    ".max_indentation(),4);
    /// assert_eq!("    \n      word".max_indentation(),6);
    /// assert_eq!("    \nword".max_indentation(),4);
    ///
    /// ```
    ///
    #[cfg(any(core_str_methods, feature = "std"))]
    fn max_indentation(&self) -> usize {
        self.borrow()
            .lines()
            .map(|v| v.line_indentation())
            .max()
            .unwrap_or(0)
    }
}

impl<T: ?Sized> StringExt for T where T: Borrow<str> {}

//----------------------------------------------------------------------------------------


pub(crate) trait StrMethods: Borrow<str> {
    #[cfg(any(core_str_methods, feature = "std"))]
    fn trim_start_(&self)->&str{
        let this=self.borrow();
        #[cfg(not(trim_left_right_method_deprecation))]
        {
            this.trim_left()
        }
        #[cfg(trim_left_right_method_deprecation)]
        {
            this.trim_start()
        }
    }
    #[cfg(any(core_str_methods, feature = "std"))]
    fn trim_end_(&self)->&str{
        let this=self.borrow();
        #[cfg(not(trim_left_right_method_deprecation))]
        {
            this.trim_right()
        }
        #[cfg(trim_left_right_method_deprecation)]
        {
            this.trim_end()
        }
    }
}

impl<T> StrMethods for T 
where T:Borrow<str>+?Sized
{}


//----------------------------------------------------------------------------------------

/// Add `padding` padding to `string` in the Display impl.
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
            for _ in 0..self.padding {
                f.write_char(' ')?;
            }
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
    fn test_left_pad() {
        let s = "what\n  the\n    hell";
        assert_eq!(s.left_pad(0), s);

        assert_eq!(
            "what\n  the\n    hell".left_pad(4),
            "    what\n      the\n        hell"
        );
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
    fn text_get_offset_inside_of() {
        let text = "What is this?";
        assert_eq!(
            vec![(0, "What"), (5, "is"), (8, "this?")],
            text.split_whitespace()
                .filter_map(|w| Some((try_opt!(w.get_offset_inside_of(text)), w)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
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
            "niñ"
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
