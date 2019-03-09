//! Contains many type aliases for PhantomData with different lifetime variances.
//!
//! These aliases can be constructed in these ways:
//!
//! - `PhantomData`
//!
//! - `Variance::<T>::default()`
//!
//! - `value.ty_()` constructs VariantPhantom
//! - `value.ty_d()` constructs VariantDropPhantom
//! - `value.ty_inv()` constructs InvariantPhantom
//! - `value. ty_inv_ref()` constructs InvariantRefPhantom
//!
//!
//!
//! Phantom type        lifetime variance       type variance
//!
//! VariantDropPhantom  -                       variant (with drop check)
//! VariantPhantom      -                       variant
//! InvariantRefPhantom invariant               -
//! InvariantPhantom    -                       invariant

use std_::cell::Cell;
use std_::marker::PhantomData;

/// Type alias for a variant PhantomData with drop check.
pub type VariantDropPhantom<T> = PhantomData<T>;

/// Type alias for a variant PhantomData withhout drop-check.
pub type VariantPhantom<T> = PhantomData<fn() -> T>;

/// Type alias for an invariant PhantomData.
pub type InvariantPhantom<T> = PhantomData<fn(T) -> T>;

/// Type alias for an PhantomData with an invariant lifetime.
pub type InvariantRefPhantom<'a, T> = PhantomData<Cell<&'a T>>;
