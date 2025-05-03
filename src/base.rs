use std::ops::{Add, Div, Mul, Sub};

use thiserror::Error;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BaseUnit {
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

impl BaseUnit {
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

    pub(crate) fn magnitude(self) -> u16 {
        self.meter.abs() as u16
            + self.second.abs() as u16
            + self.mole.abs() as u16
            + self.ampere.abs() as u16
            + self.kelvin.abs() as u16
            + self.candela.abs() as u16
            + self.kilogram.abs() as u16
    }
}

impl Mul for BaseUnit {
    type Output = BaseUnit;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Div for BaseUnit {
    type Output = BaseUnit;

    fn div(self, rhs: Self) -> Self::Output {
        self.divide(rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseValue<Number> {
    pub(crate) unit: BaseUnit,
    pub(crate) number: Number,
}

#[derive(Error, Debug)]
#[error("Unit '{lhs}' didn't match '{rhs}'")]
pub struct UnitMismatch {
    pub lhs: BaseUnit,
    pub rhs: BaseUnit,
}

impl<Number> Add for BaseValue<Number>
where
    Number: Add<Output = Number>,
{
    type Output = Result<Self, UnitMismatch>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Ok(Self {
                unit: self.unit,
                number: self.number + rhs.number,
            })
        } else {
            Err(UnitMismatch {
                lhs: self.unit,
                rhs: rhs.unit,
            })
        }
    }
}

impl<Number> Sub for BaseValue<Number>
where
    Number: Sub<Output = Number>,
{
    type Output = Result<Self, UnitMismatch>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Ok(Self {
                unit: self.unit,
                number: self.number - rhs.number,
            })
        } else {
            Err(UnitMismatch {
                lhs: self.unit,
                rhs: rhs.unit,
            })
        }
    }
}

impl<Number> Mul for BaseValue<Number>
where
    Number: Mul<Output = Number>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            unit: self.unit * rhs.unit,
            number: self.number * rhs.number,
        }
    }
}

impl<Number> Div for BaseValue<Number>
where
    Number: Div<Output = Number>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            unit: self.unit / rhs.unit,
            number: self.number / rhs.number,
        }
    }
}

pub const UNITLESS: BaseUnit = BaseUnit {
    meter: 0,
    second: 0,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 0,
};

/// meter (m)
pub const METER: BaseUnit = BaseUnit {
    meter: 1,
    ..UNITLESS
};

/// second (s)
pub const SECOND: BaseUnit = BaseUnit {
    meter: 0,
    second: 1,
    ..UNITLESS
};

/// mole (mol)
pub const MOLE: BaseUnit = BaseUnit {
    mole: 1,
    ..UNITLESS
};

/// ampere (A)
pub const AMPERE: BaseUnit = BaseUnit {
    ampere: 1,
    ..UNITLESS
};

/// kelvin (K)
pub const KELVIN: BaseUnit = BaseUnit {
    kelvin: 1,
    ..UNITLESS
};

/// candela (cd)
pub const CANDELA: BaseUnit = BaseUnit {
    candela: 1,
    ..UNITLESS
};

/// kilogram (kg)
pub const KILOGRAM: BaseUnit = BaseUnit {
    kilogram: 1,
    ..UNITLESS
};

pub const METER_SQ: BaseUnit = BaseUnit {
    meter: 2,
    ..UNITLESS
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        // Ints
        let one_second = BaseValue {
            unit: SECOND,
            number: 1u32,
        };
        let two_seconds = BaseValue {
            unit: SECOND,
            number: 2u32,
        };
        assert_eq!(two_seconds, (one_second + one_second).unwrap());
        // Floats
        let one_second = BaseValue {
            unit: SECOND,
            number: 1f32,
        };
        let two_seconds = BaseValue {
            unit: SECOND,
            number: 2f32,
        };
        assert_eq!(two_seconds, (one_second + one_second).unwrap());
    }

    #[test]
    fn test_subtraction() {
        // Ints
        let one_second = BaseValue {
            unit: SECOND,
            number: 1u32,
        };
        let two_seconds = BaseValue {
            unit: SECOND,
            number: 2u32,
        };
        assert_eq!(one_second, (two_seconds - one_second).unwrap());
        // Floats
        let one_second = BaseValue {
            unit: SECOND,
            number: 1f32,
        };
        let two_seconds = BaseValue {
            unit: SECOND,
            number: 2f32,
        };
        assert_eq!(one_second, (two_seconds - one_second).unwrap());
    }
}
