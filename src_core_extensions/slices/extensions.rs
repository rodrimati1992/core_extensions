//! Extension trait for \[T\] and [str].
//!
//!
//!

// use ranges::RangeBounds;
use super::{BiasDirection, SliceBias,SplitSliceWhile,RSplitSliceWhile};

use std_::borrow::Borrow;
use std_::cmp;
use std_::mem;
use std_::ops::Range;


/// Extension trait for `[T]`.
pub trait ValSliceExt: SliceExt + Borrow<[<Self as SliceExt>::Elem]> {
    /// Returns an iterator over subslices whose elements were mapped 
    /// to the same value by `mapper`.
    /// 
    /// The returned type implements 
    /// `DoubleEndedIterator<Item =`[`KeySlice`](./struct.KeySlice.html)`<Self::Elem, U>>`.
    /// 
    /// # Example
    /// 
    /// ```
    /// use core_extensions::ValSliceExt;
    /// use core_extensions::slices::KeySlice;
    /// 
    /// {
    ///     let list = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    ///     assert_eq!(
    ///         list.split_while(|x| x/4).collect::<Vec<_>>(),
    ///         vec![
    ///             KeySlice{key: 0, slice: &[0, 1, 2, 3]},
    ///             KeySlice{key: 1, slice: &[4, 5, 6, 7]},
    ///             KeySlice{key: 2, slice: &[8]},
    ///         ]
    ///     );
    /// }
    /// 
    /// {
    ///     let list = [0, 4, 1, 5, 9, 8, 7];
    /// 
    ///     assert_eq!(
    ///         list.split_while(|x| x%4).collect::<Vec<_>>(),
    ///         vec![
    ///             KeySlice{key: 0, slice: &[0, 4]},
    ///             KeySlice{key: 1, slice: &[1, 5, 9]},
    ///             KeySlice{key: 0, slice: &[8]},
    ///             KeySlice{key: 3, slice: &[7]},
    ///         ]
    ///     );
    /// }
    /// 
    /// 
    /// ```
    /// 
    fn split_while<'a, P, U>(&'a self, mut mapper: P) -> SplitSliceWhile<'a, Self::Elem, P, U>
    where
        P: FnMut(&'a Self::Elem) -> U,
        U: Eq + Clone,
    {
        let this:&'a [Self::Elem] = self.borrow();
        SplitSliceWhile {
            last_left: this.first().map(&mut mapper),
            last_right: this.last().map(&mut mapper),
            mapper,
            s: this,
        }
    }

    /// A variation of [`split_while`](#method.split_while) that iterates
    /// from the right(the order of subslices is reversed).
    /// 
    /// The returned type implements 
    /// `DoubleEndedIterator<Item =`[`KeySlice`](./struct.KeySlice.html)`<Self::Elem, U>>`.
    /// 
    /// # Example
    /// 
    /// ```
    /// use core_extensions::ValSliceExt;
    /// use core_extensions::slices::KeySlice;
    /// 
    /// {
    ///     let list = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    /// 
    ///     assert_eq!(
    ///         list.rsplit_while(|x| x/4 ).collect::<Vec<_>>(),
    ///         vec![
    ///             KeySlice{key: 2, slice: &[8]},
    ///             KeySlice{key: 1, slice: &[4, 5, 6, 7]},
    ///             KeySlice{key: 0, slice: &[0, 1, 2, 3]},
    ///         ]
    ///     );
    /// }
    /// 
    /// {
    ///     let list = [0, 4, 1, 5, 9, 8, 7];
    /// 
    ///     assert_eq!(
    ///         list.rsplit_while(|x| x%4 ).collect::<Vec<_>>(),
    ///         vec![
    ///             KeySlice{key: 3, slice: &[7]},
    ///             KeySlice{key: 0, slice: &[8]},
    ///             KeySlice{key: 1, slice: &[1, 5, 9]},
    ///             KeySlice{key: 0, slice: &[0, 4]},
    ///         ]
    ///     );
    /// }
    /// 
    /// 
    /// 
    /// ```
    /// 
    fn rsplit_while<'a, P, U>(&'a self, mut mapper: P) -> RSplitSliceWhile<'a, Self::Elem, P, U>
    where
        P: FnMut(&'a Self::Elem) -> U,
        U: Eq + Clone,
    {
        let this: &'a [Self::Elem] = self.borrow();
        RSplitSliceWhile {
            last_left: this.first().map(&mut mapper),
            last_right: this.last().map(&mut mapper),
            mapper,
            s: this,
        }
    }
}

impl<This> ValSliceExt for This
where
    This: ?Sized + SliceExt,
    This: Borrow<[Self::Elem]>,
{}



/// Extension trait for `[T]` and `str`.
pub trait SliceExt {
    /// The type of a slice element.
    type Elem;

    /// Checks whether self fully contains the `other` slice in memory.
    ///
    /// If `other` is a zero-length slice it is not contained inside `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SliceExt;
    ///
    /// let list = vec![0, 1, 2, 3, 4, 5];
    /// 
    /// let another = [6, 7, 8];
    /// 
    /// assert!(list.contains_slice(&list[..1]));
    /// assert!(list.contains_slice(&list[3..]));
    ///
    /// // Empty slices aren't considered contained by any other slice
    /// assert!(!list.contains_slice(&list[..0]));
    /// assert!(!list.contains_slice(&another[..0]));
    ///
    /// assert!(!list.contains_slice(&another));
    ///
    /// ```
    fn contains_slice(&self, other: &Self) -> bool;

    /// Checks whether `self` is exactly the `other` slice in memory.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SliceExt;
    ///
    /// let list = [0, 1, 2, 3, 4, 5];
    /// let slice_0 = &list[..0];
    /// let slice_1 = &list[..];
    ///
    /// let other = [0, 1, 2, 3, 4, 5];
    ///
    /// assert!( slice_0.is_slice(slice_0));
    /// assert!( slice_0.is_slice(&list[..0]));
    /// assert!(!slice_0.is_slice(slice_1));
    /// assert!(!slice_0.is_slice(&[]));
    ///
    /// assert!(!list.is_slice(&other));
    ///
    /// ```
    fn is_slice(&self, other: &Self) -> bool;

    /// Returns the index at which `other` starts.
    ///
    /// If `other` is not inside `self`, this returns `self.len()`
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SliceExt;
    ///
    /// let list = vec![0, 1, 2, 3, 4, 5];
    /// 
    /// let other = [0, 1, 2, 3];
    ///
    /// assert_eq!(list.offset_of_slice(&list[..0]), 0);
    /// assert_eq!(list.offset_of_slice(&list[3..]), 3);
    /// assert_eq!(list.offset_of_slice(&list[5..]), 5);
    /// assert_eq!(list.offset_of_slice(&list[6..]), list.len());
    /// assert_eq!(list.offset_of_slice(&[]), list.len());
    ///
    /// assert_eq!(list.offset_of_slice(&other), list.len());
    ///
    /// ```
    fn offset_of_slice(&self, other: &Self) -> usize;

    /// Returns the index at which `other` starts.
    ///
    /// If `other` is a zero-length slice, or is not inside `self`, this returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SliceExt;
    ///
    /// let list = [0, 1, 2, 3, 4, 5];
    /// 
    /// let other = [0, 1, 2, 3];
    ///
    /// assert_eq!(list.get_offset_of_slice(&list[..0]), None);
    /// assert_eq!(list.get_offset_of_slice(&list[1..]), Some(1));
    /// assert_eq!(list.get_offset_of_slice(&list[3..]), Some(3));
    /// assert_eq!(list.get_offset_of_slice(&list[5..]), Some(5));
    /// assert_eq!(list.get_offset_of_slice(&list[6..]), None);
    ///
    /// assert_eq!(list.get_offset_of_slice(&other), None);
    ///
    /// ```
    fn get_offset_of_slice(&self, other: &Self) -> Option<usize>;

    /// Returns the index of `other` if it's stored in the slice (if it points within the slice).
    ///
    /// If `other` is not inside `self`, this returns `self.len()`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SliceExt;
    ///
    /// let list = vec![0, 1, 2, 3, 4, 5];
    ///
    /// let other = [0, 1, 2, 3];
    ///
    /// assert_eq!(list.index_of(&list[0]), 0);
    /// assert_eq!(list.index_of(&list[3]), 3);
    /// assert_eq!(list.index_of(&list[5]), 5);
    /// assert_eq!(list.index_of(list.as_ptr().wrapping_offset(6)), 6);
    /// assert_eq!(list.index_of(list.as_ptr().wrapping_offset(7)), 6);
    /// assert_eq!(list.index_of(&0), list.len());
    ///
    /// assert_eq!(list.index_of(&other[0]), list.len());
    /// assert_eq!(list.index_of(&other[1]), list.len());
    ///
    /// ```
    fn index_of(&self, other: *const Self::Elem) -> usize;

    /// Returns the index of `other` if it's stored in the slice (if it points within the slice).
    ///
    /// If `other` is not inside `self`, this returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SliceExt;
    ///
    /// let list = vec![0, 1, 2, 3, 4, 5];
    ///
    /// let other = [0, 1, 2, 3];
    ///
    /// assert_eq!(list.get_index_of(&list[0]), Some(0));
    /// assert_eq!(list.get_index_of(&list[3]), Some(3));
    /// assert_eq!(list.get_index_of(&list[5]), Some(5));
    /// assert_eq!(list.get_index_of(list.as_ptr().wrapping_offset(6)), None);
    /// assert_eq!(list.get_index_of(list.as_ptr().wrapping_offset(7)), None);
    /// assert_eq!(list.get_index_of(&0), None);
    ///
    /// assert_eq!(list.get_index_of(&other[0]), None);
    /// assert_eq!(list.get_index_of(&other[1]), None);
    ///
    /// ```
    fn get_index_of(&self, other: *const Self::Elem) -> Option<usize>;

    /// Used for non-panicking slicing.
    ///
    /// If `range.end` is less than `range.start`, this returns an empty slice.
    ///
    /// # `bias` parameter
    /// 
    /// The `bias` parameter, by being converted into a [`SliceBias`], 
    /// determines how this method handles invalid ranges.
    ///
    /// The impl for `[T]` ignores this parameter, saturating ranges at `self.len()`.
    ///
    /// For `str`, it grows the range in the directions determined by `bias` parameter.
    ///
    /// # Examples
    ///
    /// ### `[T]` slice
    ///
    /// ```
    /// use core_extensions::SliceExt;
    ///
    /// let arr = [1, 2, 3, 4, 5, 6];
    /// assert_eq!(arr.slice_lossy(0..3, ()), &arr[..3]);
    /// assert_eq!(arr.slice_lossy(3..1000, ()), &arr[3..]);
    /// assert_eq!(arr.slice_lossy(1000..1000, ()), &[]);
    /// assert_eq!(arr.slice_lossy(1000..0, ()), &[]);
    /// ```
    ///
    /// ### `str` slice
    ///
    /// ```
    /// use core_extensions::SliceExt;
    /// use core_extensions::slices::SliceBias;
    ///
    /// let word = "niño"; // 'ñ' is 2 bytes long , spanning the range 2..4
    ///
    /// assert_eq!(word.slice_lossy(0..3, SliceBias::LEFT ), "ni");
    /// assert_eq!(word.slice_lossy(0..3, SliceBias::RIGHT), "niñ");
    /// assert_eq!(word.slice_lossy(0..3, SliceBias::IN ), "ni");
    /// assert_eq!(word.slice_lossy(0..3, SliceBias::OUT), "niñ");
    ///
    /// assert_eq!(word.slice_lossy(3..10000, ()), "ño");
    /// assert_eq!(word.slice_lossy(3..10000, SliceBias::OUT), "ño");
    ///
    /// assert_eq!(word.slice_lossy(1000..1000, ()), "");
    /// assert_eq!(word.slice_lossy(1000..1000, SliceBias::OUT), "");
    /// assert_eq!(word.slice_lossy(1000..0, SliceBias::OUT), "");
    /// ```
    ///
    /// [`SliceBias`]: struct.SliceBias.html
    ///
    fn slice_lossy<SB>(&self, range: Range<usize>, bias: SB) -> &Self
    where
        SB: Into<SliceBias>;

    /// Used for non-panicking mutable slicing.
    ///
    /// Identical behavior to [`slice_lossy`](#tymethod.slice_lossy) with respect to ranges.
    fn slice_lossy_mut<SB>(&mut self, range: Range<usize>, bias: SB) -> &mut Self
    where
        SB: Into<SliceBias>;
}

macro_rules! impl_common_slice_extensions {($T:ident) => {
    type Elem = $T;

    fn contains_slice(&self,other:&Self)->bool{
        if other.is_empty() {
            return false;
        } else if mem::size_of::<$T>() == 0 {
            return self.as_ptr() == other.as_ptr() && self.len() >= other.len(); 
        }

        let start_self  = self.as_ptr() as usize;
        let end_self    = start_self + self.len() * mem::size_of::<$T>();
        let start_other = other.as_ptr() as usize;
        let end_other   = start_other + other.len() * mem::size_of::<$T>();
        start_self <= start_other && end_other <= end_self
    }

    fn is_slice(&self,other:&Self)->bool{
        self.as_ptr() as usize == other.as_ptr() as usize &&
        self.len() == other.len()
    }

    fn offset_of_slice(&self,other:&Self)->usize{
        if mem::size_of::<$T>() == 0 {
            return if self.as_ptr() == other.as_ptr() {
                0
            } else {
                self.len()
            }
        }
        let offset = (other.as_ptr() as usize).wrapping_sub(self.as_ptr() as usize);
        cmp::min(self.len(),offset / mem::size_of::<$T>())
    }

    fn get_offset_of_slice(&self,other:&Self)->Option<usize>{
        if mem::size_of::<$T>() == 0 {
            if self.as_ptr() == other.as_ptr() {
                Some(0)
            } else {
                None
            }
        } else if self.contains_slice(other) {
            Some((other.as_ptr() as usize - self.as_ptr() as usize)/mem::size_of::<$T>())
        }else{
            None
        }
    }

    fn index_of(&self,other:*const $T)->usize{
        if mem::size_of::<$T>() == 0 {
            return if self.as_ptr() == other {
                0
            } else {
                self.len()
            };
        }
        let offset = (other as *const $T as usize).wrapping_sub(self.as_ptr() as usize);
        cmp::min(self.len(), offset / mem::size_of::<$T>())
    }

    fn get_index_of(&self,other:*const $T)->Option<usize>{
        if mem::size_of::<$T>() == 0 {
            return if self.as_ptr() == other {
                Some(0)
            } else {
                None
            };
        }

        let sub = (other as *const $T as usize)
            .wrapping_sub(self.as_ptr() as usize)
            /mem::size_of::<$T>();

        if sub >= self.len() {
            None
        } else {
            Some(sub)
        }
    }

}}

mod str_impls {
    use super::*;

    fn lossy_str_range(this: &str, mut range: Range<usize>, bias: SliceBias) -> Range<usize> {
        #[inline]
        fn bias_bound(this: &str, mut index: usize, bias: BiasDirection) -> usize {
            if index > this.len() {
                return this.len();
            }
            
            match bias {
                BiasDirection::Left => {
                    while !this.is_char_boundary(index) {
                        index -= 1;
                    }
                },
                BiasDirection::Right => {
                    while !this.is_char_boundary(index) {
                        index += 1;
                    }
                },
            };

            index
        }
        range.start = bias_bound(this, range.start, bias.start);
        range.end = bias_bound(this, range.end, bias.end);
        range.end = cmp::max(range.start, range.end);
        range
    }
    impl SliceExt for str {
        impl_common_slice_extensions! {u8}

        fn slice_lossy<SB>(&self, range: Range<usize>, bias: SB) -> &Self
        where
            SB: Into<SliceBias>,
        {
            &self[lossy_str_range(self, range, bias.into())]
        }

        fn slice_lossy_mut<SB>(&mut self, range: Range<usize>, bias: SB) -> &mut Self
        where
            SB: Into<SliceBias>,
        {
            let r = lossy_str_range(self, range, bias.into());
            &mut self[r]
        }
    }
}

mod slice_impls {
    use super::*;

    fn lossy_range<T>(this: &[T], mut range: Range<usize>) -> Range<usize> {
        let len = this.len();
        range.end = cmp::min(range.end, len);
        range.start = cmp::min(range.start, range.end);
        range
    }

    impl<T> SliceExt for [T] {
        impl_common_slice_extensions! {T}

        fn slice_lossy<SB>(&self, range: Range<usize>, _bias: SB) -> &Self {
            &self[lossy_range(self, range)]
        }

        fn slice_lossy_mut<SB>(&mut self, range: Range<usize>, _bias: SB) -> &mut Self {
            let r = lossy_range(self, range);
            &mut self[r]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc_::{
        vec::Vec,
        string::String,
    };

    #[test]
    fn contains_slice() {
        fn inner<T>(list: &[T; 12]){
            let slice_a = &list[0..4];
            let slice_b = &list[4..8];
            let slice_c = &list[8..12];

            assert_eq!(slice_b.contains_slice(&slice_a[3..]), false);
            assert_eq!(slice_b.contains_slice(&slice_a[4..]), false);

            assert_eq!(slice_b.contains_slice(&slice_b[0..]), true);
            assert_eq!(slice_b.contains_slice(&slice_b[1..]), true);
            assert_eq!(slice_b.contains_slice(&slice_b[2..]), true);
            assert_eq!(slice_b.contains_slice(&slice_b[3..]), true);
            
            assert_eq!(slice_b.contains_slice(&slice_c[0..0]), false);
            assert_eq!(slice_b.contains_slice(&slice_c[0..]), false);
            assert_eq!(slice_b.contains_slice(&slice_c[1..]), false);
        }

        inner(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        inner(&[0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        inner(&[""; 12]);

        {
            let list = [(); 12];
            let slice_a = &list[0..4];
            let slice_b = &list[4..8];
            let slice_c = &list[8..12];
            
            let other = [(); 12];

            assert_eq!(slice_b.contains_slice(&slice_a[3..]), true);
            assert_eq!(slice_b.contains_slice(&slice_a[4..]), false);

            assert_eq!(slice_b.contains_slice(&slice_b[0..]), true);
            assert_eq!(slice_b.contains_slice(&slice_b[1..]), true);
            assert_eq!(slice_b.contains_slice(&slice_b[2..]), true);
            assert_eq!(slice_b.contains_slice(&slice_b[3..]), true);
            
            assert_eq!(slice_b.contains_slice(&slice_c[0..]), true);
            assert_eq!(slice_b.contains_slice(&slice_c[1..]), true);
            
            assert_eq!(list.contains_slice(&other), false);
            assert_eq!(slice_a.contains_slice(&other), false);
            assert_eq!(slice_b.contains_slice(&other), false);
            assert_eq!(slice_c.contains_slice(&other), false);

            assert_eq!(other.contains_slice(&list), false);
            assert_eq!(other.contains_slice(&slice_a), false);
            assert_eq!(other.contains_slice(&slice_b), false);
            assert_eq!(other.contains_slice(&slice_c), false);
        }
    }
    #[test]
    fn offset_of_slice() {
        fn inner<T>(list: &[T; 12]){
            let slice_a = &list[0..4];
            let slice_b = &list[4..8];
            let slice_c = &list[8..12];

            assert_eq!(slice_b.offset_of_slice(&slice_a[3..]), slice_b.len());

            assert_eq!(slice_b.offset_of_slice(&slice_b[0..]), 0);
            assert_eq!(slice_b.offset_of_slice(&slice_b[1..]), 1);
            assert_eq!(slice_b.offset_of_slice(&slice_b[2..]), 2);
            assert_eq!(slice_b.offset_of_slice(&slice_b[3..]), 3);
            
            assert_eq!(slice_b.offset_of_slice(&slice_c[0..]), slice_b.len());
            assert_eq!(slice_b.offset_of_slice(&slice_c[1..]), slice_b.len());
        }

        inner(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        inner(&[0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        inner(&[""; 12]);

        {
            let list = [(); 12];
            let slice_a = &list[0..4];
            let slice_b = &list[4..8];
            let slice_c = &list[8..12];
            
            let other = [(); 12];

            assert_eq!(slice_b.offset_of_slice(&slice_a[3..]), 0);

            assert_eq!(slice_b.offset_of_slice(&slice_b[0..]), 0);
            assert_eq!(slice_b.offset_of_slice(&slice_b[1..]), 0);
            assert_eq!(slice_b.offset_of_slice(&slice_b[2..]), 0);
            assert_eq!(slice_b.offset_of_slice(&slice_b[3..]), 0);
            
            assert_eq!(slice_b.offset_of_slice(&slice_c[0..]), 0);
            assert_eq!(slice_b.offset_of_slice(&slice_c[1..]), 0);

            assert_eq!(list.offset_of_slice(&other), 12);
            assert_eq!(slice_a.offset_of_slice(&other), 4);
            assert_eq!(slice_b.offset_of_slice(&other), 4);
            assert_eq!(slice_c.offset_of_slice(&other), 4);

            assert_eq!(other.offset_of_slice(&list), 12);
            assert_eq!(other.offset_of_slice(&slice_a), 12);
            assert_eq!(other.offset_of_slice(&slice_b), 12);
            assert_eq!(other.offset_of_slice(&slice_c), 12);
        }
    }
    #[test]
    fn get_offset_of_slice() {
        fn inner<T>(list: &[T; 12]){
            let slice_a = &list[0..4];
            let slice_b = &list[4..8];
            let slice_c = &list[8..12];

            assert_eq!(slice_b.get_offset_of_slice(&slice_a[3..]), None);

            assert_eq!(slice_b.get_offset_of_slice(&slice_b[1..1]), None);
            assert_eq!(slice_b.get_offset_of_slice(&slice_b[0..]), Some(0));
            assert_eq!(slice_b.get_offset_of_slice(&slice_b[1..]), Some(1));
            assert_eq!(slice_b.get_offset_of_slice(&slice_b[2..]), Some(2));
            assert_eq!(slice_b.get_offset_of_slice(&slice_b[3..]), Some(3));
            
            assert_eq!(slice_b.get_offset_of_slice(&slice_c[0..]), None);
            assert_eq!(slice_b.get_offset_of_slice(&slice_c[1..]), None);
        }

        inner(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        inner(&[0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        inner(&[""; 12]);

        {
            let list = [(); 12];
            let slice_a = &list[0..4];
            let slice_b = &list[4..8];
            let slice_c = &list[8..12];

            let other = [(); 12];

            assert_eq!(slice_b.get_offset_of_slice(&slice_a[3..]), Some(0));

            assert_eq!(slice_b.get_offset_of_slice(&slice_b[0..]), Some(0));
            assert_eq!(slice_b.get_offset_of_slice(&slice_b[1..]), Some(0));
            assert_eq!(slice_b.get_offset_of_slice(&slice_b[2..]), Some(0));
            assert_eq!(slice_b.get_offset_of_slice(&slice_b[3..]), Some(0));
            
            assert_eq!(slice_b.get_offset_of_slice(&slice_c[0..]), Some(0));
            assert_eq!(slice_b.get_offset_of_slice(&slice_c[1..]), Some(0));
            
            assert_eq!(list.get_offset_of_slice(&other), None);
            assert_eq!(slice_a.get_offset_of_slice(&other), None);
            assert_eq!(slice_b.get_offset_of_slice(&other), None);
            assert_eq!(slice_c.get_offset_of_slice(&other), None);

            assert_eq!(other.get_offset_of_slice(&list), None);
            assert_eq!(other.get_offset_of_slice(&slice_a), None);
            assert_eq!(other.get_offset_of_slice(&slice_b), None);
            assert_eq!(other.get_offset_of_slice(&slice_c), None);
        }
    }
    #[test]
    fn index_of() {
        fn inner<T>(list: &[T; 12]){
            let slice_a = &list[0..4];
            let slice_b = &list[4..8];
            let slice_c = &list[8..12];

            assert_eq!(slice_b.index_of(&slice_a[3]), slice_b.len());

            assert_eq!(slice_b.index_of(&slice_b[0]), 0);
            assert_eq!(slice_b.index_of(&slice_b[1]), 1);
            assert_eq!(slice_b.index_of(&slice_b[2]), 2);
            assert_eq!(slice_b.index_of(&slice_b[3]), 3);
            
            assert_eq!(slice_b.index_of(&slice_c[0]), slice_b.len());
            assert_eq!(slice_b.index_of(&slice_c[1]), slice_b.len());
        }

        inner(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        inner(&[0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        inner(&[""; 12]);

        {
            let list = [(); 12];
            let slice_a = &list[0..4];
            let slice_b = &list[4..8];
            let slice_c = &list[8..12];
            
            let other = [(); 12];

            assert_eq!(slice_b.index_of(&slice_a[3]), 0);

            assert_eq!(slice_b.index_of(&slice_b[0]), 0);
            assert_eq!(slice_b.index_of(&slice_b[1]), 0);
            assert_eq!(slice_b.index_of(&slice_b[2]), 0);
            assert_eq!(slice_b.index_of(&slice_b[3]), 0);
            
            assert_eq!(slice_b.index_of(&slice_c[0]), 0);
            assert_eq!(slice_b.index_of(&slice_c[1]), 0);

            assert_eq!(list.index_of(&other[0]), 12);
            assert_eq!(slice_a.index_of(&other[0]), 4);
            assert_eq!(slice_b.index_of(&other[0]), 4);
            assert_eq!(slice_c.index_of(&other[0]), 4);

            assert_eq!(other.index_of(&list[0]), 12);
            assert_eq!(other.index_of(&slice_a[0]), 12);
            assert_eq!(other.index_of(&slice_b[0]), 12);
            assert_eq!(other.index_of(&slice_c[0]), 12);
        }
    }
    #[test]
    fn get_index_of() {
        fn inner<T>(list: &[T; 12]){
            let slice_a = &list[0..4];
            let slice_b = &list[4..8];
            let slice_c = &list[8..12];

            assert_eq!(slice_b.get_index_of(&slice_a[3]), None);

            assert_eq!(slice_b.get_index_of(&slice_b[0]), Some(0));
            assert_eq!(slice_b.get_index_of(&slice_b[1]), Some(1));
            assert_eq!(slice_b.get_index_of(&slice_b[2]), Some(2));
            assert_eq!(slice_b.get_index_of(&slice_b[3]), Some(3));
            
            assert_eq!(slice_b.get_index_of(&slice_c[0]), None);
            assert_eq!(slice_b.get_index_of(&slice_c[1]), None);
        }

        inner(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        inner(&[0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        inner(&[""; 12]);

        {
            let list = [(); 12];
            let slice_a = &list[0..4];
            let slice_b = &list[4..8];
            let slice_c = &list[8..12];
            
            let other = [(); 12];

            assert_eq!(slice_b.get_index_of(&slice_a[3]), Some(0));

            assert_eq!(slice_b.get_index_of(&slice_b[0]), Some(0));
            assert_eq!(slice_b.get_index_of(&slice_b[1]), Some(0));
            assert_eq!(slice_b.get_index_of(&slice_b[2]), Some(0));
            assert_eq!(slice_b.get_index_of(&slice_b[3]), Some(0));
            
            assert_eq!(slice_b.get_index_of(&slice_c[0]), Some(0));
            assert_eq!(slice_b.get_index_of(&slice_c[1]), Some(0));
            
            assert_eq!(list.get_index_of(&other[0]), None);
            assert_eq!(slice_a.get_index_of(&other[0]), None);
            assert_eq!(slice_b.get_index_of(&other[0]), None);
            assert_eq!(slice_c.get_index_of(&other[0]), None);

            assert_eq!(other.get_index_of(&list[0]), None);
            assert_eq!(other.get_index_of(&slice_a[0]), None);
            assert_eq!(other.get_index_of(&slice_b[0]), None);
            assert_eq!(other.get_index_of(&slice_c[0]), None);
        }
    }
    #[test]
    #[cfg(feature = "alloc")]
    fn slice_lossy_slice_examples() {
        let list = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(list.slice_lossy(0..list.len(), ()), &list[..]);
        assert_eq!(list.slice_lossy(0..1000, ()), &list[..]);
        assert_eq!(list.slice_lossy(0..1000, ()), &list[..]);
        assert_eq!(list.slice_lossy(10..10000, ()), &list[list.len()..]);
        assert_eq!(list.slice_lossy(3..10000, ()), &list[3..]);
        assert_eq!(list.slice_lossy(0..3, ()), &list[..3]);
        assert_eq!(list.slice_lossy(0..2, ()), &list[..2]);
        assert_eq!(list.slice_lossy(0..1, ()), &list[..1]);
        assert_eq!(list.slice_lossy(0..0, ()), &list[..0]);
    }
    #[test]
    fn slice_lossy_str_examples() {
        let word = "niño";
        assert_eq!(word.len(), 5);

        assert!(word
            .slice_lossy(0..word.len(), SliceBias::OUT)
            .is_slice(&word[..]));
        assert!(word
            .slice_lossy(0..1000, SliceBias::OUT)
            .is_slice(&word[..]));
        assert!(word
            .slice_lossy(0..1000, SliceBias::OUT)
            .is_slice(&word[..]));
        assert!(word
            .slice_lossy(10..10000, SliceBias::OUT)
            .is_slice(&word[word.len()..]));
        assert!(word.slice_lossy(0..4, SliceBias::OUT).is_slice(&word[..4]));
        assert!(word.slice_lossy(0..3, SliceBias::OUT).is_slice(&word[..4]));
        assert!(word.slice_lossy(0..2, SliceBias::OUT).is_slice(&word[..2]));

        assert!(word
            .slice_lossy(10..10000, SliceBias::IN)
            .is_slice(&word[word.len()..]));
        assert!(word.slice_lossy(0..4, SliceBias::IN).is_slice(&word[0..4]));
        assert!(word.slice_lossy(0..3, SliceBias::IN).is_slice(&word[0..2]));
        assert!(word.slice_lossy(3..3, SliceBias::IN).is_slice(&word[4..4]));
        assert!(word.slice_lossy(3..4, SliceBias::IN).is_slice(&word[4..4]));
        assert!(word.slice_lossy(2..3, SliceBias::IN).is_slice(&word[2..2]));
        assert!(word.slice_lossy(0..2, SliceBias::IN).is_slice(&word[0..2]));

        assert!(word
            .slice_lossy(10..10000, SliceBias::LEFT)
            .is_slice(&word[word.len()..]));
        assert!(word
            .slice_lossy(0..4, SliceBias::LEFT)
            .is_slice(&word[0..4]));
        assert!(word
            .slice_lossy(0..3, SliceBias::LEFT)
            .is_slice(&word[0..2]));
        assert!(word
            .slice_lossy(3..3, SliceBias::LEFT)
            .is_slice(&word[2..2]));
        assert!(word
            .slice_lossy(3..4, SliceBias::LEFT)
            .is_slice(&word[2..4]));
        assert!(word
            .slice_lossy(2..3, SliceBias::LEFT)
            .is_slice(&word[2..2]));
        assert!(word
            .slice_lossy(0..2, SliceBias::LEFT)
            .is_slice(&word[0..2]));

        assert!(word
            .slice_lossy(10..10000, SliceBias::RIGHT)
            .is_slice(&word[word.len()..]));
        assert!(word
            .slice_lossy(0..4, SliceBias::RIGHT)
            .is_slice(&word[0..4]));
        assert!(word
            .slice_lossy(0..3, SliceBias::RIGHT)
            .is_slice(&word[0..4]));
        assert!(word
            .slice_lossy(3..3, SliceBias::RIGHT)
            .is_slice(&word[4..4]));
        assert!(word
            .slice_lossy(3..4, SliceBias::RIGHT)
            .is_slice(&word[4..4]));
        assert!(word
            .slice_lossy(2..3, SliceBias::RIGHT)
            .is_slice(&word[2..4]));
        assert!(word
            .slice_lossy(0..2, SliceBias::RIGHT)
            .is_slice(&word[0..2]));

        let sub_word = word.slice_lossy(3..10000, SliceBias::OUT);
        assert_eq!(sub_word, &word[2..]);
        assert_eq!(sub_word, "ño");
    }

    #[test]
    #[cfg(feature = "alloc")]
    // Too slow to run in miri, and there's no unsafe code here.
    #[cfg(not(miri))]
    fn slice_lossy_slice_no_panic() {
        use rand::Rng;

        let mut rng = ::rand::thread_rng();
        for _ in 0..50 {
            let slice_len = rng.gen_range(0, 100);
            let slice_ = rng.gen_iter::<usize>().take(slice_len).collect::<Vec<_>>();
            for _ in 0..500 {
                let start = if slice_len == 0 {
                    0
                } else {
                    rng.gen_range(0, slice_len * 2)
                };
                let end = if slice_len == 0 {
                    0
                } else {
                    rng.gen_range(0, slice_len * 2)
                };
                slice_.slice_lossy(start..end, rng.gen::<SliceBias>());
            }
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    // Too slow to run in miri, and there's no unsafe code here.
    #[cfg(not(miri))]
    fn slice_lossy_str_no_panic() {
        use rand::Rng;

        let mut rng = ::rand::thread_rng();
        for _ in 0..50 {
            let char_len = rng.gen_range(0, 100);
            let string = rng.gen_iter::<char>().take(char_len).collect::<String>();
            let slice_len = string.len();
            for _ in 0..500 {
                let start = if slice_len == 0 {
                    0
                } else {
                    rng.gen_range(0, slice_len * 2)
                };
                let end = if slice_len == 0 {
                    0
                } else {
                    rng.gen_range(0, slice_len * 2)
                };
                string.slice_lossy(start..end, rng.gen::<SliceBias>());
            }
        }
    }
}
