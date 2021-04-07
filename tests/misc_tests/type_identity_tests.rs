use krate::TypeIdentity;

#[test]
fn test_core() {
    let make = ||"hello".to_string();
    
    {
        let foo: String = make().into_type();
        // making sure that the return type isn't generic
        let _ = make().into_type();
        let _ = <String as TypeIdentity>::into_type(make());
        

        assert_eq!(foo, "hello");
        assert_eq!(<String>::from_type(make()), "hello");
    }
    {
        let foo: &'static str = "hello".as_type();
        // making sure that the return type isn't generic
        let _ = "hello".as_type();
        let _ = <str>::as_type("hello");
        
        assert_eq!(foo, "hello");
        assert_eq!(<str as TypeIdentity>::from_type_ref(foo), "hello");
    }
    {
        let mut arr = [3, 5, 8, 13, 21];
        
        // making sure that the return type isn't generic
        let _  = (&mut arr[..3]).as_type_mut();

        let _: &mut [u8]  = <[u8] as TypeIdentity>::as_type_mut(&mut arr[..3]);

        let foo: &mut [u8]  = (&mut arr[..3]).as_type_mut();
        
        assert_eq!(foo, &mut [3, 5, 8][..]);
        assert_eq!(<[u8]>::from_type_mut(foo), &mut [3, 5, 8][..]);
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
        let foo: Box<str> = make_box().into_type_box();
        // making sure that the return type isn't generic
        let _ = make_box().into_type_box();
        let _ = <str as TypeIdentity>::into_type_box(make_box());

        assert_eq!(foo, make_box());
        assert_eq!(<str>::from_type_box(foo), make_box());
    }

    {
        let foo: Arc<str> = TypeIdentity::into_type_arc(make_arc());
        // making sure that the return type isn't generic
        let _ = TypeIdentity::into_type_arc(make_arc());
        let _ = <str as TypeIdentity>::into_type_arc(make_arc());

        assert_eq!(foo, make_arc());
        assert_eq!(<str>::from_type_arc(foo), make_arc());
    }
    #[cfg(feature = "rust_1_46")]
    {
        let foo: Arc<str> = make_arc().into_type_arc();
        // making sure that the return type isn't generic
        let _ = make_arc().into_type_arc();

        assert_eq!(foo, make_arc());
    }    

    {
        let foo: Rc<str> = TypeIdentity::into_type_rc(make_rc());
        // making sure that the return type isn't generic
        let _ = TypeIdentity::into_type_rc(make_rc());
        let _ = <str as TypeIdentity>::into_type_rc(make_rc());

        assert_eq!(foo, make_rc());
        assert_eq!(<str>::from_type_rc(foo), make_rc());
    }
    #[cfg(feature = "rust_1_46")]
    {
        let foo: Rc<str> = make_rc().into_type_rc();
        // making sure that the return type isn't generic
        let _ = make_rc().into_type_rc();
        
        assert_eq!(foo, make_rc());
    }
}