use std::ops::Mul;

use crate::{base, derived};

impl<Number> base::BaseValue<Number> {
    pub const fn kilograms(number: Number) -> Self {
        Self {
            unit: base::KILOGRAM,
            number,
        }
    }

    pub const fn meters(number: Number) -> Self {
        Self {
            unit: base::METER,
            number,
        }
    }

    pub const fn seconds(number: Number) -> Self {
        Self {
            unit: base::SECOND,
            number,
        }
    }

    pub const fn moles(number: Number) -> Self {
        Self {
            unit: base::MOLE,
            number,
        }
    }

    pub const fn amperes(number: Number) -> Self {
        Self {
            unit: base::AMPERE,
            number,
        }
    }

    pub const fn kelvin(number: Number) -> Self {
        Self {
            unit: base::KELVIN,
            number,
        }
    }

    pub const fn candela(number: Number) -> Self {
        Self {
            unit: base::CANDELA,
            number,
        }
    }
}

impl<Number> base::BaseValue<Number> {
    pub const fn hertz(number: Number) -> Self {
        Self {
            unit: base::HERTZ,
            number,
        }
    }

    pub const fn newton(number: Number) -> Self {
        Self {
            unit: base::NEWTON,
            number,
        }
    }

    pub const fn pascal(number: Number) -> Self {
        Self {
            unit: base::PASCAL,
            number,
        }
    }

    pub const fn joules(number: Number) -> Self {
        Self {
            unit: base::JOULE,
            number,
        }
    }

    pub const fn coulombs(number: Number) -> Self {
        Self {
            unit: base::COULOMB,
            number,
        }
    }

    pub const fn volts(number: Number) -> Self {
        Self {
            unit: base::VOLT,
            number,
        }
    }

    pub const fn farads(number: Number) -> Self {
        Self {
            unit: base::FARAD,
            number,
        }
    }

    pub const fn ohms(number: Number) -> Self {
        Self {
            unit: base::OHM,
            number,
        }
    }

    pub const fn siemens(number: Number) -> Self {
        Self {
            unit: base::SIEMENS,
            number,
        }
    }

    pub const fn webers(number: Number) -> Self {
        Self {
            unit: base::WEBER,
            number,
        }
    }

    pub const fn teslas(number: Number) -> Self {
        Self {
            unit: base::TESLA,
            number,
        }
    }

    pub const fn henries(number: Number) -> Self {
        Self {
            unit: base::HENRY,
            number,
        }
    }

    pub const fn lux(number: Number) -> Self {
        Self {
            unit: base::LUX,
            number,
        }
    }

    pub const fn becquerels(number: Number) -> Self {
        Self {
            unit: base::BECQUEREL,
            number,
        }
    }

    pub const fn grays(number: Number) -> Self {
        Self {
            unit: base::GRAY,
            number,
        }
    }

    pub const fn sieverts(number: Number) -> Self {
        Self {
            unit: base::SIEVERT,
            number,
        }
    }

    pub const fn katals(number: Number) -> Self {
        Self {
            unit: base::KATAL,
            number,
        }
    }
}

impl<Number> derived::DerivedValue<Number> {
    pub const fn kilograms(number: Number) -> Self {
        Self {
            unit: derived::KILOGRAM,
            number,
        }
    }

    pub const fn meters(number: Number) -> Self {
        Self {
            unit: derived::METER,
            number,
        }
    }

    pub const fn seconds(number: Number) -> Self {
        Self {
            unit: derived::SECOND,
            number,
        }
    }

    pub const fn moles(number: Number) -> Self {
        Self {
            unit: derived::MOLE,
            number,
        }
    }

    pub const fn amperes(number: Number) -> Self {
        Self {
            unit: derived::AMPERE,
            number,
        }
    }

    pub const fn kelvin(number: Number) -> Self {
        Self {
            unit: derived::KELVIN,
            number,
        }
    }

    pub const fn candela(number: Number) -> Self {
        Self {
            unit: derived::CANDELA,
            number,
        }
    }
}

impl<Number> derived::DerivedValue<Number> {
    pub const fn hertz(number: Number) -> Self {
        Self {
            unit: derived::HERTZ,
            number,
        }
    }

    pub const fn newton(number: Number) -> Self {
        Self {
            unit: derived::NEWTON,
            number,
        }
    }

    pub const fn pascal(number: Number) -> Self {
        Self {
            unit: derived::PASCAL,
            number,
        }
    }

    pub const fn joules(number: Number) -> Self {
        Self {
            unit: derived::JOULE,
            number,
        }
    }

    pub const fn coulombs(number: Number) -> Self {
        Self {
            unit: derived::COULOMB,
            number,
        }
    }

    pub const fn volts(number: Number) -> Self {
        Self {
            unit: derived::VOLT,
            number,
        }
    }

    pub const fn farads(number: Number) -> Self {
        Self {
            unit: derived::FARAD,
            number,
        }
    }

    pub const fn ohms(number: Number) -> Self {
        Self {
            unit: derived::OHM,
            number,
        }
    }

    pub const fn siemens(number: Number) -> Self {
        Self {
            unit: derived::SIEMENS,
            number,
        }
    }

    pub const fn webers(number: Number) -> Self {
        Self {
            unit: derived::WEBER,
            number,
        }
    }

    pub const fn teslas(number: Number) -> Self {
        Self {
            unit: derived::TESLA,
            number,
        }
    }

    pub const fn henries(number: Number) -> Self {
        Self {
            unit: derived::HENRY,
            number,
        }
    }

    pub const fn lux(number: Number) -> Self {
        Self {
            unit: derived::LUX,
            number,
        }
    }

    pub const fn becquerels(number: Number) -> Self {
        Self {
            unit: derived::BECQUEREL,
            number,
        }
    }

    pub const fn grays(number: Number) -> Self {
        Self {
            unit: derived::GRAY,
            number,
        }
    }

    pub const fn sieverts(number: Number) -> Self {
        Self {
            unit: derived::SIEVERT,
            number,
        }
    }

    pub const fn katals(number: Number) -> Self {
        Self {
            unit: derived::KATAL,
            number,
        }
    }
}


impl<Number> base::BaseValue<Number>
where
    Number: Mul<Output = Number>,
    Number: From<u16>
{
    pub fn minutes(number: Number) -> Self {
        Self {
            unit: base::SECOND,
            number: number * (60u16.into()),
        }
    }

    pub fn hours(number: Number) -> Self {
        Self {
            unit: base::SECOND,
            number: number * (3600u16.into()),
        }
    }
}


impl<Number> derived::DerivedValue<Number>
where
    Number: Mul<Output = Number>,
    Number: From<u16>
{
    pub fn minutes(number: Number) -> Self {
        Self {
            unit: derived::SECOND,
            number: number * (60u16.into()),
        }
    }

    pub fn hours(number: Number) -> Self {
        Self {
            unit: derived::SECOND,
            number: number * (3600u16.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{base, derived};

    #[test]
    fn test_time_base_int() {
        assert_eq!(base::BaseValue::seconds(60), base::BaseValue::minutes(1));
        assert_eq!(base::BaseValue::seconds(120), base::BaseValue::minutes(2));
        assert_eq!(base::BaseValue::seconds(180), base::BaseValue::minutes(3));
        assert_eq!(base::BaseValue::seconds(240), base::BaseValue::minutes(4));
        assert_eq!(base::BaseValue::seconds(300), base::BaseValue::minutes(5));
        assert_eq!(base::BaseValue::seconds(360), base::BaseValue::minutes(6));
    }

    #[test]
    fn test_time_derived_int() {
        assert_eq!(derived::DerivedValue::seconds(60), derived::DerivedValue::minutes(1));
        assert_eq!(derived::DerivedValue::seconds(120), derived::DerivedValue::minutes(2));
        assert_eq!(derived::DerivedValue::seconds(180), derived::DerivedValue::minutes(3));
        assert_eq!(derived::DerivedValue::seconds(240), derived::DerivedValue::minutes(4));
        assert_eq!(derived::DerivedValue::seconds(300), derived::DerivedValue::minutes(5));
        assert_eq!(derived::DerivedValue::seconds(360), derived::DerivedValue::minutes(6));
    }

    #[test]
    fn test_time_base_float() {
        // floating point
        assert_eq!(base::BaseValue::seconds(0f32), base::BaseValue::minutes(0f32));
        assert_eq!(base::BaseValue::seconds(15f32), base::BaseValue::minutes(0.25f32));
        assert_eq!(base::BaseValue::seconds(30f32), base::BaseValue::minutes(0.5f32));
        assert_eq!(base::BaseValue::seconds(45f32), base::BaseValue::minutes(0.75f32));
        assert_eq!(base::BaseValue::seconds(60f32), base::BaseValue::minutes(1f32));
        assert_eq!(base::BaseValue::seconds(75f32), base::BaseValue::minutes(1.25f32));
        assert_eq!(base::BaseValue::seconds(90f32), base::BaseValue::minutes(1.5f32));

        assert_eq!(base::BaseValue::seconds(0f32), base::BaseValue::hours(0f32));
        assert_eq!(base::BaseValue::seconds(900f32), base::BaseValue::hours(0.25f32));
        assert_eq!(base::BaseValue::seconds(1800f32), base::BaseValue::hours(0.5f32));
        assert_eq!(base::BaseValue::seconds(2700f32), base::BaseValue::hours(0.75f32));
        assert_eq!(base::BaseValue::seconds(3600f32), base::BaseValue::hours(1f32));
        assert_eq!(base::BaseValue::seconds(4500f32), base::BaseValue::hours(1.25f32));
        assert_eq!(base::BaseValue::seconds(5400f32), base::BaseValue::hours(1.5f32));
    }

    #[test]
    fn test_time_derived_float() {
        // floating point
        assert_eq!(derived::DerivedValue::seconds(0f32), derived::DerivedValue::minutes(0f32));
        assert_eq!(derived::DerivedValue::seconds(15f32), derived::DerivedValue::minutes(0.25f32));
        assert_eq!(derived::DerivedValue::seconds(30f32), derived::DerivedValue::minutes(0.5f32));
        assert_eq!(derived::DerivedValue::seconds(45f32), derived::DerivedValue::minutes(0.75f32));
        assert_eq!(derived::DerivedValue::seconds(60f32), derived::DerivedValue::minutes(1f32));
        assert_eq!(derived::DerivedValue::seconds(75f32), derived::DerivedValue::minutes(1.25f32));
        assert_eq!(derived::DerivedValue::seconds(90f32), derived::DerivedValue::minutes(1.5f32));

        assert_eq!(derived::DerivedValue::seconds(0f32), derived::DerivedValue::hours(0f32));
        assert_eq!(derived::DerivedValue::seconds(900f32), derived::DerivedValue::hours(0.25f32));
        assert_eq!(derived::DerivedValue::seconds(1800f32), derived::DerivedValue::hours(0.5f32));
        assert_eq!(derived::DerivedValue::seconds(2700f32), derived::DerivedValue::hours(0.75f32));
        assert_eq!(derived::DerivedValue::seconds(3600f32), derived::DerivedValue::hours(1f32));
        assert_eq!(derived::DerivedValue::seconds(4500f32), derived::DerivedValue::hours(1.25f32));
        assert_eq!(derived::DerivedValue::seconds(5400f32), derived::DerivedValue::hours(1.5f32));
    }
}
