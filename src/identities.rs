use crate::{base, derived::*};

pub fn basic_identities() -> [DerivedUnit; 19] {
    let hertz_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::HERTZ),
        hertz: 1,
        ..UNITLESS
    };

    let newton_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::NEWTON),
        newton: 1,
        ..UNITLESS
    };

    let pascal_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::PASCAL),
        pascal: 1,
        ..UNITLESS
    };

    let joule_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::JOULE),
        joule: 1,
        ..UNITLESS
    };

    let watt_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::WATT),
        watt: 1,
        ..UNITLESS
    };

    let coulomb_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::COULOMB),
        coulomb: 1,
        ..UNITLESS
    };

    let volt_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::VOLT),
        volt: 1,
        ..UNITLESS
    };

    let farad_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::FARAD),
        farad: 1,
        ..UNITLESS
    };

    let ohm_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::OHM),
        ohm: 1,
        ..UNITLESS
    };

    let siemens_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::SIEMENS),
        siemens: 1,
        ..UNITLESS
    };

    let weber_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::WEBER),
        weber: 1,
        ..UNITLESS
    };

    let tesla_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::TESLA),
        tesla: 1,
        ..UNITLESS
    };

    let henry_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::HENRY),
        henry: 1,
        ..UNITLESS
    };

    let lux_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::LUX),
        lux: 1,
        ..UNITLESS
    };

    let becquerel_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::BECQUEREL),
        becquerel: 1,
        ..UNITLESS
    };

    let gray_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::GRAY),
        gray: 1,
        ..UNITLESS
    };

    let sievert_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::SIEVERT),
        sievert: 1,
        ..UNITLESS
    };

    let katal_identity: DerivedUnit = DerivedUnit {
        base: base::UNITLESS.divide(base::KATAL),
        katal: 1,
        ..UNITLESS
    };

    [
        // Energy-related
        joule_identity,
        watt_identity,
        newton_identity,
        pascal_identity,
        // Ampere-derived
        coulomb_identity,
        volt_identity,
        farad_identity,
        ohm_identity,
        siemens_identity,
        weber_identity,
        henry_identity,
        tesla_identity,
        henry_identity,
        // Time-derived
        hertz_identity,
        becquerel_identity, // Redundant
        sievert_identity,
        gray_identity, // Redundant
        // Candela-derive
        lux_identity,
        // mole-derived
        katal_identity,
    ]
}

#[cfg(test)]
fn extra_identities() -> [DerivedUnit; 25] {
    [
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
    ]
}

#[cfg(test)]
mod tests {
    use crate::derived;

    use super::*;

    #[test]
    fn test_basic_identities() {
        for identity in basic_identities() {
            let expected = derived::UNITLESS;
            let actual = identity;
            assert_eq!(expected, actual, "{expected} != {actual}");
        }
    }
    #[test]
    fn test_extra_identities() {
        for identity in extra_identities() {
            let expected = derived::UNITLESS;
            let actual = identity;
            assert_eq!(expected, actual, "{expected} != {actual}");
        }
    }
}
