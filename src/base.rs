use core::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use thiserror::Error;

use crate::exponents::UnitExponent;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BaseUnit<ExponentType = i8> {
    /// kilogram (kg)
    pub(crate) kilogram: ExponentType,
    /// meter (m)
    pub(crate) meter: ExponentType,
    /// second (s)
    pub(crate) second: ExponentType,
    /// mole (mol)
    pub(crate) mole: ExponentType,
    /// ampere (A)
    pub(crate) ampere: ExponentType,
    /// kelvin (K)
    pub(crate) kelvin: ExponentType,
    /// candela (cd)
    pub(crate) candela: ExponentType,
}

impl<ExponentType: UnitExponent> BaseUnit<ExponentType> {
    pub fn unitless() -> Self {
        let zero = ExponentType::from_int(0);
        Self {
            kilogram: zero,
            meter: zero,
            second: zero,
            mole: zero,
            ampere: zero,
            kelvin: zero,
            candela: zero,
        }
    }

    pub fn multiply(self, other: Self) -> Self {
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

    pub fn divide(self, other: Self) -> Self {
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

    pub fn pow(self, power: i8) -> Self {
        Self {
            kilogram: self.kilogram * power,
            meter: self.meter * power,
            second: self.second * power,
            mole: self.mole * power,
            ampere: self.ampere * power,
            kelvin: self.kelvin * power,
            candela: self.candela * power,
        }
    }

    pub fn root(self, root: i8) -> Self {
        Self {
            kilogram: self.kilogram.strict_div(root),
            meter: self.meter.strict_div(root),
            second: self.second.strict_div(root),
            mole: self.mole.strict_div(root),
            ampere: self.ampere.strict_div(root),
            kelvin: self.kelvin.strict_div(root),
            candela: self.candela.strict_div(root),
        }
    }

    pub(crate) fn magnitude(self) -> u16 {
        self.meter.magnitude()
            + self.second.magnitude()
            + self.mole.magnitude()
            + self.ampere.magnitude()
            + self.kelvin.magnitude()
            + self.candela.magnitude()
            + self.kilogram.magnitude()
    }
}

// We only allow fractional exponents to be multiplied by integers
// to keep things simple, so while raising a unit by an arbitrary exponent
// type isn't generally allowed, it is allowed if the Self type uses integers.
// 
// Note: this mostly exists because it's useful in converting derived units
// where we are multiplying the number of some unit in the identity
// (e.g. 1 Second in the Couloumb identity) by the exponent associated with
// that derived unit.
impl BaseUnit<i8> {
    pub fn int_pow<ExponentType>(self, other: ExponentType) -> BaseUnit<ExponentType>
    where
        ExponentType: UnitExponent,
    {
        BaseUnit {
            kilogram: other * self.kilogram,
            meter: other * self.meter,
            second: other * self.second,
            mole: other * self.mole,
            ampere: other * self.ampere,
            kelvin: other * self.kelvin,
            candela: other * self.candela,
        }
    }
}

impl<ExponentType: UnitExponent> Mul for BaseUnit<ExponentType> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl<ExponentType: UnitExponent> Div for BaseUnit<ExponentType> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.divide(rhs)
    }
}

#[derive(Clone, Copy)]
pub struct BaseValue<Number, ExponentType = i8> {
    pub(crate) unit: BaseUnit<ExponentType>,
    pub(crate) number: Number,
}

#[derive(Error, Debug)]
#[error("Unit '{lhs}' didn't match '{rhs}'")]
pub struct UnitMismatch<ExponentType>
where
    ExponentType: UnitExponent,
{
    pub lhs: BaseUnit<ExponentType>,
    pub rhs: BaseUnit<ExponentType>,
}

impl<ExponentType, Number> PartialEq for BaseValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
    Number: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.unit == other.unit && self.number == other.number
    }
}

impl<ExponentType, Number> Eq for BaseValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
    Number: PartialEq,
{
}

impl<ExponentType, Number> fmt::Debug for BaseValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
    Number: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BaseValue")
            .field("unit", &self.unit)
            .field("number", &self.number)
            .finish()
    }
}

impl<ExponentType, Number> Neg for BaseValue<Number, ExponentType>
where
    Number: Neg<Output = Number>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            unit: self.unit,
            number: -self.number,
        }
    }
}

impl<ExponentType, Number> Add for BaseValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
    Number: Add<Output = Number>,
{
    type Output = Result<Self, UnitMismatch<ExponentType>>;

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

impl<ExponentType, Number> Sub for BaseValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
    Number: Sub<Output = Number>,
{
    type Output = Result<Self, UnitMismatch<ExponentType>>;

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

impl<ExponentType, Number> Mul for BaseValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
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

impl<ExponentType, Number> Div for BaseValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
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

/// meters squared (m²)
pub const METER_SQ: BaseUnit = BaseUnit {
    meter: 2,
    ..UNITLESS
};

/// hertz (Hz)
pub const HERTZ: BaseUnit = BaseUnit {
    second: -1,
    ..UNITLESS
};

/// newton (N)
pub const NEWTON: BaseUnit = BaseUnit {
    kilogram: 1,
    meter: 1,
    second: -2,
    ..UNITLESS
};

/// pascal (Pa)
pub const PASCAL: BaseUnit = BaseUnit {
    kilogram: 1,
    meter: -1,
    second: -2,
    ..UNITLESS
};

/// joule (J)
pub const JOULE: BaseUnit = BaseUnit {
    kilogram: 1,
    meter: 2,
    second: -2,
    ..UNITLESS
};

/// watt (W)
pub const WATT: BaseUnit = BaseUnit {
    meter: 2,
    second: -3,
    kilogram: 1,
    ..UNITLESS
};

/// coulomb (C)
pub const COULOMB: BaseUnit = BaseUnit {
    second: 1,
    ampere: 1,
    ..UNITLESS
};

/// volt (V)
pub const VOLT: BaseUnit = BaseUnit {
    meter: 2,
    second: -3,
    ampere: -1,
    kilogram: 1,
    ..UNITLESS
};

/// farad (F)
pub const FARAD: BaseUnit = BaseUnit {
    meter: -2,
    second: 4,
    ampere: 2,
    kilogram: -1,
    ..UNITLESS
};

/// ohm (Ω)
pub const OHM: BaseUnit = BaseUnit {
    kilogram: 1,
    meter: 2,
    second: -3,
    ampere: -2,
    ..UNITLESS
};

/// siemens (S)
pub const SIEMENS: BaseUnit = BaseUnit {
    kilogram: -1,
    meter: -2,
    second: 3,
    ampere: 2,
    ..UNITLESS
};

/// weber (Wb)
pub const WEBER: BaseUnit = BaseUnit {
    kilogram: 1,
    meter: 2,
    second: -2,
    ampere: -1,
    ..UNITLESS
};

/// tesla (T)
pub const TESLA: BaseUnit = BaseUnit {
    kilogram: 1,
    second: -2,
    ampere: -1,
    ..UNITLESS
};

/// henry (H)
pub const HENRY: BaseUnit = BaseUnit {
    kilogram: 1,
    meter: 2,
    second: -2,
    ampere: -2,
    ..UNITLESS
};

/// lux (lx)
pub const LUX: BaseUnit = BaseUnit {
    meter: -2,
    candela: 1,
    ..UNITLESS
};

/// becquerel (Bq)
pub const BECQUEREL: BaseUnit = BaseUnit {
    second: -1,
    ..UNITLESS
};

/// gray (Gy)
pub const GRAY: BaseUnit = BaseUnit {
    meter: 2,
    second: -2,
    ..UNITLESS
};

/// sievert (Sv)
pub const SIEVERT: BaseUnit = BaseUnit {
    meter: 2,
    second: -2,
    ..UNITLESS
};

/// katal (kat)
pub const KATAL: BaseUnit = BaseUnit {
    second: -1,
    mole: 1,
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

    #[test]
    fn test_identities() {
        assert_eq!(HERTZ, UNITLESS / SECOND);

        assert_eq!(NEWTON, KILOGRAM * METER / (SECOND * SECOND));

        assert_eq!(PASCAL, NEWTON / (METER * METER));

        assert_eq!(JOULE, METER * NEWTON);
        assert_eq!(JOULE, COULOMB * VOLT);
        assert_eq!(JOULE, WATT * SECOND);

        assert_eq!(WATT, JOULE / SECOND);
        assert_eq!(WATT, VOLT * AMPERE);

        assert_eq!(COULOMB, SECOND * AMPERE);
        assert_eq!(COULOMB, FARAD * VOLT);

        assert_eq!(VOLT, WATT / AMPERE);
        assert_eq!(VOLT, JOULE / COULOMB);

        assert_eq!(FARAD, COULOMB / VOLT);
        assert_eq!(FARAD, SECOND / OHM);

        assert_eq!(OHM, UNITLESS / SIEMENS);
        assert_eq!(OHM, VOLT / AMPERE);

        assert_eq!(SIEMENS, UNITLESS / OHM);
        assert_eq!(SIEMENS, AMPERE / VOLT);

        assert_eq!(WEBER, JOULE / AMPERE);
        assert_eq!(WEBER, TESLA * METER * METER);
        assert_eq!(WEBER, VOLT * SECOND);

        assert_eq!(TESLA, VOLT * SECOND / (METER * METER));
        assert_eq!(TESLA, WEBER / (METER * METER));
        assert_eq!(TESLA, NEWTON / (AMPERE * METER));

        assert_eq!(HENRY, VOLT * SECOND / AMPERE);
        assert_eq!(HENRY, OHM * SECOND);
        assert_eq!(HENRY, WEBER / AMPERE);

        assert_eq!(LUX, CANDELA / (METER * METER));

        assert_eq!(BECQUEREL, UNITLESS / SECOND);

        assert_eq!(GRAY, JOULE / KILOGRAM);

        assert_eq!(SIEVERT, JOULE / KILOGRAM);

        assert_eq!(KATAL, MOLE / SECOND);
    }
}
