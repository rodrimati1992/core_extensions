use krate::transparent_newtype::{TransparentNewtype, TransparentNewtypeExt};

#[cfg(feature = "alloc")]
use krate::transparent_newtype::{from_inner_vec, into_inner_vec};


#[cfg(feature = "derive")]
mod tn_derive_tests;




#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
struct Trans<T: ?Sized>(T);

unsafe impl<T: ?Sized> TransparentNewtype for Trans<T>{
    type Inner = T;

    krate::impl_transparent_newtype!{Self}
}


fn assert_tyoe<L: ?Sized, R: ?Sized>(_: &L) {
    assert_eq!(std::any::type_name::<L>(), std::any::type_name::<R>());
}



#[test]
fn test_core() {
    {
        let foo = Trans::from_inner("hello".to_string());
        assert_tyoe::<_, Trans<String>>(&foo);
        assert_eq!(foo.0, "hello");

        assert_tyoe::<_, String>(&Trans("hello".to_string()).into_inner());
        assert_eq!(foo.into_inner(), "hello");
    }
    {
        let foo = Trans::from_inner_ref("hello");
        assert_tyoe::<_, &Trans<str>>(&foo);
        assert_eq!(&foo.0, "hello");
        
        assert_tyoe::<_, &str>(&foo.as_inner());
        assert_eq!(foo.as_inner(), "hello");
    }
    {
        let mut arr = [3u8, 5, 8];
        let foo = Trans::from_inner_mut(&mut arr[..]);
        assert_tyoe::<_, &mut Trans<[u8]>>(&foo);
        assert_eq!(&mut foo.0, &mut [3, 5, 8][..]);

        assert_tyoe::<_, &mut [u8]>(&foo.as_inner_mut());
        assert_eq!(foo.as_inner_mut(), &mut [3, 5, 8][..]);
    }
}

#[test]
#[cfg(feature = "alloc")]
fn test_alloc() {
    use alloc::{
        boxed::Box,
        rc::Rc,
        sync::Arc,
    };

    let make_box = || Box::<str>::from("hello");
    let make_rc = || Rc::<str>::from("hello");
    let make_arc = || Arc::<str>::from("hello");

    {
        let foo = Trans::from_inner_box(make_box());
        assert_tyoe::<_, Box<Trans<str>>>(&foo);
        assert_eq!(foo.0, *"hello");

        assert_tyoe::<_, Box<str>>(&Trans::from_inner_box(make_box()).into_inner_box());
        assert_eq!(&*foo.into_inner_box(), "hello");
    }

    #[cfg(feature = "rust_1_46")]
    {
        let foo = Trans::from_inner_arc(make_arc());
        assert_tyoe::<_, Arc<Trans<str>>>(&foo);
        assert_eq!(foo.0, *"hello");

        assert_tyoe::<_, Arc<str>>(&Trans::from_inner_arc(make_arc()).into_inner_arc());
        assert_eq!(&*foo.into_inner_arc(), "hello");
    }
    {
        let foo = Trans::from_inner_arc(make_arc());
        assert_tyoe::<_, Arc<Trans<str>>>(&foo);
        assert_eq!(foo.0, *"hello");

        assert_tyoe::<_, Arc<str>>(&Trans::into_inner_arc(Trans::from_inner_arc(make_arc())));
        assert_eq!(&*Trans::into_inner_arc(foo), "hello");
    }

    #[cfg(feature = "rust_1_46")]
    {
        let foo = Trans::from_inner_rc(make_rc());
        assert_tyoe::<_, Rc<Trans<str>>>(&foo);
        assert_eq!(foo.0, *"hello");

        assert_tyoe::<_, Rc<str>>(&Trans::from_inner_rc(make_rc()).into_inner_rc());
        assert_eq!(&*foo.into_inner_rc(), "hello");
    }
    {
        let foo = Trans::from_inner_rc(make_rc());
        assert_tyoe::<_, Rc<Trans<str>>>(&foo);
        assert_eq!(foo.0, *"hello");

        assert_tyoe::<_, Rc<str>>(&Trans::into_inner_rc(Trans::from_inner_rc(make_rc())));
        assert_eq!(&*Trans::into_inner_rc(foo), "hello");
    }
}


#[test]
#[cfg(feature = "alloc")]
fn test_other() {
    use core::{
        num::Wrapping,
        mem::ManuallyDrop,
    };

    use alloc::{
        boxed::Box,
        rc::Rc,
    };


    macro_rules! test_types {
        ($typ:ident, $make:expr) => {
            {
                let boxed = Box::new([3u8, 5, 8]) as Box<[u8]>;
                let slice = || <[$typ<_>]>::from_inner_box(boxed.clone());
                assert_tyoe::<_, Box<[$typ<u8>]>>(&slice());
                
                assert_tyoe::<_, Box<[u8]>>(&slice().into_inner_box());
                assert_eq!(slice().into_inner_box(), boxed);
            }
            {
                let boxed = Rc::new([3u8, 5, 8]) as Rc<[u8]>;
                let slice = || <[$typ<_>]>::from_inner_rc(boxed.clone());
                assert_tyoe::<_, Rc<[$typ<u8>]>>(&slice());
                
                assert_tyoe::<_, Rc<[u8]>>(&TransparentNewtypeExt::into_inner_rc(slice()));
                assert_eq!(TransparentNewtypeExt::into_inner_rc(slice()), boxed);
            }
        }
    }

    test_types!{Trans, Trans}
    test_types!{Wrapping, Wrapping}
    test_types!{ManuallyDrop, ManuallyDrop::new}
}



#[test]
#[cfg(feature = "alloc")]
fn vec_test() {
    {
        let from = from_inner_vec::<Trans<_>>(vec![3u8, 5, 8]);

        assert_tyoe::<_, Vec<Trans<u8>>>(&from_inner_vec::<Trans<_>>(vec![3u8, 5, 8]));

        assert_eq!(from, vec![Trans(3), Trans(5), Trans(8)]);
    }
    {
        let into = into_inner_vec(vec![Trans(3u8), Trans(5), Trans(8)]);

        assert_tyoe::<_, Vec<u8>>(&into_inner_vec(vec![Trans(3u8), Trans(5), Trans(8)]));

        assert_eq!(into, vec![3, 5, 8]);
    }
}