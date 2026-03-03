// Fixed Point Arithmetic

use core::fmt;
use std::{
    fmt::Debug,
    i8,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// A numeric value that can be used to represent an exponent of a unit
/// e.g. the 2 in meters squared (m^2).
///
/// It must support basic arithmetic with small numbers.
pub trait UnitExponent:
    Sized
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Clone
    + Copy
    + Default
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<i8, Output = Self>
    + Div<i8, Output = Self>
    + Debug
{
    const ZERO: Self;
    const ONE: Self;

    /// Takes an integer in the range (-32, 31) inclusive-inclusive
    /// and produces a FractionalExponent with that value.
    fn from_int(int: i8) -> Self;

    fn to_parts(self) -> ExponentParts;

    fn magnitude(self) -> u16;
}

pub struct ExponentParts {
    pub sign_positive: bool,
    pub whole_part: u8,
    pub percent_part: u8,
}

impl UnitExponent for i8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    fn from_int(int: i8) -> Self {
        int
    }

    fn to_parts(self) -> ExponentParts {
        ExponentParts {
            sign_positive: self >= 0,
            whole_part: self.unsigned_abs(),
            percent_part: 0,
        }
    }

    fn magnitude(self) -> u16 {
        self.unsigned_abs() as u16
    }
}

/// A signed 8 bit fixed point number with 2 fractional bits
/// that represents the exponent of a unit (e.g. m^2).
///
/// ## Unit Operations and Exponent Arithmetic
///
/// Since these values are semantically exponents of physical units,
/// operations on the units result in operations on the numeric
/// values of the exponents that follow the exponent rules.
///
/// * multiplying values adds unit exponents
///   * ex: 5 m^1 * 100 m^2 = 500 m^3 (since 1 + 2 = 3)
/// * dividing values subtracts unit exponents
///   * ex: (10 m^2) / (2 m^1) = 5 m^1 (since 2 - 1 = 1)
/// * raising a value to an exponent multiplies the unit exponent
///   * ex: (10 m^1)^2 = 100 m^2 (since 1 * 2 = 2)
/// * taking the root of a value divides the unit exponent
///   * ex: sqrt(400 m^2) = 20 m^1 (since 2 / 2 = 1)
///
/// **Note:** adding and subtracting values with the same units doesn't
/// doesn't require us to modify the units (e.g. 10m + 20m = 30m),
/// but it does require them to match.
///
/// ## Supported Exponent Arithmetic
///
/// * Addition and subtraction between unit exponents is fully supported and overflows
/// the same way as integers in Rust do by default.
/// * Unit exponents may be multiplied by unsigned integers and overflow normally.
/// * Unit exponents may be divided by unsigned integers, but division by zero panics and division must be exact
///
/// To keep thing simple, multiplication requires that one of the two values is an integer.
/// Since we only multiply two exponents when one is raised to the
///
/// ## Representation
///
/// It is represented as an i8 (signed 8-bit integer) and its value
/// is equal to the value of that i8 divided by 4.
///
/// | Bit Pattern | Value |
/// |-------------|-------|
/// | 0000 0000 | 0 |
/// | 0000 0001 | 1/4 |
/// | 0000 0010 | 1/2 |
/// | 0000 0011 | 3/4 |
/// | 0000 0100 | 1 |
/// | ... | ... |
/// | 0111 1111 | 31 3/4 |
/// | 1000 0000 | -32 |
/// | 1000 0001 | -31 3/4 |
/// | ... | ... |
/// | 1111 1101 | -1 |
/// | 1111 1101 | -3/4 |
/// | 1111 1110 | -1/2 |
/// | 1111 1111 | -1/4 |
///
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct FractionalExponent(i8);

pub const fn ue(int: i8) -> FractionalExponent {
    FractionalExponent(int * 4)
}

impl FractionalExponent {
    /// Takes an integer in the range (-32, 31) inclusive-inclusive
    /// and produces a FractionalExponent with that value.
    pub fn from_int(int: i8) -> Self {
        if int > 31 {
            panic!(
                "Integer {} is too high for 6-bit field in FractionalExponent (max: 31)",
                int
            );
        }
        if int < -32 {
            panic!(
                "Integer {} is too low for 6-bit field in FractionalExponent (min: -32)",
                int
            );
        }
        FractionalExponent(int * 4)
    }

    pub const fn const_add(self, rhs: Self) -> Self {
        FractionalExponent(self.0 + rhs.0)
    }

    pub const fn const_sub(self, rhs: Self) -> Self {
        FractionalExponent(self.0 - rhs.0)
    }

    pub const fn const_mult(self, rhs: i8) -> Self {
        Self(self.0 * rhs)
    }

    pub const fn const_div(self, rhs: i8) -> Self {
        if rhs == 0 {
            panic!("Attempted to divide by zero.")
        }
        let remainder = self.0 % rhs;
        // Only divide if it leaves no remainder
        if remainder == 0 {
            return Self(self.0 / rhs);
        }
        // Panic otherwise
        panic!("Attempted division that would leave a remainder.")
    }

    pub const fn magnitude(self) -> u16 {
        self.0.unsigned_abs() as u16
    }
}

impl UnitExponent for FractionalExponent {
    const ZERO: Self = Self(0);
    const ONE: Self = Self(1);

    fn from_int(int: i8) -> Self {
        Self::from_int(int)
    }

    fn to_parts(self) -> ExponentParts {
        let abs = self.0.unsigned_abs();
        ExponentParts {
            sign_positive: self.0 >= 0,
            whole_part: abs / 4,
            percent_part: (abs % 4) * 25,
        }
    }

    fn magnitude(self) -> u16 {
        self.magnitude()
    }
}

impl Default for FractionalExponent {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Neg for FractionalExponent {
    type Output = FractionalExponent;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for FractionalExponent {
    type Output = FractionalExponent;

    fn add(self, rhs: Self) -> Self::Output {
        self.const_add(rhs)
    }
}

impl Sub for FractionalExponent {
    type Output = FractionalExponent;

    fn sub(self, rhs: Self) -> Self::Output {
        self.const_sub(rhs)
    }
}

impl Mul<i8> for FractionalExponent {
    type Output = FractionalExponent;

    fn mul(self, rhs: i8) -> Self::Output {
        self.const_mult(rhs)
    }
}

impl Div<i8> for FractionalExponent {
    type Output = FractionalExponent;

    fn div(self, rhs: i8) -> Self::Output {
        self.const_div(rhs)
    }
}

struct FractionalExponentParts {
    sign_positive: bool,
    whole_part: u8,
    fraction_part: u8,
}

impl From<FractionalExponent> for FractionalExponentParts {
    fn from(value: FractionalExponent) -> Self {
        let sign_positive = value.0 > 0;
        let whole_part = (value.0.abs() / 4) as u8;
        let fraction_part = (value.0.abs() % 4) as u8;
        FractionalExponentParts {
            sign_positive,
            whole_part,
            fraction_part,
        }
    }
}

impl fmt::Debug for FractionalExponentParts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.sign_positive {
            write!(f, "-")?;
        }
        if self.whole_part != 0 {
            write!(f, "{}", self.whole_part)?;
            if self.fraction_part != 0 {
                write!(f, " ")?;
            }
        }
        match self.fraction_part {
            0 => {}
            1 => write!(f, "1/4")?,
            2 => write!(f, "1/2")?,
            3 => write!(f, "3/4")?,
            _ => unreachable!(),
        }
        Ok(())
    }
}

impl fmt::Debug for FractionalExponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts = FractionalExponentParts::from(*self);
        f.debug_tuple("FractionalExponent").field(&parts).finish()
    }
}

impl fmt::Display for FractionalExponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts = FractionalExponentParts::from(*self);
        parts.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_I8_VALUES: std::ops::RangeInclusive<i8> = i8::MIN..=i8::MAX;

    #[test]
    fn simple_fractions() {
        // Compute 1/4, 1/2, and 3/4 and verify their values.
        let quarter = ue(1) / 4;
        assert_eq!(quarter.0, 1);
        assert_eq!(format!("{}", quarter), "1/4");
        assert_eq!(quarter * 4, ue(1));

        let half = ue(1) / 2;
        assert_eq!(half.0, 2);
        assert_eq!(format!("{}", half), "1/2");
        assert_eq!(half * 2, ue(1));

        let three_quarters = ue(3) / 4;
        assert_eq!(three_quarters.0, 3);
        assert_eq!(format!("{}", three_quarters), "3/4");
        assert_eq!((three_quarters * 4) / 3, ue(1));

        // Form compound fractions and verify their values.
        let one_and_a_quarter = ue(1) + quarter;
        assert_eq!(one_and_a_quarter.0, 5);
        assert_eq!(format!("{}", one_and_a_quarter), "1 1/4");
        assert_eq!((one_and_a_quarter * 4) - ue(4), ue(1));

        let one_and_a_half = ue(1) + half;
        assert_eq!(one_and_a_half.0, 6);
        assert_eq!(format!("{}", one_and_a_half), "1 1/2");
        assert_eq!((one_and_a_half * 2) - ue(2), ue(1));

        let one_and_three_quarters = ue(1) + three_quarters;
        assert_eq!(one_and_three_quarters.0, 7);
        assert_eq!(format!("{}", one_and_three_quarters), "1 3/4");
        assert_eq!((one_and_three_quarters * 4) - ue(6), ue(1));

        // Compute -1/4, -1/2, and -3/4 and verify their values.
        let neg_quarter = -quarter;
        assert_eq!(neg_quarter.0, -1);
        assert_eq!(format!("{}", neg_quarter), "-1/4");
        assert_eq!(neg_quarter * 4, ue(-1));

        let neg_half = -half;
        assert_eq!(neg_half.0, -2);
        assert_eq!(format!("{}", neg_half), "-1/2");
        assert_eq!(neg_half * 2, ue(-1));

        let neg_three_quarters = -three_quarters;
        assert_eq!(neg_three_quarters.0, -3);
        assert_eq!(format!("{}", neg_three_quarters), "-3/4");
        assert_eq!((neg_three_quarters * 4) / 3, ue(-1));

        // Add positives and negatives together
        assert_eq!(quarter, half + neg_quarter);
        assert_eq!(neg_quarter, half + neg_three_quarters);

        // Check various identities
        assert_eq!(ue(1), quarter + quarter + quarter + quarter);
        assert_eq!(ue(1), three_quarters + quarter);
        assert_eq!(ue(1), half + half);

    }

    #[test]
    fn multiplication_identities() {
        for i in ALL_I8_VALUES {
            let value = FractionalExponent(i);
            assert_eq!(value, value * 1);
            assert_eq!(ue(0), value * 0);
        }
    }

    #[test]
    fn multiplication_integers() {
        // Sweeps over the whole range of integers that can be multiplied
        for i in 0..32i8 {
            for j in 0..32i8 {
                // Skips numbers that wouldn't fit
                if (i * 4).checked_mul(j * 4).is_none() {
                    continue;
                }
                assert_eq!(ue(i) * j, ue(i * j));
                assert_eq!(ue(-i) * j, ue(-(i * j)));
                assert_eq!(ue(i) * -j, ue(-(i * j)));
                assert_eq!(ue(-i) * -j, ue(i * j));
            }
        }
    }
}
