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