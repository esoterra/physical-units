use crate::Unit;

/// hertz (Hz)
pub const HERTZ: Unit = Unit {
    meter: 0,
    second: -1,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 0,
};

/// newton (N)
pub const NEWTON: Unit = Unit {
    kilogram: 1,
    meter: 1,
    second: -2,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
};

/// pascal (Pa)
pub const PASCAL: Unit = Unit {
    meter: -1,
    second: -2,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 1,
};

/// joule (J)
pub const JOULE: Unit = Unit {
    meter: 2,
    second: -2,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 1,
};

/// watt (W)
pub const WATT: Unit = Unit {
    meter: 2,
    second: -3,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
    kilogram: 1,
};

/// coulomb (C)
pub const COULOMB: Unit = Unit {
    meter: 0,
    second: 1,
    mole: 0,
    ampere: 1,
    kelvin: 0,
    candela: 0,
    kilogram: 0,
};

/// volt (V)
pub const VOLT: Unit = Unit {
    meter: 2,
    second: -3,
    mole: 0,
    ampere: -1,
    kelvin: 0,
    candela: 0,
    kilogram: 1,
};

/// farad (F)
pub const FARAD: Unit = Unit {
    meter: -2,
    second: 4,
    mole: 0,
    ampere: 2,
    kelvin: 0,
    candela: 0,
    kilogram: -1,
};

/// ohm (Ω)
pub const OHM: Unit = Unit {
    kilogram: 1,
    meter: 2,
    second: -3,
    mole: 0,
    ampere: -2,
    kelvin: 0,
    candela: 0,
};

/// siemens (Ω)
pub const SIEMENS: Unit = Unit {
    kilogram: -1,
    meter: -2,
    second: 3,
    mole: 0,
    ampere: 2,
    kelvin: 0,
    candela: 0,
};

/// weber (Wb)
pub const WEBER: Unit = Unit {
    kilogram: 1,
    meter: 2,
    second: -2,
    mole: 0,
    ampere: -1,
    kelvin: 0,
    candela: 0,
};

/// tesla (T)
pub const TESLA: Unit = Unit {
    kilogram: 1,
    meter: 0,
    second: -2,
    mole: 0,
    ampere: -1,
    kelvin: 0,
    candela: 0,
};

/// henry (H)
pub const HENRY: Unit = Unit {
    kilogram: 1,
    meter: 2,
    second: -2,
    mole: 0,
    ampere: -2,
    kelvin: 0,
    candela: 0,
};

// TODO: celsius (C)

// TODO: lumen (lm)

/// lux (lx)
pub const LUX: Unit = Unit {
    kilogram: 0,
    meter: -2,
    second: 0,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 1,
};

/// becquerel (Bq)
pub const BECQUEREL: Unit = Unit {
    kilogram: 0,
    meter: 0,
    second: -1,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
};

/// gray (Gy)
pub const GRAY: Unit = Unit {
    kilogram: 0,
    meter: 2,
    second: -2,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
};

/// sievert (Gy)
pub const SIEVERT: Unit = Unit {
    kilogram: 0,
    meter: 2,
    second: -2,
    mole: 0,
    ampere: 0,
    kelvin: 0,
    candela: 0,
};

/// katal (kat)
pub const KATAL: Unit = Unit {
    kilogram: 0,
    meter: 0,
    second: -1,
    mole: 1,
    ampere: 0,
    kelvin: 0,
    candela: 0,
};

#[cfg(test)]
mod tests {
    use crate::base::{self, AMPERE};
    use super::*;

    #[test]
    fn test_identities() {
        assert_eq!(HERTZ, base::UNITLESS / base::SECOND);

        assert_eq!(NEWTON, base::KILOGRAM * base::METER / (base::SECOND * base::SECOND));

        assert_eq!(PASCAL, NEWTON / (base::METER * base::METER));

        assert_eq!(JOULE, base::METER * NEWTON);
        assert_eq!(JOULE, COULOMB * VOLT);
        assert_eq!(JOULE, WATT * base::SECOND);

        assert_eq!(WATT, JOULE / base::SECOND);
        assert_eq!(WATT, VOLT * base::AMPERE);

        assert_eq!(COULOMB, base::SECOND * base::AMPERE);
        assert_eq!(COULOMB, FARAD * VOLT);

        assert_eq!(VOLT, WATT / base::AMPERE);
        assert_eq!(VOLT, JOULE / COULOMB);

        assert_eq!(FARAD, COULOMB / VOLT);
        assert_eq!(FARAD, base::SECOND / OHM);

        assert_eq!(OHM, base::UNITLESS / SIEMENS);
        assert_eq!(OHM, VOLT / base::AMPERE);

        assert_eq!(SIEMENS, base::UNITLESS / OHM);
        assert_eq!(SIEMENS, base::AMPERE / VOLT);

        assert_eq!(WEBER, JOULE / base::AMPERE);
        assert_eq!(WEBER, TESLA * base::METER * base::METER);
        assert_eq!(WEBER, VOLT * base::SECOND);

        assert_eq!(TESLA, VOLT * base::SECOND / (base::METER * base::METER));
        assert_eq!(TESLA, WEBER / (base::METER * base::METER));
        assert_eq!(TESLA, NEWTON / (AMPERE * base::METER));

        assert_eq!(HENRY, VOLT * base::SECOND / AMPERE);
        assert_eq!(HENRY, OHM * base::SECOND);
        assert_eq!(HENRY, WEBER / AMPERE);

        assert_eq!(LUX, base::CANDELA / (base::METER * base::METER));

        assert_eq!(BECQUEREL, base::UNITLESS / base::SECOND);

        assert_eq!(GRAY, JOULE / base::KILOGRAM);

        assert_eq!(SIEVERT, JOULE / base::KILOGRAM);

        assert_eq!(KATAL, base::MOLE / base::SECOND);

    }
}