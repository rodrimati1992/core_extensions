//! Iterator adaptors and constructors.

use std_::{
    cmp::Ordering,
    mem,
};

use crate::prelude::*;

/// Iterator,lazy version of [::std::iter::Once],only evaluating the item when
/// [Iterator::next] is called.
///
/// # Example
///
/// ```
/// use core_extensions::iterators::LazyOnce;
///
/// let mut number=0;
/// assert_eq!(number,0);
/// LazyOnce::new(||{ number+=10; number });
/// assert_eq!(number,0);
/// for i in LazyOnce::new(||{ number+=10; number }) {
///     assert_eq!(i,10);
/// }
/// assert_eq!(number,10);
///
/// ```
#[derive(Copy, Clone)]
pub struct LazyOnce<F> {
    func: Option<F>,
}

impl<F> LazyOnce<F> {
    /// Constructs a LazyOnce.
    pub fn new(f: F) -> Self {
        LazyOnce { func: Some(f) }
    }
}

impl<F, T> Iterator for LazyOnce<F>
where
    F: FnOnce() -> T,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.func.take().map(|f| f())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}

impl<F, T> DoubleEndedIterator for LazyOnce<F>
where
    F: FnOnce() -> T,
{
    fn next_back(&mut self) -> Option<T> {
        self.func.take().map(|f| f())
    }
}

impl<F, T> ExactSizeIterator for LazyOnce<F> where F: FnOnce() -> T {}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct Unreplaced<T> {
    nth: usize,
    current: usize,
    with: T,
}

#[derive(Debug, Clone)]
enum ReplaceNthState<T> {
    Unreplaced(Unreplaced<T>),
    Replaced,
}

/// An Iterator that replaces the nth element with another value.
#[derive(Debug, Clone)]
pub struct ReplaceNth<I>
where
    I: Iterator,
{
    iter: I,
    state: ReplaceNthState<I::Item>,
}

impl<I> ReplaceNth<I>
where
    I: Iterator,
{
    /// Constructs a ReplaceNth
    pub fn new(iter: I, nth: usize, with: I::Item) -> Self {
        Self {
            iter,
            state: ReplaceNthState::Unreplaced(Unreplaced {
                nth,
                current: 0,
                with,
            }),
        }
    }
}

impl<I> Iterator for ReplaceNth<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        use self::ReplaceNthState as RNS;

        let mut ret = self.iter.next()?;

        let replace = match self.state {
            RNS::Unreplaced(ref mut unreplaced) => {
                (unreplaced.nth == unreplaced.current).observe(|v| {
                    if !v {
                        unreplaced.current += 1
                    }
                })
            }
            RNS::Replaced => false,
        };
        if replace {
            if let RNS::Unreplaced(unreplaced) = mem::replace(&mut self.state, RNS::Replaced) {
                ret = unreplaced.with;
            }
        }
        Some(ret)
    }

    fn nth(&mut self, nth: usize) -> Option<I::Item> {
        use self::ReplaceNthState as RNS;

        let mut ret = self.iter.nth(nth)?;

        let mut replace = Ordering::Greater;
        if let RNS::Unreplaced(ref mut unreplaced) = self.state {
            unreplaced.current += nth;
            replace = unreplaced.current.cmp(&unreplaced.nth);
            if replace == Ordering::Less {
                unreplaced.current += 1;
            }
        }
        match replace {
            Ordering::Less => {}
            Ordering::Equal => {
                if let RNS::Unreplaced(unreplaced) = mem::replace(&mut self.state, RNS::Replaced) {
                    ret = unreplaced.with;
                }
            }
            Ordering::Greater => self.state = RNS::Replaced,
        }
        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }
}

#[cfg(test)]
mod test_replace_nth {
    use alloc_::vec::Vec;

    use super::*;
    #[test]
    fn nth_method() {
        let list = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        for i in 0..list.len() {
            let mut iter = ReplaceNth::new(list.iter().cloned(), i, 100);
            println!("iteration:{}", i);
            println!("values:{:?}", iter.clone().collect::<Vec<_>>());
            if i != 0 {
                let j = i - 1;
                assert_eq!(iter.nth(j).unwrap(), list[j])
            }
            assert_eq!(iter.next().unwrap(), 100);
            if i + 1 < list.len() {
                assert_eq!(iter.next().unwrap(), list[i + 1]);
            }
            if i + 2 < list.len() {
                assert_eq!(iter.next().unwrap(), list[i + 2]);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Extension trait for [Iterator] implementors.
pub trait IteratorExt: Iterator {
    /// Collects into an existing collection by extending it.
    ///
    /// # Example
    /// ```
    /// use core_extensions::iterators::IteratorExt;
    ///
    /// let mut list=vec![101,102];
    /// (0..10)
    ///     .filter(|&v| v<5 )
    ///     .map(|v| v*2 )
    ///     .extending(&mut list);
    /// assert_eq!(list,vec![101,102,0,2,4,6,8]);
    ///
    /// ```
    #[inline(always)]
    fn extending<C>(self, extend: &mut C)
    where
        Self: Sized,
        C: Extend<Self::Item>,
    {
        extend.extend(self);
    }

    /// Collects into a pre-allocated collection,returning it by value.
    ///
    /// # Example
    /// ```
    /// use core_extensions::iterators::IteratorExt;
    ///
    /// let list=(0..10)
    ///     .filter(|&v| v<5 )
    ///     .map(|v| v*2 )
    ///     .collect_into(Vec::with_capacity(5));
    ///
    /// assert_eq!(list.capacity(),5);
    /// assert_eq!(list,vec![0,2,4,6,8]);
    ///
    /// ```
    /// # Example
    ///
    /// Reusing an existing collection.
    ///
    /// ```
    /// use core_extensions::iterators::IteratorExt;
    ///
    /// let mut list=Vec::with_capacity(7);
    /// list.push(100);
    /// list.push(101);
    ///
    /// let list=(0..10)
    ///     .filter(|&v| v<5 )
    ///     .map(|v| v*2 )
    ///     .collect_into(list);
    ///
    /// assert_eq!(list.capacity(),7);
    /// assert_eq!(list,vec![100,101,0,2,4,6,8]);
    ///
    /// ```
    #[inline(always)]
    fn collect_into<C>(self, mut extend: C) -> C
    where
        Self: Sized,
        C: Extend<Self::Item>,
    {
        extend.extend(self);
        extend
    }

    /// An Iterator that replaces the nth element with another value.
    ///
    /// # Example
    /// ```
    /// use core_extensions::iterators::ReplaceNth;
    ///
    /// assert_eq!(
    ///     ReplaceNth::new( 0..10,5,1337 ).collect::<Vec<_>>(),
    ///     vec![0,1,2,3,4,1337,6,7,8,9]
    /// );
    ///
    /// let list=vec!["hello","dear","world"];
    /// assert_eq!(
    ///     ReplaceNth::new( list.into_iter(),1,"my" ).collect::<Vec<_>>(),
    ///     vec!["hello","my","world"]
    /// );
    ///
    ///
    /// ```
    #[inline(always)]
    fn replace_nth(self, nth: usize, with: Self::Item) -> ReplaceNth<Self>
    where
        Self: Sized,
    {
        ReplaceNth::new(self, nth, with)
    }
}

impl<I> IteratorExt for I where I: Iterator {}

////////////////////////////////////////////////////////////////////////////////

/// Constructs [Iterator]s using a closure.
///
/// This can construct an Iterator (with IntoIterator::into_iter)
/// multiple times if the closure is Copy.
///
/// # Example
///
/// ```rust
/// use core_extensions::iterators::IterConstructor;
///
/// let list=vec!["hello","world"];
///
/// let constructor=IterConstructor::new(||{
///     list.iter().enumerate().map(|(i,v)| v.repeat(i) )
/// });
///
/// let list_2=vec!["".to_string(),"world".into()];
///
/// assert_eq!(constructor.into_iter().collect::<Vec<_>>() , list_2);
///
/// assert_eq!(constructor.into_iter().collect::<Vec<_>>() , list_2);
///
/// ```
#[derive(Copy, Clone)]
pub struct IterConstructor<F> {
    /// The closure.
    pub f: F,
}

impl<F> IterConstructor<F> {
    /// Constructs an IterConstructor.
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F, I> IntoIterator for IterConstructor<F>
where
    F: FnOnce() -> I,
    I: Iterator,
{
    type Item = I::Item;
    type IntoIter = I;

    fn into_iter(self) -> Self::IntoIter {
        (self.f)()
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Use this macro to create an
/// [IterCloner](iterators/struct.IterCloner.html)
/// from an Iterator value.
///
/// This macro takes an iterator by value,and then allows iterating multiple times with
/// the same iterator,so long as it is Clone.
///
/// Closures can be Copy and/or Clone since Rust 1.26
/// ,this affects iterators since most iterator methods take closures.
///
/// # Example
///
/// This example only runs from Rust 1.26 onwards,
///
#[cfg_attr(not(enable_copy_closures), doc = r#" ```ignore"#)]
#[cfg_attr(enable_copy_closures, doc = " ```")]
///
/// #[macro_use]
/// extern crate core_extensions;
///
/// # fn main(){
///
/// let list=vec!["this","is","not","really","great"];
/// let lengths=vec![4,2,3,6,5];
/// iter_cloner!(let iter=list.iter().map(|v|v.len()));
/// for _ in 0..2{
///     assert_eq!(iter.into_iter().collect::<Vec<_>>(),lengths);
/// }
/// # }
///
/// ```
#[macro_export]
macro_rules! iter_cloner {
    (let $ident:ident = $expr:expr) => {
        let $ident = $expr;
        let $ident = $crate::iterators::IterCloner::new(&$ident);
    };
}

/// Constructs an [Iterator] by cloning the one it references,if the Iterator is Clone.
///
/// Constructs an Iterator inside [IntoIterator::into_iter]
/// by cloning the one it references.
///
/// To construct this from an iterator value use [this macro](../macro.iter_cloner.html) ,
///
/// Closures can be Copy and/or Clone since Rust 1.26
/// ,this affects iterators since most iterator methods take closures.
///
/// # Example
///
/// This example only runs from Rust 1.26 onwards,
///
#[cfg_attr(not(enable_copy_closures), doc = r#" ```ignore"#)]
#[cfg_attr(enable_copy_closures, doc = " ```")]
///
/// use core_extensions::iterators::IterCloner;
///
/// let list=vec!["hello","awesome","world"];
/// let iter=list.iter().map(|v|v.len()).filter(|&v| v<6 );
/// {
///     let iter_clone=IterCloner::new(&iter);
///    
///     for _ in 0..2{
///         assert_eq!(iter_clone.into_iter().collect::<Vec<_>>(),vec![5,5]);
///     }
/// }
/// assert_eq!(iter.into_iter().collect::<Vec<_>>(),vec![5,5]);
///
/// ```
pub struct IterCloner<'a, I: 'a> {
    /// The iterator being cloned.
    pub iter: &'a I,
}

impl<'a, I> Copy for IterCloner<'a, I> {}
impl<'a, I> Clone for IterCloner<'a, I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, I: 'a> IterCloner<'a, I>
where
    I: Iterator + Clone,
{
    /// Constructs an IterCloner.
    pub fn new(iter: &'a I) -> Self {
        Self { iter }
    }
}

impl<'a, I: 'a> IntoIterator for IterCloner<'a, I>
where
    I: Iterator + Clone,
{
    type Item = I::Item;
    type IntoIter = I;

    fn into_iter(self) -> Self::IntoIter {
        self.iter.clone()
    }
}
