use crate::misc_tests::utils_tests::remove_whitespace;

macro_rules! psg {
    ($prefix:tt $suffix:tt) => {{
        mod fooo {
            krate::parse_split_generics!{
                stringify_no_whitespace!$prefix
                $suffix
            }
        }
        mod baaar {
            krate::parse_split_generics!{
                $crate::stringify_no_whitespace!$prefix
                $suffix
            }
        }
        assert_eq!(fooo::S, baaar::S);
        remove_whitespace(fooo::S)
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


    assert_is!{
        parse_generics
        {aa bb}
        ('a: 'b, 'c, 'd: 'e + 'f, 'g: 'h +, T, const U: usize)
        (
            aa bb
            ('a: 'b + , 'c, 'd: 'e + 'f + , 'g: 'h +, T, const U: usize,)
            ('a: 'b + , 'c, 'd: 'e + 'f + , 'g: 'h +, T, const U: usize,)
            ('a, 'c, 'd, 'g, T, U,)
            ($crate::__::PD<(&'a(),&'c(),&'d(),&'g(),$crate::__::PD<T>,)>)
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
    assert_is!{
        parse_generics
        {aa bb}
        ('a, T, U = hi, V: 'a + (hello world) + Trait<F> = foo, W: www)
        (
            aa bb
            ('a, T, U = hi, V: 'a + (hello world) + Trait<F> + = foo, W: www +,)
            ('a, T, U, V: 'a + (hello world) + Trait<F> +, W: www +,)
            ('a, T, U, V, W,)
            (
                $crate::__::PD<(
                    &'a(),
                    $crate::__::PD<T>,
                    $crate::__::PD<U>,
                    $crate::__::PD<V>,
                    $crate::__::PD<W>,
                )>
            )
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
    assert_is!{
        parse_generics
        {aa bb}
        ('a, T, const A: usize, U, const B: usize = 100)
        (
            aa bb
            ('a, T, const A: usize, U, const B: usize = 100,)
            ('a, T, const A: usize, U, const B: usize,)
            ('a, T, A, U, B,)
            ($crate::__::PD<(&'a(),$crate::__::PD<T>,$crate::__::PD<U>,)>)
        )
    }
}



#[test]
fn parse_generics_and_where_test() {
    assert_is!{
        parse_generics_and_where
        {aa bb}
        (
            <'a, T: Foo = A, const N: usize>
            (Foo, Bar, Baz)
            where
                T: Bar;
        )
        (
            aa bb
            ('a, T: Foo + = A, const N: usize,)
            ('a, T: Foo + , const N: usize,)
            ('a, T, N,)
            ($crate::__::PD<(&'a(),$crate::__::PD<T>,)>)
            ((Foo, Bar, Baz))
            (T: Bar,)
            ( ; )
        )
    }
}

#[test]
fn parse_split_generics_test() {
    assert_is!{
        parse_split_generics
        {aa bb}
        ('a, 'b: 'a, T: Foo = A, U, const N: usize, const M: bool)
        (
            aa bb
            (
                ('a: ())
                ('b: ('a +))
                (type T: (Foo +) = A,)
                (type U: (),)
                (const N: usize,)
                (const M: bool,)
            )
            (
                ('a: (), 'b: ('a +),)
                (T: (Foo+) = A, U: (),)
                (N: usize, M: bool,)
            )
        )
    }
}


#[test]
fn parse_split_generics_and_where_test() {
    assert_is!{
        parse_split_generics_and_where
        {aa bb}
        (
            <'a, T: Foo = A, const N: usize>
            (Foo, Bar, Baz)
            where
                T: Bar;
        )
        (
            aa bb
            (
                ('a: ())
                (type T: (Foo+) = A,)
                (const N: usize,)
            )
            (
                ('a: (),)
                (T: (Foo+) = A,)
                (N: usize,)
            )

            ((Foo, Bar, Baz))
            (T: Bar,)
            ( ; )
        )
    }
}



#[test]
fn split_generics_and_where_test() {
    assert_is!{
        split_generics_and_where
        {aa bb}
        (
            <'a, T: Foo = A, const N: usize>
            (Foo, Bar, Baz)
            where
                T: Bar;
        )
        (
            aa bb
            ('a, T: Foo = A, const N: usize)
            ((Foo, Bar, Baz))
            (T: Bar,)
            ( ; )
        )
    }
}























