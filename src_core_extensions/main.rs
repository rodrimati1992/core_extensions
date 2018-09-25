extern crate core_extensions;

use core_extensions::type_level_bool::{True,False,MutableIfTrue};

#[derive(Debug,PartialEq,Copy,Clone)]
pub struct Point{
    x:u32,
    y:u32,
}

fn main(){
    let mut wrapped=MutableIfTrue::new(Point{x:0,y:0},True);
    let point_1=Point{x:0,y:1};
    *wrapped=point_1;
    assert_eq!(*wrapped,point_1);
    let mut wrapped:MutableIfTrue<Point,False>=
        wrapped.freeze();

    // Neither of the lines bellow will compile because mutability==`False`
    //*wrapped=Point{x:0,y:1};
    //wrapped.x=200;

    let point_2=Point{x:101,y:202};
    //But this will compile
    wrapped=MutableIfTrue::new(point_2,wrapped.mutability());

    assert_eq!(*wrapped,point_2);


}