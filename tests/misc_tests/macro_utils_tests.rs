use krate::rewrap_macro_parameters;

macro_rules! identity {
    ($macro:ident ! {$($tokens:tt)*}) => {
        $macro ! {$($tokens)*}
    };
}

#[test]
fn different_delimiters() {
    macro_rules! assertion_1 {
        (
            (hello world)
            [foo bar]
            {aaa bbb ccc}
            (1 + 1) 
            (fn foo(){})
            (Vec<u32>)
            (didnt)
        ) => {
            pub const S: &'static str = "matched";
        };
        ($($tt:tt)*) => { compile_error!{stringify!($($tt)*)} };
    }

    macro_rules! assertion_2 {
        (($($t:tt)*)) => { pub const S: &'static str = "no match"; };
        (1 + 1) => {
            pub const S: &'static str = "matched";
        };
    }

    macro_rules! somemacro {
        (
            $hello:expr,
            $item:item,
            $ty:ty,
            $ident:ident,
        ) => {
            mod foo {
                use super::*;
                rewrap_macro_parameters!{
                    assertion_1 ! {
                        ~(hello world) ~[foo bar] ~{aaa bbb ccc} 
                        ~$hello 
                        ~$item
                        ~$ty
                        ~$ident
                    }
                }

            }
            assert_eq!(foo::S, "matched");
            
            mod bar{
                use super::*;
                rewrap_macro_parameters!{assertion_2!{$hello}}
            }
            assert_eq!(bar::S, "matched");
        };
    }

    somemacro!{
        1 + 1,
        fn foo() {},
        Vec<u32>,
        didnt,
    }

}

