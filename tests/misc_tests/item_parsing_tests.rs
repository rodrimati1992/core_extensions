

#[test]
fn split_trait_tests() {
    assert_is!{
        impl_split
        {aa bb}
        (
            #[this is]
            const unsafe impl Foo<u32> for u64<> ; hello world
        )
        (
            aa bb
            (#[this is])
            (const unsafe)
            ()
            trait( Foo<u32> )
            type(u64<>)
            ()
            (; hello world)
        )
    }
    assert_is!{
        impl_split
        {aa bb}
        (
            #[this is]
            impl<'a, T: X> const Foo<u32> for Vec<N, {100}> 
            where;
            hello world
        )
        (
            aa bb
            (#[this is])
            ()
            ('a, T: X)
            trait( const Foo<u32> )
            type(Vec<N, {100}>)
            ()
            (; hello world)
        )
    }
}



#[test]
fn split_type_tests() {
    assert_is!{
        impl_split
        {aa bb}
        (
            impl Vec<N, {100}> 
            where
                T: Foo
            = hello foo bar baz
        )
        (
            aa bb
            ()
            ()
            ()
            type(Vec<N, {100}>)
            (T: Foo,)
            ( = hello foo bar baz )
        )
    }
    assert_is!{
        impl_split
        {aa bb}
        (
            foo #[bar] bar
            impl for<N> Vec<N, {100}> 
            where
                T: Foo<'a, T, {100}>,
            { hello }
            foo bar baz
        )
        (
            aa bb
            ()
            (foo #[bar] bar)
            ()
            type( for<N> Vec<N, {100}> )
            (T: Foo<'a, T, {100}>,)
            (
                { hello }
                foo bar baz
            )
        )
    }
    assert_is!{
        impl_split
        {aa bb}
        (
            foo #[bar] bar
            impl<> dyn for<N> XXX<N, {100}> 
            where
                T: Foo<'a, T, {100}>,
            { hello }
            foo bar baz
        )
        (
            aa bb
            ()
            (foo #[bar] bar)
            ()
            type( dyn for<N> XXX<N, {100}> )
            (T: Foo<'a, T, {100}>,)
            (
                { hello }
                foo bar baz
            )
        )
    }
}


#[test]
fn parse_generics_test() {
    assert_is!{
        impl_parse_generics
        {aa bb}
        (
            #[bar]
            const impl<T: Hi> Trait for Foo {} A B
        )
        (
            aa bb
            (#[bar])
            (const)
            (()(T:(Hi +),)())
            trait(Trait)
            type(Foo)
            ()
            ({} A B)
        )
    }
    assert_is!{
        impl_parse_generics
        {aa bb}
        (
            impl Foo where T: Hi {} A B
        )
        (
            aa bb
            ()
            ()
            (()()())
            type(Foo)
            (T: Hi,)
            ({} A B)
        )
    }
}














