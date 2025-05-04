use crate::{base, derived::*};

pub const BASIC_IDENTITIES: [DerivedUnit; 19] = [
    // Energy-related
    JOULE_IDENTITY,
    WATT_IDENTITY,
    NEWTON_IDENTITY,
    PASCAL_IDENTITY,
    // Ampere-derived
    COULOMB_IDENTITY,
    VOLT_IDENTITY,
    FARAD_IDENTITY,
    OHM_IDENTITY,
    SIEMENS_IDENTITY,
    WEBER_IDENTITY,
    HENRY_IDENTITY,
    TESLA_IDENTITY,
    HENRY_IDENTITY,
    // Time-derived
    HERTZ_IDENTITY,
    BECQUEREL_IDENTITY, // Redundant
    SIEVERT_IDENTITY,
    GRAY_IDENTITY, // Redundant
    // Candela-derive
    LUX_IDENTITY,
    // mole-derived
    KATAL_IDENTITY,
];

pub const EXTRA_IDENTITIES: [DerivedUnit; 25] = [
    // identities between derived units
    PASCAL.divide(NEWTON.divide(METER_SQ)),
    JOULE.divide(NEWTON.multiply(METER)),
    JOULE.divide(COULOMB.multiply(VOLT)),
    JOULE.divide(WATT.multiply(SECOND)),
    WATT.divide(JOULE.divide(SECOND)),
    WATT.divide(VOLT.multiply(AMPERE)),
    COULOMB.divide(FARAD.multiply(VOLT)),
    VOLT.divide(WATT.divide(AMPERE)),
    VOLT.divide(JOULE.divide(COULOMB)),
    FARAD.divide(COULOMB.divide(VOLT)),
    FARAD.divide(SECOND.divide(OHM)),
    // Ohms-Siemens
    OHM.multiply(SIEMENS),
    OHM.divide(VOLT.divide(AMPERE)),
    SIEMENS.divide(AMPERE.divide(VOLT)),
    WEBER.divide(JOULE.divide(AMPERE)),
    WEBER.divide(TESLA.multiply(METER).multiply(METER)),
    WEBER.divide(VOLT.multiply(SECOND)),
    TESLA.divide(VOLT.multiply(SECOND).divide(METER_SQ)),
    TESLA.divide(WEBER.divide(METER_SQ)),
    TESLA.divide(NEWTON.divide(AMPERE.multiply(METER))),
    HENRY.divide(VOLT.multiply(SECOND).divide(AMPERE)),
    HENRY.divide(OHM.multiply(SECOND)),
    HENRY.divide(WEBER.divide(AMPERE)),
    GRAY.divide(JOULE.divide(KILOGRAM)),
    SIEVERT.divide(JOULE.divide(KILOGRAM)),
];

pub const HERTZ_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::HERTZ),
    hertz: 1,
    ..UNITLESS
};

pub const NEWTON_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::NEWTON),
    newton: 1,
    ..UNITLESS
};

pub const PASCAL_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::PASCAL),
    pascal: 1,
    ..UNITLESS
};

pub const JOULE_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::JOULE),
    joule: 1,
    ..UNITLESS
};

pub const WATT_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::WATT),
    watt: 1,
    ..UNITLESS
};

pub const COULOMB_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::COULOMB),
    coulomb: 1,
    ..UNITLESS
};

pub const VOLT_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::VOLT),
    volt: 1,
    ..UNITLESS
};

pub const FARAD_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::FARAD),
    farad: 1,
    ..UNITLESS
};

pub const OHM_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::OHM),
    ohm: 1,
    ..UNITLESS
};

pub const SIEMENS_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::SIEMENS),
    siemens: 1,
    ..UNITLESS
};

pub const WEBER_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::WEBER),
    weber: 1,
    ..UNITLESS
};

pub const TESLA_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::TESLA),
    tesla: 1,
    ..UNITLESS
};

pub const HENRY_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::HENRY),
    henry: 1,
    ..UNITLESS
};

pub const LUX_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::LUX),
    lux: 1,
    ..UNITLESS
};

pub const BECQUEREL_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::BECQUEREL),
    becquerel: 1,
    ..UNITLESS
};

pub const GRAY_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::GRAY),
    gray: 1,
    ..UNITLESS
};

pub const SIEVERT_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::SIEVERT),
    sievert: 1,
    ..UNITLESS
};

pub const KATAL_IDENTITY: DerivedUnit = DerivedUnit {
    base: base::UNITLESS.divide(base::KATAL),
    katal: 1,
    ..UNITLESS
};