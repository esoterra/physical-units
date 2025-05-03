use crate::{base, derived};

impl From<base::BaseUnit> for derived::DerivedUnit {
    fn from(value: base::BaseUnit) -> Self {
        derived::DerivedUnit {
            base: value,
            ..derived::UNITLESS
        }
    }
}

impl base::BaseUnit {
    pub const fn to_derived(self) -> derived::DerivedUnit {
        derived::DerivedUnit {
            base: self,
            ..derived::UNITLESS
        }
    }
}

impl From<derived::DerivedUnit> for base::BaseUnit {
    fn from(value: derived::DerivedUnit) -> Self {
        let mut output = value.base;
        let derived_units = [
            (value.hertz, derived::HERTZ_BASE),
            (value.newton, derived::NEWTON_BASE),
            (value.pascal, derived::PASCAL_BASE),
            (value.joule, derived::JOULE_BASE),
            (value.watt, derived::WATT_BASE),
            (value.coulomb, derived::COULOMB_BASE),
            (value.volt, derived::VOLT_BASE),
            (value.farad, derived::FARAD_BASE),
            (value.ohm, derived::OHM_BASE),
            (value.siemens, derived::SIEMENS_BASE),
            (value.weber, derived::WEBER_BASE),
            (value.tesla, derived::TESLA_BASE),
            (value.henry, derived::HENRY_BASE),
            (value.lux, derived::LUX_BASE),
            (value.becquerel, derived::BECQUEREL_BASE),
            (value.gray, derived::GRAY_BASE),
            (value.sievert, derived::SIEVERT_BASE),
            (value.katal, derived::KATAL_BASE),
        ];
        for (n, base) in derived_units {
            if n > 0 {
                for _ in 0..n {
                    output = output.multiply(base);
                }
            } else {
                for _ in 0..-n {
                    output = output.divide(base);
                }
            }
        }
        output
    }
}

impl<Number> From<base::BaseValue<Number>> for derived::DerivedValue<Number> {
    fn from(value: base::BaseValue<Number>) -> Self {
        Self {
            unit: value.unit.into(),
            number: value.number,
        }
    }
}

impl<Number> From<derived::DerivedValue<Number>> for base::BaseValue<Number> {
    fn from(value: derived::DerivedValue<Number>) -> Self {
        Self {
            unit: value.unit.into(),
            number: value.number,
        }
    }
}

impl base::BaseUnit {
    pub fn derive(self) -> derived::DerivedUnit {
        let mut output = derived::DerivedUnit {
            base: self,
            ..derived::UNITLESS
        };

        let identities = [
            derived::JOULE_IDENTITY,
            derived::NEWTON_IDENTITY,
            derived::PASCAL_IDENTITY,
            derived::WATT_IDENTITY,
            // Ampere-derived
            derived::COULOMB_IDENTITY,
            derived::VOLT_IDENTITY,
            derived::FARAD_IDENTITY,
            derived::OHM_IDENTITY,
            derived::SIEMENS_IDENTITY,
            derived::WEBER_IDENTITY,
            derived::HENRY_IDENTITY,
            derived::TESLA_IDENTITY,
            derived::HENRY_IDENTITY,
            // Time-derived
            derived::HERTZ_IDENTITY,
            derived::BECQUEREL_IDENTITY, // Redundant
            derived::SIEVERT_IDENTITY,
            derived::GRAY_IDENTITY, // Redundant
            // Candela-derive
            derived::LUX_IDENTITY,
            // mole-derived
            derived::KATAL_IDENTITY,
        ];
        for identity in identities {
            let after_mul = output.multiply(identity);
            let after_div = output.divide(identity);
            if after_mul.magnitude() < output.magnitude() {
                let mut after_mul = after_mul;
                let mut after_mul_mag = after_mul.magnitude();
                loop {
                    let next = after_mul.multiply(identity);
                    let next_mag = next.magnitude();
                    if next_mag >= after_mul_mag {
                        break;
                    }
                    after_mul = next;
                    after_mul_mag = next_mag;
                }
                output = after_mul;
            } else if after_div.magnitude() < output.magnitude() {
                let mut after_div = after_div;
                let mut after_div_mag = after_div.magnitude();
                loop {
                    let next = after_div.divide(identity);
                    let next_mag = next.magnitude();
                    if next_mag >= after_div_mag {
                        break;
                    }
                    after_div = next;
                    after_div_mag = next_mag;
                }
                output = after_div;
            }
        }
        output
    }
}

impl derived::DerivedUnit {
    pub fn simplify(self) -> Self {
        let mut output = self;

        let derived_units = [
            (self.newton, derived::NEWTON_IDENTITY),
            (self.pascal, derived::PASCAL_IDENTITY),
            (self.joule, derived::JOULE_IDENTITY),
            (self.watt, derived::WATT_IDENTITY),
            (self.coulomb, derived::COULOMB_IDENTITY),
            (self.volt, derived::VOLT_IDENTITY),
            (self.farad, derived::FARAD_IDENTITY),
            (self.ohm, derived::OHM_IDENTITY),
            (self.siemens, derived::SIEMENS_IDENTITY),
            (self.weber, derived::WEBER_IDENTITY),
            (self.tesla, derived::TESLA_IDENTITY),
            (self.henry, derived::HENRY_IDENTITY),
            (self.lux, derived::LUX_IDENTITY),
            (self.becquerel, derived::BECQUEREL_IDENTITY),
            (self.gray, derived::GRAY_IDENTITY),
            (self.sievert, derived::SIEVERT_IDENTITY),
            (self.katal, derived::KATAL_IDENTITY),
            (self.hertz, derived::HENRY_IDENTITY),
        ];
        for (n, base) in derived_units {
            if n > 0 {
                for _ in 0..n {
                    output = output.multiply(base);
                }
            } else {
                for _ in 0..-n {
                    output = output.divide(base);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::{base, derived};

    #[test]
    fn test_base_derived_conversions() {
        assert_eq!(derived::HERTZ_BASE, derived::HERTZ.into());
        assert_eq!(derived::NEWTON_BASE, derived::NEWTON.into());
        assert_eq!(derived::PASCAL_BASE, derived::PASCAL.into());
        assert_eq!(derived::JOULE_BASE, derived::JOULE.into());
        assert_eq!(derived::WATT_BASE, derived::WATT.into());
        assert_eq!(derived::COULOMB_BASE, derived::COULOMB.into());
        assert_eq!(derived::VOLT_BASE, derived::VOLT.into());
        assert_eq!(derived::FARAD_BASE, derived::FARAD.into());
        assert_eq!(derived::OHM_BASE, derived::OHM.into());
        assert_eq!(derived::SIEMENS_BASE, derived::SIEMENS.into());
        assert_eq!(derived::WEBER_BASE, derived::WEBER.into());
        assert_eq!(derived::TESLA_BASE, derived::TESLA.into());
        assert_eq!(derived::HENRY_BASE, derived::HENRY.into());
        assert_eq!(derived::LUX_BASE, derived::LUX.into());
        assert_eq!(derived::BECQUEREL_BASE, derived::BECQUEREL.into());
        assert_eq!(derived::GRAY_BASE, derived::GRAY.into());
        assert_eq!(derived::SIEVERT_BASE, derived::SIEVERT.into());
        assert_eq!(derived::KATAL_BASE, derived::KATAL.into());
    }

    #[test]
    fn test_derive() {
        let base_joule =
            base::KILOGRAM * (base::METER * base::METER) / (base::SECOND * base::SECOND);
        let derived_joule = base_joule.derive();
        assert_eq!(
            derived::JOULE,
            derived_joule,
            "{} != {}",
            derived::JOULE,
            derived_joule
        );

        let base_joule =
            base::KILOGRAM * (base::METER * base::METER * base::METER) / (base::SECOND * base::SECOND);
        let actual = base_joule.derive();
        let expected = derived::JOULE * derived::METER;
        assert_eq!(
            expected,
            actual,
            "{} != {}",
            expected,
            actual
        );
    }
}
