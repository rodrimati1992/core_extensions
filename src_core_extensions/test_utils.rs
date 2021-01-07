use std_::cell::Cell;
use std_::cmp::PartialEq;

///////////////////////////////////////////////////////////////////////////////


/// Used to test that types that manually drop stuff do it correctly.
#[derive(Debug, Clone)]
pub struct DecOnDrop<'a> {
    counter: &'a Cell<usize>,
}

impl<'a> DecOnDrop<'a> {
    pub fn new(counter: &'a Cell<usize>) -> Self {
        Self { counter }
    }
}

impl<'a> Drop for DecOnDrop<'a> {
    fn drop(&mut self) {
        self.counter.set(self.counter.get() - 1);
    }
}


///////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Clone)]
pub struct WithVal<'a, T>(pub T, pub DecOnDrop<'a>);

impl<'a, 'b, T> PartialEq<WithVal<'b, T>> for WithVal<'a, T>
where
    T: PartialEq
{
    fn eq(&self, other: &WithVal<'b, T>) -> bool {
        self.0 == other.0
    }
}


///////////////////////////////////////////////////////////////////////////////


#[derive(Debug)]
pub struct CloneLimit {
    max: usize,
    current: Cell<usize>,
    dropped: Cell<usize>,
}

impl CloneLimit {
    pub fn new(max: usize) -> Self {
        Self {
            max,
            current: Cell::new(0),
            dropped: Cell::new(0),
        }
    }

    pub fn clone_count(&self) -> usize {
        self.current.get()
    }

    pub fn drop_count(&self) -> usize {
        self.dropped.get()
    }
}


#[derive(Debug)]
pub struct MaxClones<'a, T>{
    value: T,
    limit: &'a CloneLimit,
}

impl<'a, T> MaxClones<'a, T> {
    pub fn new(value: T, limit: &'a CloneLimit) -> Self {
        Self{value, limit}
    }
}

impl<'a, T> Clone for MaxClones<'a, T> 
where
    T: Clone
{
    fn clone(&self) -> Self {
        if self.limit.current.get() < self.limit.max {
            let n = &self.limit.current;
            n.set(n.get() + 1);
        } else {
            panic!("Reached max clone count: {}", self.limit.max);
        }

        Self::new(self.value.clone(), self.limit)
    }
}

impl<'a, 'b, T> PartialEq<MaxClones<'b, T> > for MaxClones<'a, T> 
where
    T: PartialEq
{
    fn eq(&self, other: &MaxClones<'b, T> ) -> bool {
        self.value == other.value
    }
}

impl<'a, T> Drop for MaxClones<'a, T>  {
    fn drop(&mut self) {
        let n = &self.limit.dropped;
        n.set(n.get() + 1);
    }
}




