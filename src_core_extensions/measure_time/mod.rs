//! Time measurement,including functions and types.
//!

use std_::fmt;
use std_::ops::{Deref, DerefMut};

#[cfg(any(enable_duration, feature = "std"))]
use std_::time::Duration;

use integer_extensions::ToTime;
#[allow(unused_imports)]
use SelfOps;

#[cfg(feature="serde_")]
mod serde_duration_expanded_impls;


/// Wrapper type for ::std::time::Duration which is
/// specialized for measuring code execution time.
///
/// This type implements the [Units](../formatting/trait.Units.html) trait,used for displaying
/// types with many units.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MyDuration(pub Duration);

impl MyDuration {
    /// Creates a MyDuration from `hours` hours.
    pub fn from_hours(hours: u64) -> Self {
        MyDuration(hours.hours())
    }
    /// Creates a MyDuration from `minutes` minutes.
    pub fn from_minutes(minutes: u64) -> Self {
        MyDuration(minutes.minutes())
    }
    /// Creates a MyDuration from `seconds` seconds.
    pub fn from_seconds(seconds: u64) -> Self {
        MyDuration(seconds.seconds())
    }
    /// Creates a MyDuration from `miliseconds` miliseconds.
    pub fn from_mili(miliseconds: u64) -> Self {
        MyDuration(miliseconds.miliseconds())
    }
    /// Creates a MyDuration from `microseconds` microseconds.
    pub fn from_micro(microseconds: u64) -> Self {
        MyDuration(microseconds.microseconds())
    }
    /// Creates a MyDuration from `nanoseconds` nanoseconds.
    pub fn from_nano(nanoseconds: u64) -> Self {
        MyDuration(nanoseconds.nanoseconds())
    }

    /// How many nanoseconds this is.
    pub fn nanoseconds(self) -> u64 {
        self.0.subsec_nanos() as u64 + self.0.as_secs() * 1_000_000_000
    }
    /// How many microseconds this is.
    pub fn microseconds(self) -> u64 {
        self.0.subsec_nanos() as u64 / 1000 + self.0.as_secs() * 1_000_000
    }
    /// How many miliseconds this is.
    pub fn miliseconds(self) -> u64 {
        self.0.subsec_nanos() as u64 / 1_000_000 + self.0.as_secs() * 1_000
    }
    /// How many seconds this is.
    pub fn seconds(self) -> u64 {
        self.0.as_secs()
    }
    /// How many minutes this is.
    pub fn minutes(self) -> u64 {
        self.0.as_secs() / 60
    }
    /// How many hours this is.
    pub fn hours(self) -> u64 {
        self.0.as_secs() / 3600
    }

    /// Maps the underlying Duration.
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(Duration) -> Duration,
    {
        f(self.into()).into()
    }
}

#[cfg(feature = "serde_")]
mod duration_serde{
    use super::*;
    use super::serde_duration_expanded_impls::SerdeDuration;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<'de> Deserialize<'de> for MyDuration {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            SerdeDuration::deserialize(deserializer)?
                .piped(|d|Duration::new(d.seconds,d.subsec_nanos))
                .piped(MyDuration)
                .piped(Ok)
        }
    }

    /// This impl is only enabled if the "serde_" feature is enabled.
    ///
    impl Serialize for MyDuration {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            SerdeDuration{
                subsec_nanos:self.0.subsec_nanos(),
                seconds:self.0.as_secs(),
            }.serialize(serializer)
        }
    }
}





impl Deref for MyDuration {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MyDuration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for MyDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fn split(dec: &mut u64, dur: &mut u64, factor: u64) -> bool {
            let old_dur = *dur;
            *dec = old_dur % factor;
            *dur = old_dur / factor;
            *dur < factor
        }

        let mut decimal = 0;
        let mut dur = self.nanoseconds();

        let unit = if dur < 1000 {
            "ns"
        } else if split(&mut decimal, &mut dur, 1000) {
            "μs"
        } else if split(&mut decimal, &mut dur, 1000) {
            "ms"
        } else {
            split(&mut decimal, &mut dur, 1000);
            "s"
        };

        let mut dec_digits = 3;

        loop {
            if decimal == 0 {
                dec_digits = 0;
                break;
            } else if decimal % 10 == 0 {
                decimal /= 10;
                dec_digits -= 1;
            } else {
                break;
            }
        }

        write!(f, "{}", dur)?;
        if decimal != 0 {
            write!(
                f,
                ".{:0decimal_digits$}",
                decimal,
                decimal_digits = dec_digits
            )?;
        }
        write!(f, "{}", unit)?;
        Ok(())
    }
}

impl From<Duration> for MyDuration {
    fn from(from: Duration) -> Self {
        MyDuration(from)
    }
}

impl From<MyDuration> for Duration {
    fn from(this: MyDuration) -> Duration {
        this.0
    }
}

/// Measures the time taken by `f` to execute, returning a pair of (`MyDuration`,`T`).
#[inline(never)]
#[cfg(feature = "std")]
pub fn measure<F, T>(f: F) -> (MyDuration, T)
where
    F: FnOnce() -> T,
{
    let now = std_::time::Instant::now();
    let ret = f();
    let duration = now.elapsed();
    let microseconds = MyDuration::from(duration);
    (microseconds, ret)
}

/// Measures the time taken by fallible function `f` to execute,
/// returning a pair of Result<(`MyDuration`,`T`),E>,
/// so that this function can be used in combination with `?`.
#[inline(never)]
#[cfg(feature = "std")]
pub fn try_measure<F, T, E>(f: F) -> Result<(MyDuration, T), E>
where
    F: FnOnce() -> Result<T, E>,
{
    match measure(f) {
        (_, Err(e)) => Err(e),
        (t, Ok(v)) => Ok((t, v)),
    }
}

#[cfg(test)]
mod tests {
    use measure_time::MyDuration;

    use alloc_::string::{String,ToString};

    #[test]
    fn test_precision() {
        // use core_extensions::formatting::{write_unit,Precision};
        fn example(nanos: u64) -> String {
            MyDuration::from_nano(nanos).to_string()
        }

        assert_eq!(example(100), "100ns");
        assert_eq!(example(100), "100ns");
        assert_eq!(example(100), "100ns");
        assert_eq!(example(1000), "1μs");
        assert_eq!(example(1001), "1.001μs");
        assert_eq!(example(1010), "1.01μs");
        assert_eq!(example(1200), "1.2μs");
        assert_eq!(example(1230), "1.23μs");
        assert_eq!(example(1234), "1.234μs");
        assert_eq!(example(12340), "12.34μs");
        assert_eq!(example(12345), "12.345μs");
        assert_eq!(example(10), "10ns");
        assert_eq!(example(10), "10ns");
        assert_eq!(example(1_000_000_000), "1s");
        assert_eq!(example(1_200_000_000), "1.2s");
        assert_eq!(example(1_230_000_000), "1.23s");
        assert_eq!(example(12_300_000_000), "12.3s");
        assert_eq!(example(123_000_000_000), "123s");
        assert_eq!(example(123_567_000_000), "123.567s");
    }

}
