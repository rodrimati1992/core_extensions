use crate::misc_tests::utils_tests::remove_whitespace;

macro_rules! stringify_no_whitespace {
    ($($tokens:tt)*) => {
        pub const S: &str = stringify!($($tokens)*);
    };
}

macro_rules! psg {
    ($prefix:tt $suffix:tt) => {{
        mod fooo {
            krate::parse_split_generics!{
                stringify_no_whitespace!$prefix
                $suffix
            }
        }
        remove_whitespace(fooo::S)
    }};
}

macro_rules! assert_pg {
    ($prefix:tt $suffix:tt $output:tt) => {{
        mod fooo {
            macro_rules! assertions {
                $output => {};
            }

            krate::parse_generics!{
                assertions!$prefix
                $suffix
            }
        }
    }};
}


#[test]
fn lifetimes() {
    assert_eq!(
        psg!({aa bb}('a: 'b, 'c, 'd: 'e + 'f, 'g: 'h +, T, const U: usize)),
        remove_whitespace("
            aabb
            (
                ('a:('b +))  ('c:())  ('d:('e + 'f +))  ('g:('h +))
                (type T:(),)
                (const U: usize,) 
            )
            (('a:('b +), 'c:(), 'd:('e + 'f +), 'g:('h +),) (T:(),) (U: usize,))
        ")
    );


    assert_pg!{
        {aa bb}
        ('a: 'b, 'c, 'd: 'e + 'f, 'g: 'h +, T, const U: usize)
        (
            aa bb
            ('a: 'b + , 'c, 'd: 'e + 'f + , 'g: 'h +, T, const U: $c0:ty,)
            ('a: 'b + , 'c, 'd: 'e + 'f + , 'g: 'h +, T, const U: $c1:ty,)
            ('a, 'c, 'd, 'g, T, U,)
            ($($phantom:tt)*)
        )
    }
}

#[test]
fn types() {
    assert_eq!(
        psg!({aa bb}('a, T, U = hi, V: 'a + (hello world) + Trait<F> = foo, W: www)),
        remove_whitespace("
            aabb
            (
                ('a:())
                (type T:(),)
                (type U:() = hi,)
                (type V:('a + (hello world) + Trait<F> +) = foo,)
                (type W:(www +),)
            )
            (
                ('a:(),)
                (
                    T:(),
                    U:() = hi,
                    V: ('a + (hello world) + Trait<F> +) = foo,
                    W:(www +),
                )
                ()
            )
        ")
    );
    assert_pg!{
        {aa bb}
        ('a, T, U = hi, V: 'a + (hello world) + Trait<F> = foo, W: www)
        (
            aa bb
            ('a, T, U = $uty:ty, V: 'a + (hello world) + Trait<F> + = $foo0:ty, W: www +,)
            ('a, T, U, V: 'a + (hello world) + Trait<F> +, W: www +,)
            ('a, T, U, V, W,)
            ($($phantom:tt)*)
        )
    }
}




#[test]
fn consts() {
    assert_eq!(
        psg!({aa bb}('a, T, const A: usize, U, const B: usize = 100)),
        remove_whitespace("
            aabb
            (
                ('a:())
                (type T:(),)
                (const A: usize,)
                (type U:(),)
                (const B: usize = 100,)
            )
            (
                ('a:(),)
                (T:(), U:(),)
                (A: usize, B: usize = 100,)
            )
        ")
    );
    assert_pg!{
        {aa bb}
        ('a, T, const A: usize, U, const B: usize = 100)
        (
            aa bb
            ('a, T, const A: $usize0:ty, U, const B: $usize1:ty = $def:expr,)
            ('a, T, const A: $usize4:ty, U, const B: $usize5:ty,)
            ('a, T, A, U, B,)
            ($($phantom:tt)*)
        )
    }
}
























