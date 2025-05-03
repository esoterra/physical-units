use core::fmt;
use std::ops::{Add, Div, Mul, Sub};

use thiserror::Error;

use crate::Unit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value<Number> {
    unit: Unit,
    number: Number
}

impl<Number> fmt::Display for Value<Number>
where
    Number: fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.number, self.unit)
    }
}

#[derive(Error, Debug)]
#[error("Unit '{lhs}' didn't match '{rhs}'")]
pub struct UnitMismatch {
    pub lhs: Unit,
    pub rhs: Unit
}

impl<Number> Add for Value<Number>
where
    Number: Add<Output = Number>
{
    type Output = Result<Self, UnitMismatch>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Ok(Self {
                unit: self.unit,
                number: self.number + rhs.number,
            })
        } else {
            Err(UnitMismatch { lhs: self.unit, rhs: rhs.unit })
        }
    }
}

impl<Number> Sub for Value<Number>
where
    Number: Sub<Output = Number>
{
    type Output = Result<Self, UnitMismatch>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Ok(Self {
                unit: self.unit,
                number: self.number - rhs.number,
            })
        } else {
            Err(UnitMismatch { lhs: self.unit, rhs: rhs.unit })
        }
    }
}

impl<Number> Mul for Value<Number>
where
    Number: Mul<Output = Number>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            unit: self.unit * rhs.unit,
            number: self.number * rhs.number,
        }
    }
}

impl<Number> Div for Value<Number>
where
    Number: Div<Output = Number>
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            unit: self.unit / rhs.unit,
            number: self.number / rhs.number,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base;
    use super::*;

    #[test]
    fn test_display() {
        // Ints
        let one_second = Value { unit: base::SECOND, number: 1u32 };
        assert_eq!(String::from("1 s"), format!("{}", one_second));
        let two_seconds = Value { unit: base::SECOND, number: 2u32 };
        assert_eq!(String::from("2 s"), format!("{}", two_seconds));
        // Floats
        let one_second = Value { unit: base::SECOND, number: 1f32 };
        assert_eq!(String::from("1 s"), format!("{}", one_second));
        let two_seconds = Value { unit: base::SECOND, number: 2f32 };
        assert_eq!(String::from("2 s"), format!("{}", two_seconds));
    }

    #[test]
    fn test_addition() {
        // Ints
        let one_second = Value { unit: base::SECOND, number: 1u32 };
        let two_seconds = Value { unit: base::SECOND, number: 2u32 };
        assert_eq!(two_seconds, (one_second + one_second).unwrap());
        // Floats
        let one_second = Value { unit: base::SECOND, number: 1f32 };
        let two_seconds = Value { unit: base::SECOND, number: 2f32 };
        assert_eq!(two_seconds, (one_second + one_second).unwrap());
    }

    #[test]
    fn test_subtraction() {
        // Ints
        let one_second = Value { unit: base::SECOND, number: 1u32 };
        let two_seconds = Value { unit: base::SECOND, number: 2u32 };
        assert_eq!(one_second, (two_seconds - one_second).unwrap());
        // Floats
        let one_second = Value { unit: base::SECOND, number: 1f32 };
        let two_seconds = Value { unit: base::SECOND, number: 2f32 };
        assert_eq!(one_second, (two_seconds - one_second).unwrap());
    }
}