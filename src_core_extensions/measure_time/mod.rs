//! Time measurement.
//!

use std_::time::Duration;

/// Measures the time taken by `f` to execute, returning a pair of `(Duration, T)`.
#[inline(never)]
pub fn measure<F, T>(f: F) -> (Duration, T)
where
    F: FnOnce() -> T,
{
    let now = ::std_::time::Instant::now();
    let ret = f();
    let duration = now.elapsed();
    let microseconds = Duration::from(duration);
    (microseconds, ret)
}

/// Measures the time taken by fallible function `f` to execute,
/// returning a pair of `Result<(Duration, T), E>`,
/// so that this function can be used in combination with `?`.
#[inline(never)]
pub fn try_measure<F, T, E>(f: F) -> Result<(Duration, T), E>
where
    F: FnOnce() -> Result<T, E>,
{
    match measure(f) {
        (_, Err(e)) => Err(e),
        (t, Ok(v)) => Ok((t, v)),
    }
}

