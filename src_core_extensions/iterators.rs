//! Iterator adaptors and constructors.

use std_::{
    cmp::Ordering,
    iter::{Product, Sum},
    mem,
};


/// A version of [`std::iter::OnceWith`] usable in Rust 1.41.0.
///
/// # Example
///
/// ```
/// use core_extensions::iterators::LazyOnce;
///
/// let mut number = 0;
///
/// // The closure here is never ran.
/// LazyOnce::new(||{ number+=10; number });
///
/// assert_eq!(number, 0);
///
/// for i in LazyOnce::new(||{ number+=10; number }) {
///     assert_eq!(i, 10);
/// }
///
/// assert_eq!(number, 10);
///
/// ```
///
/// [`std::iter::OnceWith`]: https://doc.rust-lang.org/std/iter/struct.OnceWith.html
/// [`Iterator::next`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
///
#[derive(Debug, Copy, Clone)]
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

/// An Iterator that replaces the `nth` element with another value.
///
/// # Example
///
/// ```rust
/// use core_extensions::iterators::ReplaceNth;
///
/// // This iterator replaces the 4th element with `100`
/// let list = ReplaceNth::new(0..=6, 4, 100).collect::<Vec<_>>();
///
/// assert_eq!(list, vec![0, 1, 2, 3, 100, 5, 6]);
///
/// ```
///
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
    /// Constructs a `ReplaceNth`.
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
                let x = unreplaced.nth == unreplaced.current;
                if !x {
                    unreplaced.current += 1
                }
                x
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
#[cfg(feature = "alloc")]
mod test_replace_nth {
    use alloc::vec::Vec;

    use super::*;
    #[test]
    fn nth_method() {
        let list = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        for i in 0..list.len() {
            let mut iter = ReplaceNth::new(list.iter().cloned(), i, 100);
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

/// Extension trait for [`std::iter::Iterator`] implementors.
///
/// [`std::iter::Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
pub trait IteratorExt: Iterator {
    /// Collects into an existing collection by extending it.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::iterators::IteratorExt;
    ///
    /// let mut list = vec![101, 102];
    ///
    /// (0..10)
    ///     .filter(|&v| v<5 )
    ///     .map(|v| v*2 )
    ///     .extending(&mut list);
    ///
    /// assert_eq!(list, vec![101, 102, 0, 2, 4, 6, 8]);
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
    ///
    /// ```
    /// use core_extensions::iterators::IteratorExt;
    ///
    /// let list = (0..10)
    ///     .filter(|&v| v<5 )
    ///     .map(|v| v*2 )
    ///     .collect_into(Vec::with_capacity(5));
    ///
    /// assert_eq!(list.capacity(), 5);
    /// assert_eq!(list, vec![0, 2, 4, 6, 8]);
    ///
    /// ```
    /// # Example
    ///
    /// Reusing an existing collection.
    ///
    /// ```
    /// use core_extensions::iterators::IteratorExt;
    ///
    /// let mut list = Vec::with_capacity(7);
    /// list.push(100);
    /// list.push(101);
    ///
    /// let list = (0..10)
    ///     .filter(|&v| v<5 )
    ///     .map(|v| v*2 )
    ///     .collect_into(list);
    ///
    /// assert_eq!(list.capacity(),7);
    /// assert_eq!(list, vec![100, 101, 0, 2, 4, 6, 8]);
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
    /// use core_extensions::iterators::IteratorExt;
    ///
    /// assert_eq!(
    ///     (0..=9).replace_nth(5, 1337).collect::<Vec<_>>(),
    ///     vec![0, 1, 2, 3, 4, 1337, 6, 7, 8, 9]
    /// );
    ///
    /// let list = vec!["hello", "dear", "world"];
    ///
    /// assert_eq!(
    ///     list.into_iter().replace_nth(1, "my").collect::<Vec<_>>(),
    ///     vec!["hello", "my", "world"]
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

    /// Sums the items of the iterator, into the item's type.
    ///
    /// This like the [`Iterator::sum`] method, with better type inference,
    /// since with the [`Iterator::sum`] method you must specify its return type.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use core_extensions::iterators::IteratorExt;
    ///
    /// assert_eq!((1..=4).sum_same(), 10);
    /// 
    /// let arr = [3, 7, 11, 29];
    /// assert_eq!(arr.iter().copied().sum_same(), 50);
    /// 
    /// ```
    ///  
    /// [`Iterator::sum`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum 
    #[inline]
    fn sum_same(self) -> Self::Item
    where
        Self: Sized,
        Self::Item: Sum,
    {
        <Self::Item as Sum<Self::Item>>::sum(self)
    }

    /// Multiplies the items of the iterator, into the item's type.
    ///
    /// This like the [`Iterator::product`] method, with better type inference,
    /// since with the [`Iterator::product`] method you must specify its return type.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use core_extensions::iterators::IteratorExt;
    ///
    /// assert_eq!((1..=4).product_same(), 24);
    /// 
    /// let arr = [3, 4, 6];
    /// assert_eq!(arr.iter().copied().product_same(), 72);
    /// 
    /// ```
    ///  
    /// [`Iterator::product`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.product
    #[inline]
    fn product_same(self) -> Self::Item
    where
        Self: Sized,
        Self::Item: Product,
    {
        <Self::Item as Product<Self::Item>>::product(self)
    }
}

impl<I> IteratorExt for I where I: ?Sized + Iterator {}

////////////////////////////////////////////////////////////////////////////////

/// Uses a closure to construct `Iterator`s.
///
/// This can turn this into an `Iterator` (with `IntoIterator::into_iter`)
/// multiple times if the closure is `Copy`.
///
/// # Example
///
/// ```rust
/// use core_extensions::iterators::IterConstructor;
///
/// let list = vec!["hello", "world"];
///
/// let constructor = IterConstructor(||{
///     list.iter().enumerate().map(|(i,v)| v.repeat(i) )
/// });
///
/// for _ in 0..2 {
///     assert_eq!(
///         constructor.into_iter().collect::<Vec<_>>(),
///         ["".to_string(), "world".to_string()],
///     );
/// }
///
/// ```
#[derive(Debug, Copy, Clone)]
pub struct IterConstructor<F> (pub F);

impl<F, I> IntoIterator for IterConstructor<F>
where
    F: FnOnce() -> I,
    I: IntoIterator,
{
    type Item = I::Item;
    type IntoIter = I::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (self.0)().into_iter()
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Use this macro to create an
/// [`IterCloner`](./iterators/struct.IterCloner.html)
/// from an [`IntoIterator`] (this includes all [`Iterator`]s).
///
/// The resulting variable clones the iterator (that `$expr` was converted into)
/// every time that you call `.into_iter()` or iterate over it with a `for` loop.
///
/// # Example
///
/// ### Mapping
///
/// ```rust
/// use core_extensions::iter_cloner;
/// 
/// let list = vec!["this", "is", "not", "really", "great"];
///
/// let lengths = vec![4, 2, 3, 6, 5];
///
/// iter_cloner!(let iter = list.iter().map(|v|v.len()));
///
/// assert_eq!(iter.into_iter().collect::<Vec<_>>(), lengths);
/// assert_eq!(iter.into_iter().collect::<Vec<_>>(), lengths);
/// assert_eq!(iter.into_iter().collect::<Vec<_>>(), lengths);
///
/// ```
///
/// ### Vector
///
/// ```rust
/// use core_extensions::iter_cloner;
///
/// iter_cloner!(let iter = vec![0, 1, 2, 3]);
///
/// assert_eq!(iter.into_iter().collect::<Vec<_>>(), [0, 1, 2, 3]);
/// assert_eq!(iter.into_iter().collect::<Vec<_>>(), [0, 1, 2, 3]);
/// assert_eq!(iter.into_iter().collect::<Vec<_>>(), [0, 1, 2, 3]);
///
/// ```
///
/// [`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
/// [`Iterator`]:  https://doc.rust-lang.org/std/iter/trait.Iterator.html
#[cfg(feature = "iterators")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iterators")))]
#[macro_export]
macro_rules! iter_cloner {
    (let $ident:ident = $expr:expr) => {
        let $ident = $crate::std_::iter::IntoIterator::into_iter($expr);
        let $ident = $crate::iterators::IterCloner(&$ident);
    };
}

/// Implements [`IntoIterator::into_iter`] by cloning the iterator it references.
///
/// You can also use the [`iter_cloner`](../macro.iter_cloner.html) macro to
/// construct this,
///
/// # Example
///
/// ```
///
/// use core_extensions::iterators::IterCloner;
///
/// let list = vec!["hello", "awesome", "world"];
///
/// let iter = list.iter().map(|v|v.len()).filter(|&v| v<6 );
///
/// let iter_clone = IterCloner(&iter);
///    
/// for _ in 0..2{
///     assert_eq!(iter_clone.into_iter().collect::<Vec<_>>(), vec![5, 5]);
/// }
///
/// ```
///
/// [`IntoIterator::into_iter`]: 
/// https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#tymethod.into_iter
///
#[derive(Debug)]
pub struct IterCloner<'a, I: 'a> (pub &'a I);

impl<'a, I> Copy for IterCloner<'a, I> {}
impl<'a, I> Clone for IterCloner<'a, I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, I: 'a> IntoIterator for IterCloner<'a, I>
where
    I: Iterator + Clone,
{
    type Item = I::Item;
    type IntoIter = I;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone()
    }
}
