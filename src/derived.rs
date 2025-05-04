use std::ops::{Add, Div, Mul, Sub};

use thiserror::Error;

use crate::base::{self, BaseUnit};

#[derive(Debug, Default, Clone, Copy)]
pub struct DerivedUnit {
    /// remaining base units
    pub(crate) base: BaseUnit,

    /// hertz (Hz)
    pub(crate) hertz: i8,
    /// newton (N)
    pub(crate) newton: i8,
    /// pascal (Pa)
    pub(crate) pascal: i8,
    /// joule (J)
    pub(crate) joule: i8,
    /// watt (W)
    pub(crate) watt: i8,
    /// coulomb (C)
    pub(crate) coulomb: i8,
    /// volt (V)
    pub(crate) volt: i8,
    /// farad (F)
    pub(crate) farad: i8,
    /// ohm (Ω)
    pub(crate) ohm: i8,
    /// siemens
    pub(crate) siemens: i8,
    /// weber (Wb)
    pub(crate) weber: i8,
    /// tesla (T)
    pub(crate) tesla: i8,
    /// henry (H)
    pub(crate) henry: i8,
    /// lux (lx)
    pub(crate) lux: i8,
    /// becquerel (Bq)
    pub(crate) becquerel: i8,
    /// gray (Gy)
    pub(crate) gray: i8,
    /// sievert (Sv)
    pub(crate) sievert: i8,
    /// katal (kat)
    pub(crate) katal: i8,
}

impl PartialEq for DerivedUnit {
    fn eq(&self, other: &Self) -> bool {
        self.to_base() == other.to_base()
    }
}
impl Eq for DerivedUnit {}

impl DerivedUnit {
    pub const fn multiply(self, other: Self) -> Self {
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

    pub const fn divide(self, other: Self) -> Self {
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

    pub(crate) fn magnitude(self) -> u16 {
        self.base.magnitude()
            + self.hertz.abs() as u16
            + self.newton.abs() as u16
            + self.pascal.abs() as u16
            + self.joule.abs() as u16
            + self.watt.abs() as u16
            + self.coulomb.abs() as u16
            + self.volt.abs() as u16
            + self.farad.abs() as u16
            + self.ohm.abs() as u16
            + self.siemens.abs() as u16
            + self.weber.abs() as u16
            + self.tesla.abs() as u16
            + self.henry.abs() as u16
            + self.lux.abs() as u16
            + self.becquerel.abs() as u16
            + self.gray.abs() as u16
            + self.sievert.abs() as u16
            + self.katal.abs() as u16
    }
}

impl Mul for DerivedUnit {
    type Output = DerivedUnit;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Div for DerivedUnit {
    type Output = DerivedUnit;

    fn div(self, rhs: Self) -> Self::Output {
        self.divide(rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DerivedValue<Number> {
    pub(crate) unit: DerivedUnit,
    pub(crate) number: Number,
}

#[derive(Error, Debug)]
#[error("Unit '{lhs}' didn't match '{rhs}'")]
pub struct UnitMismatch {
    pub lhs: DerivedUnit,
    pub rhs: DerivedUnit,
}

impl<Number> Add for DerivedValue<Number>
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

impl<Number> Sub for DerivedValue<Number>
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

impl<Number> Mul for DerivedValue<Number>
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

impl<Number> Div for DerivedValue<Number>
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

pub const UNITLESS: DerivedUnit = DerivedUnit {
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

        assert_eq!(
            NEWTON,
            KILOGRAM * METER / (SECOND * SECOND)
        );

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

        assert_eq!(
            TESLA,
            VOLT * SECOND / (METER * METER)
        );
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
