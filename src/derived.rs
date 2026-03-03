use core::fmt;
use std::ops::{Add, Div, Mul, Sub};

use thiserror::Error;

use crate::{
    base::{self, BaseUnit},
    exponents::UnitExponent,
};

#[derive(Default, Clone, Copy)]
pub struct DerivedUnit<ExponentType = i8> {
    /// remaining base units
    pub(crate) base: BaseUnit<ExponentType>,

    /// hertz (Hz)
    pub(crate) hertz: ExponentType,
    /// newton (N)
    pub(crate) newton: ExponentType,
    /// pascal (Pa)
    pub(crate) pascal: ExponentType,
    /// joule (J)
    pub(crate) joule: ExponentType,
    /// watt (W)
    pub(crate) watt: ExponentType,
    /// coulomb (C)
    pub(crate) coulomb: ExponentType,
    /// volt (V)
    pub(crate) volt: ExponentType,
    /// farad (F)
    pub(crate) farad: ExponentType,
    /// ohm (Ω)
    pub(crate) ohm: ExponentType,
    /// siemens (S)
    pub(crate) siemens: ExponentType,
    /// weber (Wb)
    pub(crate) weber: ExponentType,
    /// tesla (T)
    pub(crate) tesla: ExponentType,
    /// henry (H)
    pub(crate) henry: ExponentType,
    /// lux (lx)
    pub(crate) lux: ExponentType,
    /// becquerel (Bq)
    pub(crate) becquerel: ExponentType,
    /// gray (Gy)
    pub(crate) gray: ExponentType,
    /// sievert (Sv)
    pub(crate) sievert: ExponentType,
    /// katal (kat)
    pub(crate) katal: ExponentType,
}

impl<ExponentType> PartialEq for DerivedUnit<ExponentType>
where
    ExponentType: UnitExponent,
{
    fn eq(&self, other: &Self) -> bool {
        self.to_base() == other.to_base()
    }
}

impl<ExponentType> Eq for DerivedUnit<ExponentType> where ExponentType: UnitExponent {}

impl<ExponentType> DerivedUnit<ExponentType>
where
    ExponentType: UnitExponent,
{
    pub fn unitless() -> Self {
        let zero = ExponentType::ZERO;
        Self {
            base: BaseUnit::unitless(),
            hertz: zero,
            newton: zero,
            pascal: zero,
            joule: zero,
            watt: zero,
            coulomb: zero,
            volt: zero,
            farad: zero,
            ohm: zero,
            siemens: zero,
            weber: zero,
            tesla: zero,
            henry: zero,
            lux: zero,
            becquerel: zero,
            gray: zero,
            sievert: zero,
            katal: zero,
        }
    }

    pub fn multiply(self, other: Self) -> Self {
        Self {
            base: self.base.multiply(other.base),
            hertz: self.hertz + other.hertz,
            newton: self.newton + other.newton,
            pascal: self.pascal + other.pascal,
            joule: self.joule + other.joule,
            watt: self.watt + other.watt,
            coulomb: self.coulomb + other.coulomb,
            volt: self.volt + other.volt,
            farad: self.farad + other.farad,
            ohm: self.ohm + other.ohm,
            siemens: self.siemens + other.siemens,
            weber: self.weber + other.weber,
            tesla: self.tesla + other.tesla,
            henry: self.henry + other.henry,
            lux: self.lux + other.lux,
            becquerel: self.becquerel + other.becquerel,
            gray: self.gray + other.gray,
            sievert: self.sievert + other.sievert,
            katal: self.katal + other.katal,
        }
    }

    pub fn divide(self, other: Self) -> Self {
        Self {
            base: self.base.divide(other.base),
            hertz: self.hertz - other.hertz,
            newton: self.newton - other.newton,
            pascal: self.pascal - other.pascal,
            joule: self.joule - other.joule,
            watt: self.watt - other.watt,
            coulomb: self.coulomb - other.coulomb,
            volt: self.volt - other.volt,
            farad: self.farad - other.farad,
            ohm: self.ohm - other.ohm,
            siemens: self.siemens - other.siemens,
            weber: self.weber - other.weber,
            tesla: self.tesla - other.tesla,
            henry: self.henry - other.henry,
            lux: self.lux - other.lux,
            becquerel: self.becquerel - other.becquerel,
            gray: self.gray - other.gray,
            sievert: self.sievert - other.sievert,
            katal: self.katal - other.katal,
        }
    }

    pub fn pow(self, power: i8) -> Self {
        Self {
            base: self.base.pow(power),
            hertz: self.hertz * power,
            newton: self.newton * power,
            pascal: self.pascal * power,
            joule: self.joule * power,
            watt: self.watt * power,
            coulomb: self.coulomb * power,
            volt: self.volt * power,
            farad: self.farad * power,
            ohm: self.ohm * power,
            siemens: self.siemens * power,
            weber: self.weber * power,
            tesla: self.tesla * power,
            henry: self.henry * power,
            lux: self.lux * power,
            becquerel: self.becquerel * power,
            gray: self.gray * power,
            sievert: self.sievert * power,
            katal: self.katal * power,
        }
    }

    pub fn root(self, root: i8) -> Self {
        Self {
            base: self.base.root(root),
            hertz: self.hertz.strict_div(root),
            newton: self.newton.strict_div(root),
            pascal: self.pascal.strict_div(root),
            joule: self.joule.strict_div(root),
            watt: self.watt.strict_div(root),
            coulomb: self.coulomb.strict_div(root),
            volt: self.volt.strict_div(root),
            farad: self.farad.strict_div(root),
            ohm: self.ohm.strict_div(root),
            siemens: self.siemens.strict_div(root),
            weber: self.weber.strict_div(root),
            tesla: self.tesla.strict_div(root),
            henry: self.henry.strict_div(root),
            lux: self.lux.strict_div(root),
            becquerel: self.becquerel.strict_div(root),
            gray: self.gray.strict_div(root),
            sievert: self.sievert.strict_div(root),
            katal: self.katal.strict_div(root),
        }
    }

    pub(crate) fn magnitude(self) -> u16 {
        self.base.magnitude()
            + self.hertz.magnitude()
            + self.newton.magnitude()
            + self.pascal.magnitude()
            + self.joule.magnitude()
            + self.watt.magnitude()
            + self.coulomb.magnitude()
            + self.volt.magnitude()
            + self.farad.magnitude()
            + self.ohm.magnitude()
            + self.siemens.magnitude()
            + self.weber.magnitude()
            + self.tesla.magnitude()
            + self.henry.magnitude()
            + self.lux.magnitude()
            + self.becquerel.magnitude()
            + self.gray.magnitude()
            + self.sievert.magnitude()
            + self.katal.magnitude()
    }
}

impl<ExponentType> Mul for DerivedUnit<ExponentType>
where
    ExponentType: UnitExponent,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl<ExponentType> Div for DerivedUnit<ExponentType>
where
    ExponentType: UnitExponent,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.divide(rhs)
    }
}

#[derive(Clone, Copy)]
pub struct DerivedValue<Number, ExponentType = i8> {
    pub(crate) unit: DerivedUnit<ExponentType>,
    pub(crate) number: Number,
}

#[derive(Error)]
#[error("Unit '{lhs}' didn't match '{rhs}'")]
pub struct UnitMismatch<ExponentType> {
    pub lhs: DerivedUnit<ExponentType>,
    pub rhs: DerivedUnit<ExponentType>,
}

impl<Number, ExponentType> PartialEq for DerivedValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
    Number: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.unit == other.unit && self.number == other.number
    }
}

impl<Number, ExponentType> Eq for DerivedValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
    Number: Eq,
{
}

impl<Number, ExponentType> PartialOrd for DerivedValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
    Number: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        assert_eq!(self.unit, other.unit);
        self.number.partial_cmp(&other.number)
    }
}

impl<Number, ExponentType> Ord for DerivedValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
    Number: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        assert_eq!(self.unit, other.unit);
        self.number.cmp(&other.number)
    }
}

impl<Number, ExponentType> fmt::Debug for DerivedValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
    Number: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DerivedValue")
            .field("unit", &self.unit)
            .field("number", &self.number)
            .finish()
    }
}

impl<ExponentType, Number> Add for DerivedValue<Number, ExponentType>
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

impl<ExponentType, Number> Sub for DerivedValue<Number, ExponentType>
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

impl<ExponentType, Number> Mul for DerivedValue<Number, ExponentType>
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

impl<ExponentType, Number> Div for DerivedValue<Number, ExponentType>
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

pub const UNITLESS: DerivedUnit<i8> = DerivedUnit {
    base: base::UNITLESS,
    hertz: 0,
    newton: 0,
    pascal: 0,
    joule: 0,
    watt: 0,
    coulomb: 0,
    volt: 0,
    farad: 0,
    ohm: 0,
    siemens: 0,
    weber: 0,
    tesla: 0,
    henry: 0,
    lux: 0,
    becquerel: 0,
    gray: 0,
    sievert: 0,
    katal: 0,
};

pub const METER: DerivedUnit = DerivedUnit {
    base: base::METER,
    ..UNITLESS
};

pub const SECOND: DerivedUnit = DerivedUnit {
    base: base::SECOND,
    ..UNITLESS
};

pub const MOLE: DerivedUnit = DerivedUnit {
    base: base::MOLE,
    ..UNITLESS
};

pub const AMPERE: DerivedUnit = DerivedUnit {
    base: base::AMPERE,
    ..UNITLESS
};

pub const KELVIN: DerivedUnit = DerivedUnit {
    base: base::KELVIN,
    ..UNITLESS
};

pub const CANDELA: DerivedUnit = DerivedUnit {
    base: base::CANDELA,
    ..UNITLESS
};

pub const KILOGRAM: DerivedUnit = DerivedUnit {
    base: base::KILOGRAM,
    ..UNITLESS
};

pub const METER_SQ: DerivedUnit = DerivedUnit {
    base: base::METER_SQ,
    ..UNITLESS
};

/// hertz (Hz)
pub const HERTZ: DerivedUnit = DerivedUnit {
    hertz: 1,
    ..UNITLESS
};

/// newton (N)
pub const NEWTON: DerivedUnit = DerivedUnit {
    newton: 1,
    ..UNITLESS
};

/// pascal (Pa)
pub const PASCAL: DerivedUnit = DerivedUnit {
    pascal: 1,
    ..UNITLESS
};

/// joule (J)
pub const JOULE: DerivedUnit = DerivedUnit {
    joule: 1,
    ..UNITLESS
};

/// watt (W)
pub const WATT: DerivedUnit = DerivedUnit {
    watt: 1,
    ..UNITLESS
};

/// coulomb (C)
pub const COULOMB: DerivedUnit = DerivedUnit {
    coulomb: 1,
    ..UNITLESS
};

/// volt (V)
pub const VOLT: DerivedUnit = DerivedUnit {
    volt: 1,
    ..UNITLESS
};

/// farad (F)
pub const FARAD: DerivedUnit = DerivedUnit {
    farad: 1,
    ..UNITLESS
};

/// ohm (Ω)
pub const OHM: DerivedUnit = DerivedUnit { ohm: 1, ..UNITLESS };

/// siemens (S)
pub const SIEMENS: DerivedUnit = DerivedUnit {
    siemens: 1,
    ..UNITLESS
};

/// weber (Wb)
pub const WEBER: DerivedUnit = DerivedUnit {
    weber: 1,
    ..UNITLESS
};

/// tesla (T)
pub const TESLA: DerivedUnit = DerivedUnit {
    tesla: 1,
    ..UNITLESS
};

/// henry (H)
pub const HENRY: DerivedUnit = DerivedUnit {
    henry: 1,
    ..UNITLESS
};

// TODO: celsius (C)

// TODO: lumen (lm)

/// lux (lx)
pub const LUX: DerivedUnit = DerivedUnit { lux: 1, ..UNITLESS };

/// becquerel (Bq)
pub const BECQUEREL: DerivedUnit = DerivedUnit {
    becquerel: 1,
    ..UNITLESS
};

/// gray (Gy)
pub const GRAY: DerivedUnit = DerivedUnit {
    gray: 1,
    ..UNITLESS
};

/// sievert (Sv)
pub const SIEVERT: DerivedUnit = DerivedUnit {
    sievert: 1,
    ..UNITLESS
};

/// katal (kat)
pub const KATAL: DerivedUnit = DerivedUnit {
    katal: 1,
    ..UNITLESS
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_identities() {
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
