//! `PhantomData`-related items.
//!

use std_::{
    cell::Cell,
    marker::PhantomData,
};

/// Type alias for a variant `PhantomData` with drop check.
/// 
/// # Example
/// 
/// ```rust
/// use core_extensions::VariantDropPhantom;
/// use std::marker::PhantomData;
/// 
/// let _: VariantDropPhantom<u32> = PhantomData;
/// 
/// ```
/// 
pub type VariantDropPhantom<T> = PhantomData<T>;

/// Type alias for a variant `PhantomData`, without drop check.
/// 
/// # Example
/// 
/// ```rust
/// use core_extensions::{AsPhantomData, CovariantPhantom};
/// 
/// let _: CovariantPhantom<u32> = u32::PHANTOM_COVARIANT;
/// 
/// ```
/// 
pub type CovariantPhantom<T> = PhantomData<fn() -> T>;

/// Type alias for a contravariant `PhantomData`, without drop check.
/// 
/// # Example
/// 
/// ```rust
/// use core_extensions::{ContraVariantPhantom, AsPhantomData};
/// 
/// let _: ContraVariantPhantom<u32> = u32::PHANTOM_CONTRA;
/// 
/// ```
/// 
pub type ContraVariantPhantom<T> = PhantomData<fn(T)>;

/// Type alias for an invariant `PhantomData`.
/// 
/// # Example
/// 
/// ```rust
/// use core_extensions::{InvariantPhantom, AsPhantomData};
/// 
/// let _: InvariantPhantom<u32> = u32::PHANTOM_INVARIANT;
/// 
/// ```
/// 
pub type InvariantPhantom<T> = PhantomData<fn(T) -> T>;

/// Type alias for an `PhantomData` with an invariant lifetime.
/// 
/// # Example
/// 
/// ```rust
/// use core_extensions::InvariantRefPhantom;
/// use std::marker::PhantomData;
/// 
/// let _: InvariantRefPhantom<u32> = PhantomData;
/// 
/// ```
/// 
pub type InvariantRefPhantom<'a, T> = PhantomData<Cell<&'a T>>;


///////////////////////////////////////////////////////////////////////////


/// For getting a `PhantomData<Self>` with a variety of lifetime variances.
pub trait AsPhantomData {
    #[doc(hidden)]
    const PHANTOM_QFEO7CXJP2HJSGYWRZFRBHDTHU: PhantomData<Self> = PhantomData;

    /// Gets a `PhantomData<Self>`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use core_extensions::AsPhantomData;
    /// 
    /// use std::marker::PhantomData;
    /// 
    /// fn get_default<T: Default>(_type: PhantomData<T>) -> T {
    ///     Default::default()
    /// }
    /// 
    /// let string = String::new();
    /// let vector = vec![0u8];
    /// 
    /// assert_eq!(get_default(string.as_phantom()), "");
    /// assert_eq!(get_default(vector.as_phantom()), vec![]);
    /// 
    /// ```
    #[inline(always)]
    fn as_phantom(&self) -> PhantomData<Self> {
        PhantomData
    }

    /// Gets a `PhantomData<fn() -> Self>`, a covariant `PhantomData`.
    #[inline(always)]
    fn as_phantom_covariant(&self) -> PhantomData<fn() -> Self> {
        PhantomData
    }

    /// Gets a `PhantomData<fn(Self)>`, a contravariant `PhantomData`.
    #[inline(always)]
    fn as_phantom_contra(&self) -> PhantomData<fn(Self)> {
        PhantomData
    }

    /// Gets a `PhantomData<fn(Self) -> Self>`, an invariant `PhantomData`.
    #[inline(always)]
    fn as_phantom_invariant(&self) -> PhantomData<fn(Self) -> Self> {
        PhantomData
    }

    /// Gets a `PhantomData<Self>`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use core_extensions::AsPhantomData;
    /// 
    /// use std::marker::PhantomData;
    /// 
    /// fn get_default<T: Default>(_type: PhantomData<T>) -> T {
    ///     Default::default()
    /// }
    /// 
    /// assert_eq!(get_default(String::PHANTOM), "");
    /// assert_eq!(get_default(Vec::<()>::PHANTOM), vec![]);
    /// 
    /// ```
    const PHANTOM: PhantomData<Self> = PhantomData;

    /// Constructs a `PhantomData<fn() -> T>`, a covariant `PhantomData`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use core_extensions::{AsPhantomData, CovariantPhantom};
    /// 
    /// struct WithGhost<T> {
    ///     value: T,
    ///     _ghost: CovariantPhantom<T>,
    /// }
    /// 
    /// impl<T> WithGhost<T> {
    ///     const fn new(value: T) -> Self {
    ///         Self {
    ///             value,
    ///             _ghost: T::PHANTOM_COVARIANT,
    ///         }
    ///     }
    /// }
    /// ```
    /// 
    const PHANTOM_COVARIANT: PhantomData<fn() -> Self> = PhantomData;
    
    /// Gets a `PhantomData<fn(Self)>`, a contravariant `PhantomData`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use core_extensions::{AsPhantomData, ContraVariantPhantom};
    /// 
    /// struct WithGhost<T> {
    ///     value: T,
    ///     _ghost: ContraVariantPhantom<T>,
    /// }
    /// 
    /// impl<T> WithGhost<T> {
    ///     const fn new(value: T) -> Self {
    ///         Self {
    ///             value,
    ///             _ghost: T::PHANTOM_CONTRA,
    ///         }
    ///     }
    /// }
    /// ```
    /// 
    const PHANTOM_CONTRA: PhantomData<fn(Self)> = PhantomData;

    /// Gets a `PhantomData<fn(Self) -> Self>`, an invariant `PhantomData`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use core_extensions::{AsPhantomData, InvariantPhantom};
    /// 
    /// struct WithGhost<T> {
    ///     value: T,
    ///     _ghost: InvariantPhantom<T>,
    /// }
    /// 
    /// impl<T> WithGhost<T> {
    ///     const fn new(value: T) -> Self {
    ///         Self {
    ///             value,
    ///             _ghost: T::PHANTOM_INVARIANT,
    ///         }
    ///     }
    /// }
    /// ```
    /// 
    const PHANTOM_INVARIANT: PhantomData<fn(Self) -> Self> = PhantomData;
}

impl<T: ?Sized> AsPhantomData for T {}


///////////////////////////////////////////////////////////////////////////


/// Gets the `PhantomData` of the passed in type.
/// 
/// # Example
/// 
/// ```rust
/// use core_extensions::as_phantom;
/// 
/// use std::marker::PhantomData;
/// 
/// fn get_default<T: Default>(_type: PhantomData<T>) -> T {
///     Default::default()
/// }
/// 
/// let string = String::new();
/// let vector = vec![0u8];
/// 
/// assert_eq!(get_default(as_phantom(&string)), "".to_string());
/// assert_eq!(get_default(as_phantom(&vector)), vec![]);
/// 
/// ```
#[inline(always)]
pub const fn as_phantom<T: ?Sized>(_: &T) -> PhantomData<T> {
    PhantomData
}


///////////////////////////////////////////////////////////////////////////

/// Contains `PhantomData<fn() -> T>`,
/// required to return a `PhantomData<fn() -> T>` from the 
/// [`as_covariant_phantom`] const function.
/// 
/// [`as_covariant_phantom`]: ./fn.as_covariant_phantom.html
/// 
#[must_use = "unwrap this into a PhantomData with .0"]
pub struct CovariantPhantomData<T: ?Sized>(pub PhantomData<fn() -> T>);

impl<T: ?Sized> CovariantPhantomData<T> {
    /// Constructs a `CovariantPhantomData<T>`
    pub const NEW: Self = Self(PhantomData);
}


/// Gets the `PhantomData<fn() -> T>` of the passed in `T`.
///
/// # Example
/// 
#[cfg_attr(feature = "const_default", doc = " ```rust")]
#[cfg_attr(not(feature = "const_default"), doc = " ```ignore")]
/// use core_extensions::{AndPhantomCov, ConstDefault, as_covariant_phantom};
/// 
/// const SLICE: &[u8] = {
///     let array = [0, 1, 2];
///     
///     // phantom is a PhantomData<fn() -> [i32; 3]>;
///     let phantom = as_covariant_phantom(&array).0;
///     
///     &AndPhantomCov(phantom, ConstDefault::DEFAULT).1
/// };
/// 
/// 
/// assert_eq!(SLICE, [0, 0, 0]);
/// 
/// ```
/// 
pub const fn as_covariant_phantom<T: ?Sized>(_: &T) -> CovariantPhantomData<T> {
    CovariantPhantomData::NEW
}


///////////////////////////////////////////////////////////////////////////


/// A pair of `PhantomData<T>` and `T`.
/// useful for infering the type of the value from a `PhantomData`.
///
/// # Example
///
/// ```rust
/// use core_extensions::{AndPhantom, AsPhantomData};
/// 
/// use std::marker::PhantomData;
/// 
/// let foo = vec![0, 1, 2];
/// 
/// let mut bar = AndPhantom(foo.as_phantom(), Default::default()).1;
/// 
/// bar.push(3);
/// bar.push(4);
/// 
/// assert_eq!(bar[..], [3, 4]);
///
/// ```
#[repr(transparent)]
pub struct AndPhantom<T>(pub PhantomData<T>, pub T);




/// A pair of `PhantomData<fn() -> T>` and `T`.
/// useful for infering the type of the value from a `PhantomData`.
///
/// # Example
///
/// ```rust
/// use core_extensions::{AndPhantomCov, AsPhantomData};
/// 
/// use std::marker::PhantomData;
/// 
/// let foo = [0, 1, 2];
/// 
/// let mut bar = AndPhantomCov(foo.as_phantom_covariant(), Default::default()).1;
/// 
/// bar[0] = 3;
/// bar[1] = 5;
/// bar[2] = 8;
/// 
/// assert_eq!(bar[..], [3, 5, 8]);
///
/// ```
#[repr(transparent)]
pub struct AndPhantomCov<T>(pub PhantomData<fn() -> T>, pub T);

