//! Extension trait for \[T\] and [str].
//!
//!
//!

// use ranges::RangeBounds;
use super::{BiasDirection, SliceBias,SplitSliceWhile,RSplitSliceWhile};
#[allow(unused_imports)]
use SelfOps;

use std_::borrow::Borrow;
use std_::cmp;
use std_::mem;
use std_::ops::Range;


/// Extension trait for `[T]`.
pub trait ValSliceExt<T>:Borrow<[T]>+SliceExt<T>{
/**
Returns an iterator over subslices whose elements were mapped to the same key by mapper.

Returns an impl Iterator\<Item=[KeySlice](../struct.KeySlice.html)\<T\>>

# Example

```
use core_extensions::ValSliceExt;
use core_extensions::slices::KeySlice;

fn func<'a,T,U,F>(s:&'a [T],f:F)->Vec<(U,Vec<T>)>
where
    T:Clone,
    F:FnMut(&'a T)->U,
    U:Eq+Clone,
{
    s.split_while(f).map(|v| (v.key,v.slice.to_vec()) ).collect()
}

{
    let list=vec![0,1,2,3,4,5,6,7,8];

    assert_eq!(
        list.split_while(|x| x/4 ).collect::<Vec<_>>(),
        vec![
            KeySlice{key:0,slice:&[0,1,2,3]},
            KeySlice{key:1,slice:&[4,5,6,7]},
            KeySlice{key:2,slice:&[8]},
        ]
    );
}

{
    let list=vec![0,4,1,5,9,8,7];

    assert_eq!(
        vec![(0,vec![0,4]), (1,vec![1,5,9]), (0,vec![8]), (3,vec![7])],
        func(&list,|x| x%4 )
    );
}


```


*/
    fn split_while<'a, P, U>(&'a self, mut mapper: P) -> SplitSliceWhile<'a, T, P, U>
    where
        P: FnMut(&'a T) -> U,
        U: Eq + Clone,
    {
        let this:&'a [T]=self.borrow();
        SplitSliceWhile {
            last_left: this.first().map(&mut mapper),
            last_right: this.last().map(&mut mapper),
            mapper,
            s: this,
        }
    }
/**

A variation of split_while that iterates
from the right(the order of subslices is reversed).

Returns an impl Iterator\<Item=[KeySlice](../struct.KeySlice.html)\<T\>>

# Example

```
use core_extensions::ValSliceExt;
use core_extensions::slices::KeySlice;

fn func<'a,T,U,F>(s:&'a [T],f:F)->Vec<(U,Vec<T>)>
where
    T:Clone,
    F:FnMut(&'a T)->U,
    U:Eq+Clone,
{
    s.rsplit_while(f).map(|v| (v.key,v.slice.to_vec()) ).collect()
}

{
    let list=vec![0,1,2,3,4,5,6,7,8];

    assert_eq!(
        list.rsplit_while(|x| x/4 ).collect::<Vec<_>>(),
        vec![
            KeySlice{key:2,slice:&[8]},
            KeySlice{key:1,slice:&[4,5,6,7]},
            KeySlice{key:0,slice:&[0,1,2,3]},
        ]
    );
}

{
    let list=vec![0,4,1,5,9,8,7];

    assert_eq!(
        vec![ (3,vec![7]), (0,vec![8]), (1,vec![1,5,9]), (0,vec![0,4]) ],
        func(&list,|x| x%4 )
    );
}



```

*/
    fn rsplit_while<'a, P, U>(&'a self, mut mapper: P) -> RSplitSliceWhile<'a, T, P, U>
    where
        P: FnMut(&'a T) -> U,
        U: Eq + Clone,
    {
        let this:&'a [T]=self.borrow();
        RSplitSliceWhile {
            last_left: this.first().map(&mut mapper),
            last_right: this.last().map(&mut mapper),
            mapper,
            s: this,
        }
    }
}

impl<T,This> ValSliceExt<T> for This
where
    This:?Sized+Borrow<[T]>+SliceExt<T>,
{}



/// Extension trait for `[T]` and `str`.
pub trait SliceExt<T> {
    /// Checks whether self fully contains another slice in memory.
    ///
    /// If `other` is a zero-sized slice it is not contained inside `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SliceExt;
    /// let list=vec![0,1,2,3,4,5];
    /// 
    /// // Slices never contain empty slices
    /// assert!(!list.contains_slice(&list[..0]));
    /// assert!( list.contains_slice(&list[..1]));
    /// assert!( list.contains_slice(&list[3..]));
    ///
    /// ```
    fn contains_slice(&self, other: &Self) -> bool;
    /// Checks whether self is exactly the `other` slice in memory.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SliceExt;
    /// let list = [0,1,2,3,4,5];
    /// let slice_0=&list[..0];
    /// let slice_1=&list[..];
    ///
    /// assert!( slice_0.is_slice(slice_0));
    /// assert!( slice_0.is_slice(&list[..0]));
    /// assert!(!slice_0.is_slice(slice_1));
    /// assert!(!slice_0.is_slice(&[]));
    ///
    /// ```
    fn is_slice(&self, other: &Self) -> bool;
    /// Returns the index at which `other` starts.
    ///
    /// If `other` is not inside `self` returns self.len()
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SliceExt;
    /// let list=vec![0,1,2,3,4,5];
    /// let slice_0=&list[..0];
    /// let slice_1=&list[3..];
    ///
    /// assert_eq!(list.offset_of_slice(slice_0),0);
    /// assert_eq!(list.offset_of_slice(slice_1),3);
    /// assert_eq!(list.offset_of_slice(&[]),list.len());
    ///
    /// ```
    fn offset_of_slice(&self, other: &Self) -> usize;
    /// Returns the index at which `other` starts.
    ///
    /// If `other` is a zero-sized slice, this returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SliceExt;
    /// let list = [0,1,2,3,4,5];
    ///
    /// assert_eq!(list.get_offset_of_slice(&list[..0]), None);
    /// assert_eq!(list.get_offset_of_slice(&list[1..]), Some(1));
    /// assert_eq!(list.get_offset_of_slice(&list[3..]), Some(3));
    ///
    /// ```
    fn get_offset_of_slice(&self, other: &Self) -> Option<usize>;
    /// Returns the index of `other` if it's stored in the slice (if it points within the slice).
    ///
    /// If `other` is not inside `self` returns self.len()
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SliceExt;
    /// let list=vec![0,1,2,3,4,5];
    /// let elem_0=&list[0];
    /// let elem_3=&list[3];
    /// let outside=&0;
    ///
    /// assert_eq!(list.index_of(elem_0),0);
    /// assert_eq!(list.index_of(elem_3),3);
    /// assert_eq!(list.index_of(outside),list.len());
    ///
    /// ```
    fn index_of(&self, other: *const T) -> usize;
    /// Returns the index of `other` if it's stored in the slice (if it points within the slice).
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::SliceExt;
    /// let list=vec![0,1,2,3,4,5];
    /// let elem_0=&list[0];
    /// let elem_3=&list[3];
    /// let outside=&0;
    ///
    /// assert_eq!(list.get_index_of(elem_0),Some(0));
    /// assert_eq!(list.get_index_of(elem_3),Some(3));
    /// assert_eq!(list.get_index_of(outside),None);
    ///
    /// ```
    fn get_index_of(&self, other: *const T) -> Option<usize>;
    /// Used for non-panicking slicing.
    ///
    /// For `[T]` it simply saturates ranges at self.len().
    /// It is recommended to pass `()` as the `bias` parameter as it has no effect.
    ///
    /// For `str`,it grows the range in the directions determined by `bias`.
    ///
    /// For more examples on `str` look at [SliceBias].
    ///
    /// # Example ,\[u8\]
    /// ```
    /// use core_extensions::SliceExt;
    ///
    /// let arr=[1,2,3,4,5,6];
    /// assert_eq!(arr.slice_lossy(0   ..3   ,()),&arr[..3]);
    /// assert_eq!(arr.slice_lossy(3   ..1000,()),&arr[3..]);
    /// assert_eq!(arr.slice_lossy(1000..1000,()),&[]);
    /// ```
    ///
    /// # Example ,str
    /// ```
    /// use core_extensions::SliceExt;
    /// use core_extensions::slices::SliceBias;
    ///
    /// let word="niño"; // 'ñ' is 2 bytes long , spanning the range 2..4
    ///
    /// assert_eq!(word.slice_lossy(0..3,SliceBias::LEFT ),"ni");
    /// assert_eq!(word.slice_lossy(0..3,SliceBias::RIGHT),"niñ");
    /// assert_eq!(word.slice_lossy(0..3,SliceBias::IN ),"ni");
    /// assert_eq!(word.slice_lossy(0..3,SliceBias::OUT),"niñ");
    ///
    /// assert_eq!(word.slice_lossy(3..10000,()            ),"ño");
    /// assert_eq!(word.slice_lossy(3..10000,SliceBias::OUT),"ño");
    ///
    /// assert_eq!(word.slice_lossy(1000..1000,()            ),"");
    /// assert_eq!(word.slice_lossy(1000..1000,SliceBias::OUT),"");
    /// ```
    ///
    fn slice_lossy<SB>(&self, range: Range<usize>, bias: SB) -> &Self
    where
        SB: Into<SliceBias>;
    /// Used for non-panicking mutable slicing.
    ///
    /// Identical behavior to [slice_lossy](#tymethod.slice_lossy) with respect to ranges.
    fn slice_lossy_mut<SB>(&mut self, range: Range<usize>, bias: SB) -> &mut Self
    where
        SB: Into<SliceBias>;
}

macro_rules! impl_common_slice_extensions {($T:ident) => {
    fn contains_slice(&self,other:&Self)->bool{
        if mem::size_of::<$T>() == 0 || other.is_empty() {
            return false;
        }

        let start_self  =self.as_ptr() as usize;
        let end_self    =start_self+self.len()*mem::size_of::<$T>();
        let start_other =other.as_ptr() as usize;
        let end_other   =start_other+other.len()*mem::size_of::<$T>();
        start_self<=start_other&&end_other<=end_self
    }
    fn is_slice(&self,other:&Self)->bool{
        self.as_ptr() as usize==other.as_ptr() as usize&&
        self.len()==other.len()
    }

    fn offset_of_slice(&self,other:&Self)->usize{
        let size_of:usize=mem::size_of::<$T>();
        if size_of==0 { return 0; }
        let offset=(other.as_ptr() as usize).wrapping_sub(self.as_ptr() as usize);
        cmp::min(self.len(),offset/size_of)
    }
    fn get_offset_of_slice(&self,other:&Self)->Option<usize>{
        let size_of:usize = mem::size_of::<$T>();
        if size_of==0 {
            Some(0)
        } else if self.contains_slice(other) {
            Some((other.as_ptr() as usize - self.as_ptr() as usize)/size_of)
        }else{
            None
        }
    }
    fn index_of(&self,other:*const $T)->usize{
        let size_of:usize=mem::size_of::<$T>();
        if size_of==0 { return 0; }
        let offset=(other as *const $T as usize).wrapping_sub(self.as_ptr() as usize);
        cmp::min(self.len(),offset/size_of)
    }
    fn get_index_of(&self,other:*const $T)->Option<usize>{
        let size_of:usize=mem::size_of::<$T>();
        if size_of == 0 { return Some(0); }
        let sub = (other as *const $T as usize)
            .wrapping_sub(self.as_ptr() as usize)
            /size_of;
        if sub >= self.len() {
            None
        } else {
            Some(sub)
        }
    }

}}

mod str_impls {
    use super::*;
    use strings::StringExt;

    fn lossy_str_range(this: &str, mut range: Range<usize>, bias: SliceBias) -> Range<usize> {
        #[inline]
        fn bias_bound(this: &str, bound: &mut usize, bias: BiasDirection) {
            *bound = match bias {
                BiasDirection::Left => this.left_char_boundary(*bound),
                BiasDirection::Right => this.right_char_boundary(*bound),
            };
        }
        bias_bound(this, &mut range.start, bias.start);
        bias_bound(this, &mut range.end, bias.end);
        range.end = cmp::max(range.start, range.end);
        range
    }
    impl SliceExt<u8> for str {
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

    impl<T> SliceExt<T> for [T] {
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

    use alloc_::vec::Vec;
    use alloc_::string::String;

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

            assert_eq!(slice_b.contains_slice(&slice_a[3..]), false);
            assert_eq!(slice_b.contains_slice(&slice_a[4..]), false);

            assert_eq!(slice_b.contains_slice(&slice_b[0..]), false);
            assert_eq!(slice_b.contains_slice(&slice_b[1..]), false);
            assert_eq!(slice_b.contains_slice(&slice_b[2..]), false);
            assert_eq!(slice_b.contains_slice(&slice_b[3..]), false);
            
            assert_eq!(slice_b.contains_slice(&slice_c[0..]), false);
            assert_eq!(slice_b.contains_slice(&slice_c[1..]), false);
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

            assert_eq!(slice_b.offset_of_slice(&slice_a[3..]), 0);

            assert_eq!(slice_b.offset_of_slice(&slice_b[0..]), 0);
            assert_eq!(slice_b.offset_of_slice(&slice_b[1..]), 0);
            assert_eq!(slice_b.offset_of_slice(&slice_b[2..]), 0);
            assert_eq!(slice_b.offset_of_slice(&slice_b[3..]), 0);
            
            assert_eq!(slice_b.offset_of_slice(&slice_c[0..]), 0);
            assert_eq!(slice_b.offset_of_slice(&slice_c[1..]), 0);
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

            assert_eq!(slice_b.get_offset_of_slice(&slice_a[3..]), Some(0));

            assert_eq!(slice_b.get_offset_of_slice(&slice_b[0..]), Some(0));
            assert_eq!(slice_b.get_offset_of_slice(&slice_b[1..]), Some(0));
            assert_eq!(slice_b.get_offset_of_slice(&slice_b[2..]), Some(0));
            assert_eq!(slice_b.get_offset_of_slice(&slice_b[3..]), Some(0));
            
            assert_eq!(slice_b.get_offset_of_slice(&slice_c[0..]), Some(0));
            assert_eq!(slice_b.get_offset_of_slice(&slice_c[1..]), Some(0));
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

            assert_eq!(slice_b.index_of(&slice_a[3]), 0);

            assert_eq!(slice_b.index_of(&slice_b[0]), 0);
            assert_eq!(slice_b.index_of(&slice_b[1]), 0);
            assert_eq!(slice_b.index_of(&slice_b[2]), 0);
            assert_eq!(slice_b.index_of(&slice_b[3]), 0);
            
            assert_eq!(slice_b.index_of(&slice_c[0]), 0);
            assert_eq!(slice_b.index_of(&slice_c[1]), 0);
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

            assert_eq!(slice_b.get_index_of(&slice_a[3]), Some(0));

            assert_eq!(slice_b.get_index_of(&slice_b[0]), Some(0));
            assert_eq!(slice_b.get_index_of(&slice_b[1]), Some(0));
            assert_eq!(slice_b.get_index_of(&slice_b[2]), Some(0));
            assert_eq!(slice_b.get_index_of(&slice_b[3]), Some(0));
            
            assert_eq!(slice_b.get_index_of(&slice_c[0]), Some(0));
            assert_eq!(slice_b.get_index_of(&slice_c[1]), Some(0));
        }
    }
    #[test]
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
