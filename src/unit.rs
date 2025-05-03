use core::fmt;
use std::ops::{Div, Mul};

use crate::base;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Unit {
    /// kilogram (kg)
    pub(crate) kilogram: i8,
    /// meter (m)
    pub(crate) meter: i8,
    /// second (s)
    pub(crate) second: i8,
    /// mole (mol)
    pub(crate) mole: i8,
    /// ampere (A)
    pub(crate) ampere: i8,
    /// kelvin (K)
    pub(crate) kelvin: i8,
    /// candela (cd)
    pub(crate) candela: i8,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == base::UNITLESS {
            return write!(f, "unitless");
        }

        let array = [
            (self.kilogram, "kg"),
            (self.meter, "m"),
            (self.second, "s"),
            (self.mole, "mol"),
            (self.ampere, "A"),
            (self.kelvin, "K"),
            (self.candela, "cd")
        ];

        let mut had_superscript = true;
        for (n, symbol) in array.iter() {
            let n = *n;
            if n > 0 {
                if !had_superscript {
                    write!(f, " ")?;
                }
                write!(f, "{symbol}{}", SignedSuperscript { n })?;
                had_superscript = n > 1;
            }
        }

        let negatives = array.iter().filter(|(n, _)| *n < 0).count();
        if negatives != 0 {
            write!(f, " / ")?;

            if negatives > 1 {
                write!(f, "(")?;
            }

            let mut had_superscript = true;
            for (n, symbol) in array.iter() {
                let n = -*n;
                if n > 0 {
                    if !had_superscript {
                        write!(f, " ")?;
                    }
                    write!(f, "{symbol}{}", SignedSuperscript { n })?;
                    had_superscript = n > 1;
                }
            }

            if negatives > 1 {
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

struct SignedSuperscript {
    n: i8
}

impl fmt::Display for SignedSuperscript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.n == 0 {
            return write!(f, "⁰");
        }
        if self.n == 1 {
            return Ok(());
        }

        // Handle negatives
        let n = if self.n < 0 {
            write!(f, "⁻")?;
            -self.n as u8
        } else {
            self.n as u8
        };

        // Numbers are in the range of 1-128
        // so we can use a simple version of decimal conversion
        let hundreds = n / 100; // Will always be zero or 1
        let n = n - 100 * hundreds;
        let tens = n / 10; // Will be 0-9
        let n = n - 10 * tens;
        let ones = n; // Will be 0-9
        
        if hundreds != 0 {
            SuperscriptDigit { digit: hundreds }.fmt(f)?;
        }
        if hundreds != 0 || tens != 0 {
            SuperscriptDigit { digit: tens }.fmt(f)?;
        }
        SuperscriptDigit { digit: ones }.fmt(f)?;
        Ok(())
    }
}

struct SuperscriptDigit {
    digit: u8
}

impl fmt::Display for SuperscriptDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.digit {
            0 => write!(f, "⁰"),
            1 => write!(f, "¹"),
            2 => write!(f, "²"),
            3 => write!(f, "³"),
            4 => write!(f, "⁴"),
            5 => write!(f, "⁵"),
            6 => write!(f, "⁶"),
            7 => write!(f, "⁷"),
            8 => write!(f, "⁸"),
            9 => write!(f, "⁹"),
            _ => unreachable!()
        }
    }
}

impl Unit {
    pub const fn multiply(self, other: Self) -> Self {
        Self {
            meter: self.meter + other.meter,
            second: self.second + other.second,
            mole: self.mole + other.mole,
            ampere: self.ampere + other.ampere,
            kelvin: self.kelvin + other.kelvin,
            candela: self.candela + other.candela,
            kilogram: self.kilogram + other.kilogram,
        }
    }

    pub const fn divide(self, other: Self) -> Self {
        Self {
            meter: self.meter - other.meter,
            second: self.second - other.second,
            mole: self.mole - other.mole,
            ampere: self.ampere - other.ampere,
            kelvin: self.kelvin - other.kelvin,
            candela: self.candela - other.candela,
            kilogram: self.kilogram - other.kilogram,
        }
    }
}

impl Mul for Unit {
    type Output = Unit;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Self) -> Self::Output {
        self.divide(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let unit = Unit {
            meter: 1,
            second: -2,
            mole: 0,
            ampere: 0,
            kelvin: 0,
            candela: 0,
            kilogram: 1,
        };
        assert_eq!(String::from("kg m / s²"), format!("{}", unit));
    }
}