use std::ops::{Add, Div, Mul, Sub};

use thiserror::Error;

use crate::base::{self, BaseUnit};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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
            base: self.base.multiply(other.base),
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
pub const HERTZ_BASE: BaseUnit = BaseUnit {
    meter: 0,
    second: -1,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 0,
};

pub const HERTZ_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(HERTZ_BASE),
    hertz: 1,
    ..UNITLESS
};

pub const HERTZ: DerivedUnit = DerivedUnit {
    hertz: 1,
    ..UNITLESS
};

/// newton (N)
pub const NEWTON_BASE: BaseUnit = BaseUnit {
    kilogram: 1,
    meter: 1,
    second: -2,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
};

pub const NEWTON_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(NEWTON_BASE),
    newton: 1,
    ..UNITLESS
};

pub const NEWTON: DerivedUnit = DerivedUnit {
    newton: 1,
    ..UNITLESS
};

/// pascal (Pa)
pub const PASCAL_BASE: BaseUnit = BaseUnit {
    meter: -1,
    second: -2,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 1,
};

pub const PASCAL_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(PASCAL_BASE),
    pascal: 1,
    ..UNITLESS
};

pub const PASCAL: DerivedUnit = DerivedUnit {
    pascal: 1,
    ..UNITLESS
};

/// joule (J)
pub const JOULE_BASE: BaseUnit = BaseUnit {
    kilogram: 1,
    meter: 2,
    second: -2,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
};

pub const JOULE_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(JOULE_BASE),
    joule: 1,
    ..UNITLESS
};

pub const JOULE: DerivedUnit = DerivedUnit {
    joule: 1,
    ..UNITLESS
};

/// watt (W)
pub const WATT_BASE: BaseUnit = BaseUnit {
    meter: 2,
    second: -3,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 1,
};

pub const WATT_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(WATT_BASE),
    watt: 1,
    ..UNITLESS
};

pub const WATT: DerivedUnit = DerivedUnit {
    watt: 1,
    ..UNITLESS
};

/// coulomb (C)
pub const COULOMB_BASE: BaseUnit = BaseUnit {
    meter: 0,
    second: 1,
    mole: 0,
    ampere: 1,
    kelvin: 0,
    candela: 0,
    kilogram: 0,
};

pub const COULOMB_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(COULOMB_BASE),
    coulomb: 1,
    ..UNITLESS
};

pub const COULOMB: DerivedUnit = DerivedUnit {
    coulomb: 1,
    ..UNITLESS
};

/// volt (V)
pub const VOLT_BASE: BaseUnit = BaseUnit {
    meter: 2,
    second: -3,
    mole: 0,
    ampere: -1,
    kelvin: 0,
    candela: 0,
    kilogram: 1,
};

pub const VOLT_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(VOLT_BASE),
    volt: 1,
    ..UNITLESS
};

pub const VOLT: DerivedUnit = DerivedUnit {
    volt: 1,
    ..UNITLESS
};

/// farad (F)
pub const FARAD_BASE: BaseUnit = BaseUnit {
    meter: -2,
    second: 4,
    mole: 0,
    ampere: 2,
    kelvin: 0,
    candela: 0,
    kilogram: -1,
};

pub const FARAD_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(FARAD_BASE),
    farad: 1,
    ..UNITLESS
};

pub const FARAD: DerivedUnit = DerivedUnit {
    farad: 1,
    ..UNITLESS
};

/// ohm (Ω)
pub const OHM_BASE: BaseUnit = BaseUnit {
    kilogram: 1,
    meter: 2,
    second: -3,
    mole: 0,
    ampere: -2,
    kelvin: 0,
    candela: 0,
};

pub const OHM_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(OHM_BASE),
    ohm: 1,
    ..UNITLESS
};

pub const OHM: DerivedUnit = DerivedUnit { ohm: 1, ..UNITLESS };

/// siemens (S)
pub const SIEMENS_BASE: BaseUnit = BaseUnit {
    kilogram: -1,
    meter: -2,
    second: 3,
    mole: 0,
    ampere: 2,
    kelvin: 0,
    candela: 0,
};

pub const SIEMENS_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(SIEMENS_BASE),
    siemens: 1,
    ..UNITLESS
};

pub const SIEMENS: DerivedUnit = DerivedUnit {
    siemens: 1,
    ..UNITLESS
};

/// weber (Wb)
pub const WEBER_BASE: BaseUnit = BaseUnit {
    kilogram: 1,
    meter: 2,
    second: -2,
    mole: 0,
    ampere: -1,
    kelvin: 0,
    candela: 0,
};

pub const WEBER_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(WEBER_BASE),
    weber: 1,
    ..UNITLESS
};

pub const WEBER: DerivedUnit = DerivedUnit {
    weber: 1,
    ..UNITLESS
};

/// tesla (T)
pub const TESLA_BASE: BaseUnit = BaseUnit {
    kilogram: 1,
    meter: 0,
    second: -2,
    mole: 0,
    ampere: -1,
    kelvin: 0,
    candela: 0,
};

pub const TESLA_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(TESLA_BASE),
    tesla: 1,
    ..UNITLESS
};

pub const TESLA: DerivedUnit = DerivedUnit {
    tesla: 1,
    ..UNITLESS
};

/// henry (H)
pub const HENRY_BASE: BaseUnit = BaseUnit {
    kilogram: 1,
    meter: 2,
    second: -2,
    mole: 0,
    ampere: -2,
    kelvin: 0,
    candela: 0,
};

pub const HENRY_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(HENRY_BASE),
    henry: 1,
    ..UNITLESS
};

pub const HENRY: DerivedUnit = DerivedUnit {
    henry: 1,
    ..UNITLESS
};

// TODO: celsius (C)

// TODO: lumen (lm)

/// lux (lx)
pub const LUX_BASE: BaseUnit = BaseUnit {
    kilogram: 0,
    meter: -2,
    second: 0,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 1,
};

pub const LUX_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(LUX_BASE),
    lux: 1,
    ..UNITLESS
};

pub const LUX: DerivedUnit = DerivedUnit { lux: 1, ..UNITLESS };

/// becquerel (Bq)
pub const BECQUEREL_BASE: BaseUnit = BaseUnit {
    kilogram: 0,
    meter: 0,
    second: -1,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
};

pub const BECQUEREL_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(BECQUEREL_BASE),
    becquerel: 1,
    ..UNITLESS
};

pub const BECQUEREL: DerivedUnit = DerivedUnit {
    becquerel: 1,
    ..UNITLESS
};

/// gray (Gy)
pub const GRAY_BASE: BaseUnit = BaseUnit {
    kilogram: 0,
    meter: 2,
    second: -2,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
};

pub const GRAY_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(GRAY_BASE),
    gray: 1,
    ..UNITLESS
};

pub const GRAY: DerivedUnit = DerivedUnit {
    gray: 1,
    ..UNITLESS
};

/// sievert (Sv)
pub const SIEVERT_BASE: BaseUnit = BaseUnit {
    kilogram: 0,
    meter: 2,
    second: -2,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
};

pub const SIEVERT_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(SIEVERT_BASE),
    sievert: 1,
    ..UNITLESS
};

pub const SIEVERT: DerivedUnit = DerivedUnit {
    sievert: 1,
    ..UNITLESS
};

/// katal (kat)
pub const KATAL_BASE: BaseUnit = BaseUnit {
    kilogram: 0,
    meter: 0,
    second: -1,
    mole: 1,
    ampere: 0,
    kelvin: 0,
    candela: 0,
};

pub const KATAL_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(KATAL_BASE),
    katal: 1,
    ..UNITLESS
};

pub const KATAL: DerivedUnit = DerivedUnit {
    katal: 1,
    ..UNITLESS
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base;

    #[test]
    fn test_base_identities() {
        assert_eq!(HERTZ_BASE, base::UNITLESS / base::SECOND);

        assert_eq!(
            NEWTON_BASE,
            base::KILOGRAM * base::METER / (base::SECOND * base::SECOND)
        );

        assert_eq!(PASCAL_BASE, NEWTON_BASE / (base::METER * base::METER));

        assert_eq!(JOULE_BASE, base::METER * NEWTON_BASE);
        assert_eq!(JOULE_BASE, COULOMB_BASE * VOLT_BASE);
        assert_eq!(JOULE_BASE, WATT_BASE * base::SECOND);

        assert_eq!(WATT_BASE, JOULE_BASE / base::SECOND);
        assert_eq!(WATT_BASE, VOLT_BASE * base::AMPERE);

        assert_eq!(COULOMB_BASE, base::SECOND * base::AMPERE);
        assert_eq!(COULOMB_BASE, FARAD_BASE * VOLT_BASE);

        assert_eq!(VOLT_BASE, WATT_BASE / base::AMPERE);
        assert_eq!(VOLT_BASE, JOULE_BASE / COULOMB_BASE);

        assert_eq!(FARAD_BASE, COULOMB_BASE / VOLT_BASE);
        assert_eq!(FARAD_BASE, base::SECOND / OHM_BASE);

        assert_eq!(OHM_BASE, base::UNITLESS / SIEMENS_BASE);
        assert_eq!(OHM_BASE, VOLT_BASE / base::AMPERE);

        assert_eq!(SIEMENS_BASE, base::UNITLESS / OHM_BASE);
        assert_eq!(SIEMENS_BASE, base::AMPERE / VOLT_BASE);

        assert_eq!(WEBER_BASE, JOULE_BASE / base::AMPERE);
        assert_eq!(WEBER_BASE, TESLA_BASE * base::METER * base::METER);
        assert_eq!(WEBER_BASE, VOLT_BASE * base::SECOND);

        assert_eq!(
            TESLA_BASE,
            VOLT_BASE * base::SECOND / (base::METER * base::METER)
        );
        assert_eq!(TESLA_BASE, WEBER_BASE / (base::METER * base::METER));
        assert_eq!(TESLA_BASE, NEWTON_BASE / (base::AMPERE * base::METER));

        assert_eq!(HENRY_BASE, VOLT_BASE * base::SECOND / base::AMPERE);
        assert_eq!(HENRY_BASE, OHM_BASE * base::SECOND);
        assert_eq!(HENRY_BASE, WEBER_BASE / base::AMPERE);

        assert_eq!(LUX_BASE, base::CANDELA / (base::METER * base::METER));

        assert_eq!(BECQUEREL_BASE, base::UNITLESS / base::SECOND);

        assert_eq!(GRAY_BASE, JOULE_BASE / base::KILOGRAM);

        assert_eq!(SIEVERT_BASE, JOULE_BASE / base::KILOGRAM);

        assert_eq!(KATAL_BASE, base::MOLE / base::SECOND);
    }
}
