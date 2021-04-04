use super::RunOnDrop;

use std_::cell::Cell;  
use test_utils::DecOnDrop;  

#[test]
fn drop_guard() {
    let count = Cell::new(0);
    
    {
        let guard = RunOnDrop::new(DecOnDrop::new(&count), |rod|{
            assert_eq!(count.get(), 15);
            drop(rod);
            assert_eq!(count.get(), 14);
        });

        assert_eq!(count.get(), 0);
        count.set(16);

        let clone = guard.get().clone();
        assert_eq!(count.get(), 16);
        drop(clone);
        assert_eq!(count.get(), 15);

    }

    assert_eq!(count.get(), 14);
}


#[cfg(feature = "alloc")]
#[test]
fn consume_owned() {
    use alloc::boxed::Box;

    {
        let guard = RunOnDrop::new(Box::new(100), |mut x|{
            *x += 8;
            assert_eq!(*x, 108); 
        });
        assert_eq!(*guard.into_inner(), 100)
    }

    {
        let mut ran = false;
        let guard = RunOnDrop::new(Box::new(100), |mut x|{
            *x += 8;
            assert_eq!(*x, 108); 
            ran = true;
        });
        drop(guard);
        assert_eq!(ran, true)
    }

    {
        let mut ran = false;
        let mut guard = RunOnDrop::new(Box::new(100), |x|{
            assert_eq!(*x, 108); 
            ran = true;
        });
        **guard.get_mut() += 8;
        drop(guard);
        assert_eq!(ran, true)
    }

}



#[test]
fn unwrap_run_on_drop() {
    let count = Cell::new(0);
    
    {
        let guard = RunOnDrop::new(DecOnDrop::new(&count), |rod|{
            assert_eq!(count.get(), 15);
            drop(rod);
            assert_eq!(count.get(), 14);
        });

        assert_eq!(count.get(), 0);
        count.set(16);

        let clone = guard.get().clone();
        assert_eq!(count.get(), 16);
        drop(clone);
        assert_eq!(count.get(), 15);

        let rod = guard.into_inner();
        assert_eq!(count.get(), 15);
        drop(rod);
        assert_eq!(count.get(), 14);
    }

    assert_eq!(count.get(), 14);
}