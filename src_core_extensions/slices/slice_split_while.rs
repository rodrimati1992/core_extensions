use std_::mem;

#[allow(unused_imports)]
use super::ValSliceExt;


#[inline(always)]
fn next_split<'a,T, P, U: Eq + Clone>(
    pred: &mut P,
    s: &mut &'a [T],
    last: &mut U,
) -> Option<KeySlice<'a,T, U>>
where
    P: FnMut(&'a T) -> U,
{
    let mut next = last.clone();
    if s.is_empty() {
        return None;
    }
    let end = s.iter()
        .position(|x|{
            next=pred(x);
            *last!=next
        })
        .unwrap_or(s.len());
    let (ret, new_s) = s.split_at(end);
    *s = new_s;
    let key = mem::replace(last, next);
    Some(KeySlice { slice: ret, key })
}

#[inline(always)]
fn next_rsplit<'a, T, P, U: Eq + Clone>(
    pred: &mut P,
    s: &mut &'a [T],
    last: &mut U,
) -> Option<KeySlice<'a, T, U>>
where
    P: FnMut(&'a T) -> U,
{
    let mut next = last.clone();
    if s.is_empty() {
        return None;
    }
    let left = (*s).iter()
        .rposition(|x|{
            next=pred(x);
            *last!=next
        })
        .map_or(0,|x|x+1);
    let (new_s, ret) = s.split_at(left);
    *s = new_s;
    let key = mem::replace(last, next);
    Some(KeySlice { slice: ret, key })
}

//-------------------------------------------------------------------------------------------

/// A pair of (slice, key) returned by the 
/// [RSplitSliceWhile](struct.RSplitSliceWhile.html)/
/// [SplitSliceWhile](struct.SplitSliceWhile.html) iterators.
///
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct KeySlice<'a, T:'a, U> {
    /// A slice where every element was mapped to the same key by a closure.
    pub slice: &'a [T],
    /// The value that all the elements in the slice were mapped to.
    pub key: U,
}

impl<'a, T, U> KeySlice<'a, T, U> {
    /// Converts this into a key-slice pair.
    pub fn into_pair(self) -> (U, &'a [T]){
        (self.key, self.slice)
    }
}

//-------------------------------------------------------------------------------------------

/// Iterator over slices,
/// in which all the elements in each slice were mapped to the same key by a closure.
///
/// Look [here](trait.ValSliceExt.html#method.split_while) for examples.
#[derive(Debug, Clone)]
pub struct SplitSliceWhile<'a, T:'a, P, U> {
    pub(super) mapper: P,
    pub(super) s: &'a [T],
    pub(super) last_left: Option<U>,
    pub(super) last_right: Option<U>,
}

impl<'a, T, P, U: Eq + Clone> Iterator for SplitSliceWhile<'a, T, P, U>
where
    P: FnMut(&'a T) -> U,
{
    type Item = KeySlice<'a, T, U>;
    fn next(&mut self) -> Option<Self::Item> {
        next_split(&mut self.mapper, &mut self.s,self.last_left.as_mut()?)
    }
    fn size_hint(&self)->(usize,Option<usize>){
        let s=self.s;
        let min_len=if s.is_empty() { 0 }else{ 1 };
        ( min_len , Some(s.len()) )
    }
}

impl<'a, T, P, U: Eq + Clone> DoubleEndedIterator for SplitSliceWhile<'a, T, P, U>
where
    P: FnMut(&'a T) -> U,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        next_rsplit(&mut self.mapper, &mut self.s, self.last_right.as_mut()?)
    }
}

//-------------------------------------------------------------------------------------------

/// Iterator over slices,
/// in which all the elements in each slice were mapped to the same key by a closure,
/// iterating from the end.
///
/// Look [here](trait.ValSliceExt.html#method.rsplit_while) for examples.
#[derive(Debug, Clone)]
pub struct RSplitSliceWhile<'a, T:'a, P, U> {
    pub(super) mapper: P,
    pub(super) s: &'a [T],
    pub(super) last_left: Option<U>,
    pub(super) last_right: Option<U>,
}

impl<'a, T, P, U: Eq + Clone> Iterator for RSplitSliceWhile<'a, T, P, U>
where
    P: FnMut(&'a T) -> U,
{
    type Item = KeySlice<'a, T, U>;
    fn next(&mut self) -> Option<Self::Item> {
        next_rsplit(&mut self.mapper, &mut self.s, self.last_right.as_mut()?)
    }
    fn size_hint(&self)->(usize,Option<usize>){
        let s=self.s;
        let min_len=if s.is_empty() { 0 }else{ 1 };
        ( min_len , Some(s.len()) )
    }
}

impl<'a, T, P, U: Eq + Clone> DoubleEndedIterator for RSplitSliceWhile<'a, T, P, U>
where
    P: FnMut(&'a T) -> U,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        next_split(&mut self.mapper, &mut self.s, self.last_left.as_mut()?)
    }
}

//-------------------------------------------------------------------------------------------


#[cfg(test)]
#[cfg(feature = "alloc")]
mod test{
    use super::*;

    use alloc::{
        vec::Vec,
        vec,
    };

    fn func<'a,T,U,F>(s:&'a [T],f:F)->Vec<(U,Vec<T>)>
    where
        T:Clone,
        F:FnMut(&'a T)->U,
        U:Eq+Clone,
    {
        s.split_while(f).map(|v| (v.key,v.slice.to_vec()) ).collect()
    }

    fn rfunc<'a,T,U,F>(s:&'a [T],f:F)->Vec<(U,Vec<T>)>
    where
        T:Clone,
        F:FnMut(&'a T)->U,
        U:Eq+Clone,
    {
        s.rsplit_while(f).map(|v| (v.key,v.slice.to_vec()) ).collect()
    }

    fn new_singletons()->Vec<Vec<u32>>{
        (0..30).map(|x| vec![x] ).collect()
    }

    fn new_list_0()->Vec<u32>{
        vec![0,9,1,4,5]
    }
    fn new_list_1()->Vec<u32>{
        vec![90,91,92,93,94,95]
    }
    fn new_list_2()->Vec<u32>{
        vec![10,5,4,17]
    }
    fn mapper_0(x:&u32)->u32{x%3}

    fn mapper_1(x:&u32)->u32{*x}

    fn mapper_2(x:&u32)->u32{x/3}

    fn spair<T>(v:&T)->(T,Vec<T>)
    where T:Clone
    {
        (v.clone(),vec![v.clone()])
    }



    #[test]
    fn with_every_mapper(){

        for mapper in vec![ mapper_0 as fn(&_)->_,mapper_1,mapper_2 ] {
            assert_eq!(func(&vec![] ,mapper_0),vec![]);
            assert_eq!(rfunc(&vec![] ,mapper_0),vec![]);
            
            let singletons=new_singletons();
            for list in singletons.iter() {
                assert_eq!(func(list,mapper),vec![(mapper(&list[0]),list.clone())]);
                assert_eq!(rfunc(list,mapper),vec![(mapper(&list[0]),list.clone())]);
            }
        }
    }



    #[test]
    fn with_mapper_0(){
        {
            let list_0=new_list_0();
            let mut expected=vec![(0,vec![0,9]),(1,vec![1,4]),(2,vec![5])];
            assert_eq!(func(&list_0,mapper_0),expected);
            expected.reverse();
            assert_eq!(rfunc(&list_0,mapper_0),expected);
        }
        for list in vec![new_list_1(),new_list_2()] {
            let mut expected=list.iter()
                .map(|x| (mapper_0(x),vec![*x]) )
                .collect::<Vec<_>>();
            assert_eq!(func(&list,mapper_0),expected);
            expected.reverse();
            assert_eq!(rfunc(&list,mapper_0),expected);
        }
    }

    #[test]
    fn with_mapper_1(){
        for list in vec![new_list_0(),new_list_1(),new_list_2()] {
            let mut expected=list.iter().map(spair).collect::<Vec<_>>();
            assert_eq!(func(&list,mapper_1),expected);
            expected.reverse();
            assert_eq!(rfunc(&list,mapper_1),expected);
        }    
    }

    #[test]
    fn with_mapper_2(){
        {
            let list_0=new_list_0();
            let mut expected=vec![(0,vec![0]),(3,vec![9]),(0,vec![1]),(1,vec![4,5])];
            assert_eq!(func(&list_0,mapper_2),expected);
            expected.reverse();
            assert_eq!(rfunc(&list_0,mapper_2),expected);
        }
        {
            let list_1=new_list_1();
            let mut expected=vec![(30,vec![90,91,92]),(31,vec![93,94,95])];
            assert_eq!(func(&list_1,mapper_2),expected);
            expected.reverse();
            assert_eq!(rfunc(&list_1,mapper_2),expected);
        }
        {
            let list_2=new_list_2();
            let mut expected=vec![(3,vec![10]),(1,vec![5,4]),(5,vec![17])];
            assert_eq!(func(&list_2,mapper_2),expected);
            expected.reverse();
            assert_eq!(rfunc(&list_2,mapper_2),expected);
        }
    }



}