use crate::{base, derived, identities::BASIC_IDENTITIES};

impl From<base::BaseUnit> for derived::DerivedUnit {
    fn from(value: base::BaseUnit) -> Self {
        value.to_derived()
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
        value.to_base()
    }
}

impl derived::DerivedUnit {
    pub fn to_base(self) -> base::BaseUnit {
        let mut output = self.base;
        let derived_units = [
            (self.hertz, base::HERTZ),
            (self.newton, base::NEWTON),
            (self.pascal, base::PASCAL),
            (self.joule, base::JOULE),
            (self.watt, base::WATT),
            (self.coulomb, base::COULOMB),
            (self.volt, base::VOLT),
            (self.farad, base::FARAD),
            (self.ohm, base::OHM),
            (self.siemens, base::SIEMENS),
            (self.weber, base::WEBER),
            (self.tesla, base::TESLA),
            (self.henry, base::HENRY),
            (self.lux, base::LUX),
            (self.becquerel, base::BECQUEREL),
            (self.gray, base::GRAY),
            (self.sievert, base::SIEVERT),
            (self.katal, base::KATAL),
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
        value.to_derived()
    }
}

impl<Number> base::BaseValue<Number> {
    pub fn to_derived(self) -> derived::DerivedValue<Number> {
        derived::DerivedValue {
            unit: self.unit.to_derived(),
            number: self.number,
        }
    }
}

impl<Number> From<derived::DerivedValue<Number>> for base::BaseValue<Number> {
    fn from(value: derived::DerivedValue<Number>) -> Self {
        value.to_base()
    }
}

impl<Number> derived::DerivedValue<Number> {
    pub fn to_base(self) -> base::BaseValue<Number> {
        base::BaseValue {
            unit: self.unit.to_base(),
            number: self.number,
        }
    }
}

impl derived::DerivedUnit {
    pub fn simplify(self) -> Self {
        let mut output = self;

        for identity in BASIC_IDENTITIES {
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

#[cfg(test)]
mod tests {
    use crate::{base, derived};

    #[test]
    fn test_base_derived_conversions() {
        assert_eq!(base::HERTZ, derived::HERTZ.into());
        assert_eq!(base::NEWTON, derived::NEWTON.into());
        assert_eq!(base::PASCAL, derived::PASCAL.into());
        assert_eq!(base::JOULE, derived::JOULE.into());
        assert_eq!(base::WATT, derived::WATT.into());
        assert_eq!(base::COULOMB, derived::COULOMB.into());
        assert_eq!(base::VOLT, derived::VOLT.into());
        assert_eq!(base::FARAD, derived::FARAD.into());
        assert_eq!(base::OHM, derived::OHM.into());
        assert_eq!(base::SIEMENS, derived::SIEMENS.into());
        assert_eq!(base::WEBER, derived::WEBER.into());
        assert_eq!(base::TESLA, derived::TESLA.into());
        assert_eq!(base::HENRY, derived::HENRY.into());
        assert_eq!(base::LUX, derived::LUX.into());
        assert_eq!(base::BECQUEREL, derived::BECQUEREL.into());
        assert_eq!(base::GRAY, derived::GRAY.into());
        assert_eq!(base::SIEVERT, derived::SIEVERT.into());
        assert_eq!(base::KATAL, derived::KATAL.into());
    }

    #[test]
    fn test_derive() {
        let base_joule =
            base::KILOGRAM * (base::METER * base::METER) / (base::SECOND * base::SECOND);
        let derived_joule = base_joule.to_derived().simplify();
        assert_eq!(
            derived::JOULE,
            derived_joule,
            "{} != {}",
            derived::JOULE,
            derived_joule
        );

        let base_joule =
            base::KILOGRAM * (base::METER * base::METER * base::METER) / (base::SECOND * base::SECOND);
        let actual = base_joule.to_derived().simplify();
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
