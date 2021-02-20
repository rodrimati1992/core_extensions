//! Extension traits for integers and types used in the traits.
//!
//!
//!

use std_::{cmp, hash::Hash, fmt, ops};

#[cfg(all(not(core_duration), feature = "std"))]
use std_::time::Duration;
#[cfg(core_duration)]
use std_::time::Duration;

/// Extension trait for built-in integers.
pub trait IntegerExt:
    'static
    + cmp::Eq
    + cmp::Ord
    + Copy
    + Default
    + fmt::Binary
    + fmt::Debug
    + fmt::Display
    + fmt::LowerHex
    + fmt::Octal
    + fmt::UpperHex
    + Hash
    + ops::Add<Self, Output = Self>
    + ops::AddAssign<Self>
    + ops::BitAnd<Self, Output = Self>
    + ops::BitAndAssign<Self>
    + ops::BitOr<Self, Output = Self>
    + ops::BitOrAssign<Self>
    + ops::BitXor<Self, Output = Self>
    + ops::BitXorAssign<Self>
    + ops::Div<Self, Output = Self>
    + ops::DivAssign<Self>
    + ops::Mul<Self, Output = Self>
    + ops::MulAssign<Self>
    + ops::Not<Output = Self>
    + ops::Rem<Self, Output = Self>
    + ops::RemAssign<Self>
    + ops::Shl<Self, Output = Self>
    + ops::ShlAssign<Self>
    + ops::Shr<Self, Output = Self>
    + ops::ShrAssign<Self>
    + ops::Sub<Self, Output = Self>
    + ops::SubAssign<Self>
    + Send
    + Sync
{
    /// The unsigned version of this integer type.
    type Unsigned: IntegerExt;

    /// The minimum value of this integer type.
    const MIN: Self;
    
    /// The maximum value of this integer type.
    const MAX: Self;
    
    /// `0` of this integer type.
    const ZERO: Self;
    
    /// `1` of this integer type.
    const ONE: Self;

    /// Converts from a `u8` to `Self`.
    ///
    /// if `Self` is an `i8` this method returns `127` for `n > 127`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::IntegerExt;
    ///
    /// assert_eq!(u8::from_u8(0  ), 0);
    /// assert_eq!(u8::from_u8(255), 255);
    ///
    /// assert_eq!(i8::from_u8(0  ), 0);
    /// assert_eq!(i8::from_u8(255), 127);
    ///
    /// assert_eq!(u16::from_u8(0  ), 0);
    /// assert_eq!(u16::from_u8(255), 255);
    ///
    /// assert_eq!(i16::from_u8(0  ), 0);
    /// assert_eq!(i16::from_u8(255), 255);
    ///
    /// ```
    fn from_u8(n: u8) -> Self;

    /// Converts from an `i8` to `Self`.
    ///
    /// if `Self` is an unsigned integer type,
    /// this method returns `0` for `n < 0`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::IntegerExt;
    ///
    /// assert_eq!(u8::from_i8(-128), 0);
    /// assert_eq!(u8::from_i8(0   ), 0);
    /// assert_eq!(u8::from_i8(1   ), 1);
    /// assert_eq!(u8::from_i8(127 ), 127);
    ///
    /// assert_eq!(i8::from_i8(-128), -128);
    /// assert_eq!(i8::from_i8(0   ), 0   );
    /// assert_eq!(i8::from_i8(127 ), 127 );
    ///
    /// assert_eq!(u16::from_i8(-128), 0);
    /// assert_eq!(u16::from_i8(0   ), 0);
    /// assert_eq!(u16::from_i8(1   ), 1);
    /// assert_eq!(u16::from_i8(127 ), 127);
    ///
    /// assert_eq!(i16::from_i8(-128), -128);
    /// assert_eq!(i16::from_i8(0   ), 0   );
    /// assert_eq!(i16::from_i8(127 ), 127 );
    ///
    ///
    /// ```
    fn from_i8(n: i8) -> Self;

    /// Raises `self` to the `n`th power.
    /// 
    /// This delegates to the inherent [`pow`] method.
    /// 
    /// [`pow`]: https://doc.rust-lang.org/std/primitive.u32.html#method.pow
    fn power(self, n: u32) -> Self;

    /// Returns the absolute value of this integer as the equivalent unsigned integer type.
    ///
    /// This method allows getting the absolute value for the minimum signed integer value.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::IntegerExt;
    ///
    /// assert_eq!(0u8.abs_unsigned(), 0u8);
    /// assert_eq!(0i8.abs_unsigned(), 0u8);
    /// assert_eq!(127i8.abs_unsigned(), 127u8);
    /// assert_eq!((-1i8).abs_unsigned(), 1u8);
    /// assert_eq!((-16i8).abs_unsigned(), 16u8);
    /// assert_eq!((-128i8).abs_unsigned(), 128u8);
    ///
    /// ```
    ///
    fn abs_unsigned(self) -> Self::Unsigned;

    /// Gets the sign of this integer.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::integers::{IntegerExt, Sign};
    ///
    /// assert_eq!(0u8.get_sign(), Sign::Positive);
    /// assert_eq!(0i8.get_sign(), Sign::Positive);
    /// assert_eq!(127i8.get_sign(), Sign::Positive);
    /// assert_eq!((-1i8).get_sign(), Sign::Negative);
    /// assert_eq!((-128i8).get_sign(), Sign::Negative);
    ///
    ///
    /// ```
    ///
    #[inline]
    fn get_sign(self) -> Sign {
        if self < Self::ZERO {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }

    /// Non-panicking division which returns `self` when `other == 0`.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::IntegerExt;
    ///
    /// assert_eq!(60.safe_div(12), 5);
    /// assert_eq!(60.safe_div(30), 2);
    /// assert_eq!(60.safe_div(31), 1);
    ///
    /// assert_eq!(60.safe_div(0), 60);
    /// assert_eq!(13.safe_div(0), 13);
    ///
    /// ```
    ///
    ///
    #[inline]
    fn safe_div(self, other: Self) -> Self {
        if other == Self::ZERO {
            self
        } else {
            self / other
        }
    }

    /// Returns the number of decimal digits of `self`.
    ///
    /// This counts the `-` sign as a digit.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::IntegerExt;
    ///
    /// assert_eq!(100.number_of_digits(), 3);
    /// assert_eq!(10.number_of_digits(), 2);
    /// assert_eq!(1.number_of_digits(), 1);
    /// assert_eq!(0.number_of_digits(), 1);
    /// assert_eq!((-1).number_of_digits(), 2);
    /// assert_eq!((-100).number_of_digits(), 4);
    ///
    /// ```
    ///
    fn number_of_digits(self) -> u32;
}

/// Converts an integer to a Duration of the unit.
///
#[cfg(any(core_duration, feature = "std"))]
pub trait ToTime {
    /// Creates a [`Duration`] of `self` hours.
    ///
    /// [`Duration`]: https://doc.rust-lang.org/core/time/struct.Duration.html
    /// # Example
    ///
    /// ```
    /// use core_extensions::ToTime;
    ///
    /// use std::time::Duration;
    ///
    /// assert_eq!(1  .hours(), Duration::from_secs(1  *3600));
    /// assert_eq!(10 .hours(), Duration::from_secs(10 *3600));
    /// assert_eq!(101.hours(), Duration::from_secs(101*3600));
    /// ```
    fn hours(self) -> Duration;
    /// Creates a [`Duration`] of `self` minutes.
    ///
    /// [`Duration`]: https://doc.rust-lang.org/core/time/struct.Duration.html
    /// # Example
    ///
    /// ```
    /// use core_extensions::ToTime;
    ///
    /// use std::time::Duration;
    ///
    /// assert_eq!(1  .minutes(), Duration::from_secs(1  *60));
    /// assert_eq!(10 .minutes(), Duration::from_secs(10 *60));
    /// assert_eq!(101.minutes(), Duration::from_secs(101*60));
    /// ```
    fn minutes(self) -> Duration;
    /// Creates a [`Duration`] of `self` seconds
    ///
    /// [`Duration`]: https://doc.rust-lang.org/core/time/struct.Duration.html
    /// # Example
    ///
    /// ```
    /// use core_extensions::ToTime;
    ///
    /// use std::time::Duration;
    ///
    /// assert_eq!(1.seconds(), Duration::from_secs(1));
    /// assert_eq!(10.seconds(), Duration::from_secs(10));
    /// assert_eq!(101.seconds(), Duration::from_secs(101));
    /// ```
    fn seconds(self) -> Duration;
    /// Creates a [`Duration`] of `self` miliseconds
    ///
    /// [`Duration`]: https://doc.rust-lang.org/core/time/struct.Duration.html
    /// # Example
    ///
    /// ```
    /// use core_extensions::ToTime;
    ///
    /// use std::time::Duration;
    ///
    /// assert_eq!(0.miliseconds(), Duration::from_millis(0));
    /// assert_eq!(1.miliseconds(), Duration::from_millis(1));
    /// assert_eq!(10.miliseconds(), Duration::from_millis(10));
    ///
    /// ```
    fn miliseconds(self) -> Duration;
    /// Creates a [`Duration`] of `self` microseconds
    ///
    /// [`Duration`]: https://doc.rust-lang.org/core/time/struct.Duration.html
    /// # Example
    ///
    /// ```
    /// use core_extensions::ToTime;
    ///
    /// use std::time::Duration;
    ///
    /// assert_eq!(10.microseconds(), Duration::new(0,10_000));
    /// assert_eq!(10_000_001.microseconds(), Duration::new(10,1_000));
    ///
    /// ```
    fn microseconds(self) -> Duration;
    /// Creates a [`Duration`] of `self` nanoseconds
    ///
    /// [`Duration`]: https://doc.rust-lang.org/core/time/struct.Duration.html
    /// # Example
    ///
    /// ```
    /// use core_extensions::ToTime;
    ///
    /// use std::time::Duration;
    ///
    /// assert_eq!(10.nanoseconds(), Duration::new(0,10));
    /// assert_eq!(1_000_000.nanoseconds(), Duration::new(0,1_000_000));
    /// assert_eq!(1_000_000_000.nanoseconds(), Duration::new(1,0));
    /// assert_eq!(1_000_001_000.nanoseconds(), Duration::new(1,1_000));
    ///
    /// ```
    fn nanoseconds(self) -> Duration;
}

#[cfg(any(core_duration, feature = "std"))]
impl<T> ToTime for T
where
    T: IntegerExt + Copy,
    <T as IntegerExt>::Unsigned: Into<u64>,
{
    fn hours(self) -> Duration {
        Duration::from_secs(self.abs_unsigned().into() * 3600)
    }
    fn minutes(self) -> Duration {
        Duration::from_secs(self.abs_unsigned().into() * 60)
    }
    fn seconds(self) -> Duration {
        Duration::from_secs(self.abs_unsigned().into())
    }
    fn miliseconds(self) -> Duration {
        Duration::from_millis(self.abs_unsigned().into())
    }
    fn microseconds(self) -> Duration {
        let number: u64 = self.abs_unsigned().into();
        Duration::new(number / 1_000_000, (number % 1_000_000 * 1000) as u32)
    }
    fn nanoseconds(self) -> Duration {
        let number: u64 = self.abs_unsigned().into();
        Duration::new(number / 1_000_000_000, (number % 1_000_000_000) as u32)
    }
}

//------------------------------------------------------------------------------------

/// Represents the signedness of an integer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    /// Positive integer
    Positive = 0,
    /// Negative integer
    Negative = 1,
}

impl Sign {
    /// How long the string representation of this sign is.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::integers::Sign;
    ///
    /// assert_eq!(Sign::Positive.sign_len(), 0);
    /// assert_eq!(Sign::Negative.sign_len(), 1);
    /// ```
    ///
    #[inline]
    pub const fn sign_len(self) -> usize {
        self as _
    }
    /// The string representation of this sign.
    ///
    /// # Example
    ///
    /// ```
    /// use core_extensions::integers::Sign;
    ///
    /// assert_eq!(Sign::Positive.sign_string(), "");
    /// assert_eq!(Sign::Negative.sign_string(), "-");
    /// ```
    ///
    #[inline]
    pub const fn sign_string(self) -> &'static str {
        cfg_if!{
            (feature = "rust_1_46") {
                match self {
                    Sign::Positive => "",
                    Sign::Negative => "-",
                }
            } else {
                ["", "-"][self as usize]
            }
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.sign_string(), f)
    }
}

//---------------------------------- IMPLS -------------------------------------------

macro_rules! impl_absolute_unsigned_numbers {
    (from_u8;8,signed)=>{
        #[inline(always)]
        fn from_u8(n:u8)->Self{
            cmp::min(n,127) as _
        }
    };
    (from_u8;$($_bits:tt)*)=>{
        #[inline(always)]
        fn from_u8(n:u8)->Self{
            n as _
        }
    };
    (from_i8;unsigned)=>{
        #[inline(always)]
        fn from_i8(n:i8)->Self{
            cmp::max(n,0) as _
        }
    };
    (from_i8;signed)=>{
        #[inline(always)]
        fn from_i8(n:i8)->Self{
            n as _
        }
    };
    (num number_of_digits;delegate $n:ident $len:ident)=>{
        $n.number_of_digits()
    };
    (num number_of_digits;128 $n:ident $len:ident)=>{{
        if $n >= 1_0000_0000_0000_0000{$n /= 1_0000_0000_0000_0000; $len += 16;}
        impl_absolute_unsigned_numbers!(num number_of_digits;64 $n $len)
    }};
    (num number_of_digits;64 $n:ident $len:ident)=>{{
        if $n >= 1_0000_0000_0000{$n /= 1_0000_0000_0000; $len += 12;}
        impl_absolute_unsigned_numbers!(num number_of_digits;32 $n $len)
    }};
    (num number_of_digits;32 $n:ident $len:ident)=>{{
        if $n >= 1_0000_0000{$n /= 100_000_000; $len += 8;}
        impl_absolute_unsigned_numbers!(num number_of_digits;16 $n $len)
    }};
    (num number_of_digits;16 $n:ident $len:ident)=>{{
        if $n >= 1_0000{$n /= 1_0000; $len += 4;}
        impl_absolute_unsigned_numbers!(num number_of_digits;8 $n $len)
    }};
    (num number_of_digits;8 $n:ident $len:ident)=>{{
        if $n >= 100{$n /= 100; $len += 2;}
        if $n >= 10{            $len += 1;}
        $len
    }};

    (impl_either;
        type=$This:ty ,
        bits=$bits:tt ,
        $(cast=$cast_to:ty,)*
    )=>{
        #[allow(unused_mut,unused_variables)]
        fn number_of_digits(self)-> u32 {
            let mut n=self.abs_unsigned()  $(as $cast_to)* ;
            let mut len=self.get_sign().sign_len() as u32+1;
            impl_absolute_unsigned_numbers!(num number_of_digits;$bits n len)
        }
        #[inline]
        fn power(self,n:u32)->Self{
            self.pow(n)
        }

    };

    (  $([
            $tsig:ty,
            $tuns:ty,
            bits=$bits:tt ,
            $(cast=$cast_to:ty,)*
            $(#[$meta:meta])*
        ])*
    ) => {$(
        $(#[$meta])*
        impl IntegerExt for $tsig {
            type Unsigned = $tuns;
            #[inline]
            fn abs_unsigned(self) -> Self::Unsigned {
                // using this instead of self.abs() to avoid
                // panicking on self==Self::min_value()
                (if self < 0 { self.wrapping_neg() }else{ self }) as Self::Unsigned
            }

            const ZERO: Self = 0;

            const ONE: Self = 1;

            const MIN: Self = Self::min_value();

            const MAX: Self = Self::max_value();

            impl_absolute_unsigned_numbers!{impl_either;
                type=$tsig,
                bits=$bits,
                $(cast=$cast_to,)*
            }
            impl_absolute_unsigned_numbers!{from_u8;$bits,signed}
            impl_absolute_unsigned_numbers!{from_i8;signed}
        }

        $(#[$meta])*
        impl IntegerExt for $tuns {
            type Unsigned = $tuns;
            #[inline]
            fn abs_unsigned(self) -> Self::Unsigned {
                self
            }

            const ZERO: Self = 0;

            const ONE: Self = 1;

            const MIN: Self = Self::min_value();

            const MAX: Self = Self::max_value();

            impl_absolute_unsigned_numbers!{impl_either;
                type=$tuns,
                bits=$bits,
                $(cast=$cast_to,)*
            }

            impl_absolute_unsigned_numbers!{from_u8;$bits,unsigned}
            impl_absolute_unsigned_numbers!{from_i8;unsigned}
        }

    )*}
}

#[cfg(target_pointer_width = "8")]
type UWord = u8;
#[cfg(target_pointer_width = "16")]
type UWord = u16;
#[cfg(target_pointer_width = "32")]
type UWord = u32;
#[cfg(target_pointer_width = "64")]
type UWord = u64;
#[cfg(target_pointer_width = "128")]
type UWord = u128;

impl_absolute_unsigned_numbers!(
    [i8 ,u8 ,bits=8, ]
    [i16,u16,bits=16,]
    [i32,u32,bits=32,]
    [i64,u64,bits=64,]
    [i128,u128,bits=128,]
    [isize,usize,bits=delegate,cast=UWord,]
);

//---------------------------------- TESTS  -------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use alloc_::vec::Vec;

    const MAX_POWER: u32 = 38;

    fn check_number_of_digits<I, N>(iter: I)
    where
        I: IntoIterator<Item = (N, u32)>,
        N: PartialEq + fmt::Display + Default + Copy + IntegerExt,
    {
        for (n, digits) in iter {
            println!("n:{} digits:{}", n, digits);
            assert_eq!(n.number_of_digits(), digits, " n:{} ", n);
        }
    }

    fn generate_numbers() -> Vec<(u128, u32)> {
        let ten: u128 = 10;
        let mut out: Vec<(u128, u32)> = vec![(0, 1), (1, 1), (9, 1)];

        for power in 1..MAX_POWER {
            let digits = power + 1;
            out.push((ten.pow(power), digits));
            out.push((ten.pow(power) + 1, digits));
            out.push((ten.pow(power + 1) - 1, digits));
        }
        out.push((ten.pow(MAX_POWER), MAX_POWER + 1));
        out.push((ten.pow(MAX_POWER) + 1, MAX_POWER + 1));
        out.push((u128::max_value(), MAX_POWER + 1));

        out
    }

    macro_rules! check_number_of_digits_ {
        ($($ty:ty),*) => {
            $(
                check_number_of_digits(
                    generate_numbers()
                        .into_iter()
                        .filter(|v| v.0 <= (<$ty>::max_value() as u128))
                        .map(|v| (v.0 as $ty, v.1)),
                );
            )*
        };
    }

    #[test]
    fn number_of_digits_i8() {
        check_number_of_digits_!(i8, u8, i16, u16, i32, u32, u64, i64, usize, isize, u128, i128);
    }

    macro_rules! check_assoc_consts {
        ($($ty:ty),*) => {
            $({
                assert_eq!(<$ty as IntegerExt>::MIN, <$ty>::min_value());

                assert_eq!(<$ty as IntegerExt>::MAX, <$ty>::max_value());

                assert_eq!(<$ty as IntegerExt>::ZERO, 0);

                assert_eq!(<$ty as IntegerExt>::ONE, 1);
            })*
        };
    }

    #[test]
    fn associated_constants() {
        check_assoc_consts!(i8, u8, i16, u16, i32, u32, u64, i64, usize, isize, u128, i128);
    }
}
