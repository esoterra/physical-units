use crate::Unit;

pub const UNITLESS: Unit = Unit {
    meter: 0,
    second: 0,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 0,
};

/// meter (m)
pub const METER: Unit = Unit {
    meter: 1,
    second: 0,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 0,
};

/// second (s)
pub const SECOND: Unit = Unit {
    meter: 0,
    second: 1,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 0,
};

/// mole (mol)
pub const MOLE: Unit = Unit {
    meter: 0,
    second: 0,
    mole: 1,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 0,
};

/// ampere (A)
pub const AMPERE: Unit = Unit {
    meter: 0,
    second: 0,
    mole: 0,
    ampere: 1,
    kelvin: 0,
    candela: 0,
    kilogram: 0,
};

/// kelvin (K)
pub const KELVIN: Unit = Unit {
    meter: 0,
    second: 0,
    mole: 0,
    ampere: 0,
    kelvin: 1,
    candela: 0,
    kilogram: 0,
};

/// candela (cd)
pub const CANDELA: Unit = Unit {
    meter: 0,
    second: 0,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 1,
    kilogram: 0,
};

/// kilogram (kg)
pub const KILOGRAM: Unit = Unit {
    meter: 0,
    second: 0,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 1,
};