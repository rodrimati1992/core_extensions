//! Type-level assertions, most useful for tests.

use crate::TypeIdentity;

use std_::marker::PhantomData;


/// Asserts that its 2 type parameters are the same type.
///
/// This assertion is done on the type level,
/// so `let _: AssertEq<T, U>;` requires that `T` must be the same type as `U`.
///
/// # Example
///
/// ```
/// use core_extensions::type_asserts::AssertEq;
///
/// trait ElemTy {
///     type Elem;
/// }
///
/// impl<A> ElemTy for (A,) {
///     type Elem = A;
/// }
///
/// let _: AssertEq<u32, <(u32,) as ElemTy>::Elem>;
///
/// let _ = AssertEq::new(&0u32, &0u32);
///
/// ```
///
/// # Non-compiling
///
/// ```compile_fail
/// use core_extensions::type_asserts::AssertEq;
///
/// let _: AssertEq<(), u32>;
/// let _ = AssertEq::new(&(), &0u32);
///
/// ```
///
/// ```compile_fail
/// use core_extensions::type_asserts::AssertEq;
///
/// let _: AssertEq<u32, ()>;
/// let _ = AssertEq::new(&0u32, &());
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
    /// Constructs an `AssertEq`.
    pub fn new(_: A, _: A)->Self{
        Self{_marker: PhantomData}
    }

    /// Constructs an `AssertEq`.
    pub const NEW: Self = Self{_marker: PhantomData};
}


/// Asserts that its 3 type parameters are the same type.
///
/// This assertion is done on the type level,
/// so `let _: AssertEq3<A, B, C>;` requires that `A`, `B`, and `C` must be the same type.
///
/// # Example
///
/// ```
/// use core_extensions::type_asserts::AssertEq3;
///
/// trait TypeParams {
///     type First;
///     type Second;
/// }
///
/// impl<A, B> TypeParams for (A, B) {
///     type First = A;
///     type Second = B;
/// }
///
/// type First<T> = <T as TypeParams>::First;
///
/// type Second<T> = <T as TypeParams>::Second;
///
/// let _: AssertEq3<u32, First<(u32, ())>, Second<((), u32)>>;
///
/// let _ = AssertEq3::new(&0u32, &0u32, &0u32);
///
/// ```
///
/// # Non-compiling
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq3;
/// let _: AssertEq3<(), u32, u32>;
/// let _ = AssertEq3::new(&(), &0u32, &0u32);
/// ```
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq3;
/// let _: AssertEq3<u32, (), u32>;
/// let _ = AssertEq3::new(&0u32, &(), &0u32);
/// ```
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq3;
/// let _: AssertEq3<u32, u32, ()>;
/// let _ = AssertEq3::new(&0u32, &0u32, &());
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
    /// Constructs an `AssertEq3`.
    pub fn new(_: A, _: A, _: A)->Self{
        Self{_marker: PhantomData}
    }
    /// Constructs an `AssertEq3`.
    pub const NEW: Self = Self{_marker: PhantomData};
}


/// Asserts that its 4 type parameters are the same type.
///
/// This assertion is done on the type level,
/// so `let _: AssertEq4<A, B, C, D>;` requires that 
/// `A`, `B`, `C`, and `D` must be the same type.
///
/// # Example
///
/// ```
/// use core_extensions::type_asserts::AssertEq4;
///
/// trait TypeParams {
///     type First;
///     type Second;
///     type Third;
/// }
///
/// impl<A, B, C> TypeParams for (A, B, C) {
///     type First = A;
///     type Second = B;
///     type Third = C;
/// }
///
/// type First<T> = <T as TypeParams>::First;
///
/// type Second<T> = <T as TypeParams>::Second;
///
/// type Third<T> = <T as TypeParams>::Third;
///
/// let _: AssertEq4<
///     u32,
///     First <(u32, (), ())>,
///     Second<((), u32, ())>,
///     Third <((), (), u32)>,
/// >;
///
/// let _ = AssertEq4::new(&0u32, &0u32, &0u32, &0u32);
///
/// ```
///
/// # Non-compiling
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq4;
/// let _: AssertEq4<(), u32, u32, u32>;
/// let _ = AssertEq4::new(&(), &0u32, &0u32, &0u32);
/// ```
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq4;
/// let _: AssertEq4<u32, (), u32, u32>;
/// let _ = AssertEq4::new(&0u32, &(), &0u32, &0u32);
/// ```
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq4;
/// let _: AssertEq4<u32, u32, (), u32>;
/// let _ = AssertEq4::new(&0u32, &0u32, &(), &0u32);
/// ```
///
/// ```compile_fail
/// # use core_extensions::type_asserts::AssertEq4;
/// let _: AssertEq4<u32, u32, u32, ()>;
/// let _ = AssertEq4::new(&0u32, &0u32, &0u32, &());
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
    /// Constructs an `AssertEq4`.
    pub fn new(_: A, _: A, _: A, _: A)->Self{
        Self{_marker: PhantomData}
    }
    
    /// Constructs an `AssertEq4`.
    pub const NEW: Self = Self{_marker: PhantomData};
}


////////////////////////////////////////////////////////////////////////////////

