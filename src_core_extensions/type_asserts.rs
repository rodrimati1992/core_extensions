/*!
Assertions done on the type-level,mostly for tests.
*/

use TypeIdentity;

use std_::marker::PhantomData;


/// Struct used to assert that its type parameters are the same type.
///
/// This is most useful in tests,to make sure that types are the same.
///
/// # Example
///
/// ```
/// use core_extensions::type_asserts::AssertEq;
///
/// let _:AssertEq<u32,u32>;
/// let _=AssertEq::new( &0u32, &0u32 );
///
/// ```
///
/// # Non-compiling
///
/// ```compile_fail
/// use core_extensions::type_asserts::AssertEq;
///
/// let _:AssertEq<(),u32>;
/// let _=AssertEq::new( &(), &0u32 );
///
/// ```
///
/// ```compile_fail
/// use core_extensions::type_asserts::AssertEq;
///
/// let _:AssertEq<u32,()>;
/// let _=AssertEq::new( &0u32, &() );
///
/// ```
///
pub struct AssertEq<L:?Sized,R:?Sized>
where L:TypeIdentity<Type=R>
{
    _marker:PhantomData<(
        PhantomData<L>,
        PhantomData<R>,
    )>,
}

impl<A> AssertEq<A,A>{
    /// Constructs an AssertEq.
    pub fn new(_:A,_:A)->Self{
        Self{_marker:PhantomData}
    }
}


/// Struct used to assert that its type parameters are the same type.
///
/// # Example
///
/// ```
/// use core_extensions::type_asserts::AssertEq3;
///
/// let _:AssertEq3<u32,u32,u32>;
/// let _=AssertEq3::new( &0u32, &0u32, &0u32 );
///
/// ```
///
/// # Non-compiling
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq3;
/// let _:AssertEq3<(),u32,u32>;
/// let _=AssertEq3::new( &(), &0u32, &0u32 );
/// ```
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq3;
/// let _:AssertEq3<u32,(),u32>;
/// let _=AssertEq3::new( &0u32, &(), &0u32 );
/// ```
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq3;
/// let _:AssertEq3<u32,u32,()>;
/// let _=AssertEq3::new( &0u32, &0u32, &() );
/// ```
///
pub struct AssertEq3<A:?Sized,B:?Sized,C:?Sized>
where 
    A:TypeIdentity<Type=B>,
    A:TypeIdentity<Type=C>,
{
    _marker:PhantomData<(
        PhantomData<A>,
        PhantomData<B>,
        PhantomData<C>,
    )>,
}

impl<A> AssertEq3<A,A,A>{
    /// Constructs an AssertEq3.
    pub fn new(_:A,_:A,_:A)->Self{
        Self{_marker:PhantomData}
    }
}


/// Struct used to assert that its type parameters are the same type.
///
/// # Example
///
/// ```
/// use core_extensions::type_asserts::AssertEq4;
///
/// let _:AssertEq4<u32,u32,u32,u32>;
/// let _=AssertEq4::new( &0u32, &0u32, &0u32, &0u32 );
///
/// ```
///
/// # Non-compiling
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq4;
/// let _:AssertEq4<(),u32,u32,u32>;
/// let _=AssertEq4::new( &(), &0u32, &0u32, &0u32 );
/// ```
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq4;
/// let _:AssertEq4<u32,(),u32,u32>;
/// let _=AssertEq4::new( &0u32, &(), &0u32, &0u32 );
/// ```
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq4;
/// let _:AssertEq4<u32,u32,(),u32>;
/// let _=AssertEq4::new( &0u32, &0u32, &(), &0u32 );
/// ```
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq4;
/// let _:AssertEq4<u32,u32,u32,()>;
/// let _=AssertEq4::new( &0u32, &0u32, &0u32, &() );
/// ```
///
pub struct AssertEq4<A:?Sized,B:?Sized,C:?Sized,D:?Sized>
where 
    A:TypeIdentity<Type=B>,
    A:TypeIdentity<Type=C>,
    A:TypeIdentity<Type=D>,
{
    _marker:PhantomData<(
        PhantomData<A>,
        PhantomData<B>,
        PhantomData<C>,
        PhantomData<D>,
    )>,
}

impl<A> AssertEq4<A,A,A,A>{
    /// Constructs an AssertEq4.
    pub fn new(_:A,_:A,_:A,_:A)->Self{
        Self{_marker:PhantomData}
    }
}


////////////////////////////////////////////////////////////////////////////////

