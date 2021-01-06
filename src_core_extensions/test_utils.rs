use std_::cell::Cell;

///////////////////////////////////////////////////////////////////////////////

/// Used to test that types that manually drop stuff do it correctly.
#[derive(Clone)]
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
