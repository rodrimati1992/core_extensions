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
/// use core_extensions::{MakePhantomFn, CovariantPhantom};
/// 
/// let _: CovariantPhantom<u32> = MakePhantomFn::COVARIANT;
/// 
/// ```
/// 
pub type CovariantPhantom<T> = PhantomData<fn() -> T>;

/// Type alias for a contravariant `PhantomData`, without drop check.
/// 
/// # Example
/// 
/// ```rust
/// use core_extensions::{ContraVariantPhantom, MakePhantomFn};
/// 
/// let _: ContraVariantPhantom<u32> = MakePhantomFn::CONTRA;
/// 
/// ```
/// 
pub type ContraVariantPhantom<T> = PhantomData<fn(T)>;

/// Type alias for an invariant `PhantomData`.
/// 
/// # Example
/// 
/// ```rust
/// use core_extensions::{InvariantPhantom, MakePhantomFn};
/// 
/// let _: InvariantPhantom<u32> = MakePhantomFn::INVARIANT;
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


/// For constructing a `PhantomData<fn(....) -> _>` inside `const fn`s.
pub struct MakePhantomFn<T: ?Sized>(T);

impl<T: ?Sized> MakePhantomFn<T> {
    /// Constructs a `PhantomData<fn() -> T>`
    /// a covariant `PhantomData`, without drop check.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use core_extensions::{MakePhantomFn, CovariantPhantom};
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
    ///             _ghost: MakePhantomFn::COVARIANT,
    ///         }
    ///     }
    /// }
    /// ```
    /// 
    pub const COVARIANT: PhantomData<fn() -> T> = PhantomData;

    /// Constructs a `PhantomData<fn(T) -> T>`,
    /// an invariant `PhantomData`, without drop check.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use core_extensions::{MakePhantomFn, InvariantPhantom};
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
    ///             _ghost: MakePhantomFn::INVARIANT,
    ///         }
    ///     }
    /// }
    /// ```
    /// 
    pub const INVARIANT: PhantomData<fn(T) -> T> = PhantomData;

    /// Constructs a `PhantomData<fn(T)>`
    /// a contravariant `PhantomData`, without drop check.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use core_extensions::{MakePhantomFn, ContraVariantPhantom};
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
    ///             _ghost: MakePhantomFn::CONTRA,
    ///         }
    ///     }
    /// }
    /// ```
    /// 
    pub const CONTRA: PhantomData<fn(T)> = PhantomData;
}

/// For getting the `PhantomData<Self>` with a variety of lifetime variances.
pub trait AsPhantomData {
    #[doc(hidden)]
    const PHANTOM_QFEO7CXJP2HJSGYWRZFRBHDTHU: PhantomData<Self> = PhantomData;

    /// Gets a `PhantomData<Self>`.
    #[inline(always)]
    fn as_phantom(&self) -> PhantomData<Self> {
        PhantomData
    }

    /// Gets a `PhantomData<fn() -> Self>`.
    #[inline(always)]
    fn as_phantom_covariant(&self) -> PhantomData<fn() -> Self> {
        PhantomData
    }

    /// Gets a `PhantomData<fn(Self)>`.
    #[inline(always)]
    fn as_phantom_contra(&self) -> PhantomData<fn(Self)> {
        PhantomData
    }

    /// Gets a `PhantomData<fn(Self) -> Self>`.
    #[inline(always)]
    fn as_phantom_invariant(&self) -> PhantomData<fn(Self) -> Self> {
        PhantomData
    }

    /// Gets a `PhantomData<Self>`.
    const PHANTOM: PhantomData<Self> = PhantomData;

    /// Gets a `PhantomData<fn() -> Self>`.
    const PHANTOM_COVARIANT: PhantomData<fn() -> Self> = PhantomData;
    
    /// Gets a `PhantomData<fn(Self)>`.
    const PHANTOM_CONTRA: PhantomData<fn(Self)> = PhantomData;

    /// Gets a `PhantomData<fn(Self) -> Self>`.
    const PHANTOM_INVARIANT: PhantomData<fn(Self) -> Self> = PhantomData;
}

impl<T: ?Sized> AsPhantomData for T {}



/// Gets the `PhantomData` of the passed in type.
#[inline(always)]
pub const fn as_phantomdata<T: ?Sized>(_: &T) -> PhantomData<T> {
    PhantomData
}

/// Converts a `PhantomData<T>` to a `PhantomData<fn() -> T>`
#[inline(always)]
pub fn to_covariant<T: ?Sized>(_: PhantomData<T>) -> PhantomData<fn() -> T> {
    MakePhantomFn::COVARIANT
}

/// Converts a `PhantomData<T>` to a `PhantomData<fn(T)>`
#[inline(always)]
pub fn to_contravariant<T: ?Sized>(_: PhantomData<T>) -> PhantomData<fn(T)> {
    MakePhantomFn::CONTRA
}

/// Converts a `PhantomData<T>` to a `PhantomData<fn(T) -> T>`
#[inline(always)]
pub fn to_invariant<T: ?Sized>(_: PhantomData<T>) -> PhantomData<fn(T) -> T> {
    MakePhantomFn::INVARIANT
}



