use crate::{
    base,
    derived::{self, DerivedUnit},
    exponents::UnitExponent,
    identities::basic_identities,
};

impl<ExponentType> From<base::BaseUnit<ExponentType>> for derived::DerivedUnit<ExponentType>
where
    ExponentType: UnitExponent,
{
    fn from(value: base::BaseUnit<ExponentType>) -> Self {
        value.to_derived()
    }
}

impl<ExponentType> base::BaseUnit<ExponentType>
where
    ExponentType: UnitExponent,
{
    pub fn to_derived(self) -> derived::DerivedUnit<ExponentType> {
        derived::DerivedUnit {
            base: self,
            ..DerivedUnit::unitless()
        }
    }
}

impl<ExponentType> From<derived::DerivedUnit<ExponentType>> for base::BaseUnit<ExponentType>
where
    ExponentType: UnitExponent,
{
    fn from(value: derived::DerivedUnit<ExponentType>) -> Self {
        value.to_base()
    }
}

impl<ExponentType> derived::DerivedUnit<ExponentType>
where
    ExponentType: UnitExponent,
{
    pub fn to_base(self) -> base::BaseUnit<ExponentType> {
        self.base
            .multiply(base::HERTZ.int_pow(self.hertz))
            .multiply(base::NEWTON.int_pow(self.newton))
            .multiply(base::PASCAL.int_pow(self.pascal))
            .multiply(base::JOULE.int_pow(self.joule))
            .multiply(base::WATT.int_pow(self.watt))
            .multiply(base::COULOMB.int_pow(self.coulomb))
            .multiply(base::VOLT.int_pow(self.volt))
            .multiply(base::FARAD.int_pow(self.farad))
            .multiply(base::OHM.int_pow(self.ohm))
            .multiply(base::SIEMENS.int_pow(self.siemens))
            .multiply(base::WEBER.int_pow(self.weber))
            .multiply(base::TESLA.int_pow(self.tesla))
            .multiply(base::HENRY.int_pow(self.henry))
            .multiply(base::LUX.int_pow(self.lux))
            .multiply(base::BECQUEREL.int_pow(self.becquerel))
            .multiply(base::GRAY.int_pow(self.gray))
            .multiply(base::SIEVERT.int_pow(self.sievert))
            .multiply(base::KATAL.int_pow(self.katal))
    }
}

impl<Number> From<base::BaseValue<Number>> for derived::DerivedValue<Number> {
    fn from(value: base::BaseValue<Number>) -> Self {
        derived::DerivedValue {
            unit: value.unit.to_derived(),
            number: value.number,
        }
    }
}

impl<ExponentType, Number> base::BaseValue<Number, ExponentType>
where
    ExponentType: UnitExponent,
    Number: Copy,
{
    pub fn to_derived(self) -> derived::DerivedValue<Number, ExponentType> {
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

        for identity in basic_identities() {
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
        assert_eq!(derived::JOULE, derived_joule);

        let base_joule = base::KILOGRAM * (base::METER * base::METER * base::METER)
            / (base::SECOND * base::SECOND);
        let actual = base_joule.to_derived().simplify();
        let expected = derived::JOULE * derived::METER;
        assert_eq!(expected, actual);
    }
}
