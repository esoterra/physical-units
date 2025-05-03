use crate::derived::*;

pub const IDENTITIES: [DerivedUnit; 44] = [
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
