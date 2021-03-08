use core_extensions::{TransparentNewtype, TransparentNewtypeExt};

use std::marker::PhantomData as PD;

#[repr(transparent)]
struct Trans<T: ?Sized>(T);

unsafe impl<T: ?Sized> TransparentNewtype for Trans<T>{
    type Inner = T;

    core_extensions::impl_transparent_newtype!{Self}
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

    {
        let foo = Trans::from_inner_arc(make_arc());
        assert_tyoe::<_, Arc<Trans<str>>>(&foo);
        assert_eq!(foo.0, *"hello");

        assert_tyoe::<_, Arc<str>>(&Trans::from_inner_arc(make_arc()).into_inner_arc());
        assert_eq!(&*foo.into_inner_arc(), "hello");
    }
    #[cfg(feature = "rust_1_46")]
    {
        let foo = Trans::from_inner_arc(make_arc());
        assert_tyoe::<_, Arc<Trans<str>>>(&foo);
        assert_eq!(foo.0, *"hello");

        assert_tyoe::<_, Arc<str>>(&Trans::into_inner_arc(Trans::from_inner_arc(make_arc())));
        assert_eq!(&*Trans::into_inner_arc(foo), "hello");
    }

    {
        let foo = Trans::from_inner_rc(make_rc());
        assert_tyoe::<_, Rc<Trans<str>>>(&foo);
        assert_eq!(foo.0, *"hello");

        assert_tyoe::<_, Rc<str>>(&Trans::from_inner_rc(make_rc()).into_inner_rc());
        assert_eq!(&*foo.into_inner_rc(), "hello");
    }
    #[cfg(feature = "rust_1_46")]
    {
        let foo = Trans::from_inner_rc(make_rc());
        assert_tyoe::<_, Rc<Trans<str>>>(&foo);
        assert_eq!(foo.0, *"hello");

        assert_tyoe::<_, Rc<str>>(&Trans::into_inner_rc(Trans::from_inner_rc(make_rc())));
        assert_eq!(&*Trans::into_inner_rc(foo), "hello");
    }
}