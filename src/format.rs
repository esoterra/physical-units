use core::fmt;

use crate::{base, derived};

impl fmt::Debug for base::BaseUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == base::UNITLESS {
            return write!(f, "BaseUnit::UNITLESS");
        }

        let components = [
            (self.kilogram, "kilogram"),
            (self.meter, "meter"),
            (self.second, "second"),
            (self.mole, "mole"),
            (self.ampere, "ampere"),
            (self.kelvin, "kelvin"),
            (self.candela, "candela"),
        ];

        write!(f, "BaseUnit(")?;

        let mut is_first = true;
        for (n, symbol) in components {
            if n != 0 {
                if !is_first {
                    write!(f, "⋅")?;
                }
                write!(f, "{symbol}{}", SignedSuperscript { n })?;
                is_first = false;
            }
        }

        write!(f, ")")
    }
}

impl fmt::Display for base::BaseUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == base::UNITLESS {
            return write!(f, "Unitless");
        }

        let components = [
            (self.kilogram, "kg"),
            (self.meter, "m"),
            (self.second, "s"),
            (self.mole, "mol"),
            (self.ampere, "A"),
            (self.kelvin, "K"),
            (self.candela, "cd"),
        ];

        let mut is_first = true;
        for (n, symbol) in components.iter() {
            let n = *n;
            if n > 0 {
                if !is_first {
                    write!(f, "⋅")?;
                }
                write!(f, "{symbol}{}", SignedSuperscript { n })?;
                is_first = false;
            }
        }

        let negatives = components.iter().filter(|(n, _)| *n < 0).count();
        if negatives != 0 {
            write!(f, "/")?;

            if negatives > 1 {
                write!(f, "(")?;
            }

            let mut is_first = true;
            for (n, symbol) in components.iter() {
                let n = -*n;
                if n > 0 {
                    if !is_first {
                        write!(f, "⋅")?;
                    }
                    write!(f, "{symbol}{}", SignedSuperscript { n })?;
                    is_first = false;
                }
            }

            if negatives > 1 {
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

impl<Number> fmt::Display for base::BaseValue<Number>
where
    Number: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.unit == base::UNITLESS {
            write!(f, "{}", self.number)
        } else {
            write!(f, "{} {}", self.number, self.unit)
        }
    }
}

impl fmt::Debug for derived::DerivedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == derived::UNITLESS {
            return write!(f, "DerivedUnit::UNITLESS");
        }

        let components = [
            (self.base.kilogram, "kilogram"),
            (self.base.meter, "meter"),
            (self.base.second, "second"),
            (self.base.mole, "mole"),
            (self.base.ampere, "ampere"),
            (self.base.kelvin, "kelvin"),
            (self.base.candela, "candela"),
            (self.hertz, "hertz"),
            (self.newton, "newton"),
            (self.pascal, "pascal"),
            (self.joule, "joule"),
            (self.watt, "watt"),
            (self.coulomb, "coulomb"),
            (self.volt, "volt"),
            (self.farad, "farad"),
            (self.ohm, "ohm"),
            (self.siemens, "siemens"),
            (self.weber, "weber"),
            (self.tesla, "tesla"),
            (self.henry, "henry"),
            (self.lux, "lux"),
            (self.becquerel, "becquerel"),
            (self.gray, "gray"),
            (self.sievert, "sievert"),
            (self.katal, "katal"),
        ];

        write!(f, "DerivedUnit(")?;

        let mut is_first = true;
        for (n, symbol) in components {
            if n != 0 {
                if !is_first {
                    write!(f, "⋅")?;
                }
                write!(f, "{symbol}{}", SignedSuperscript { n })?;
                is_first = false;
            }
        }

        write!(f, ")")
    }
}

impl fmt::Display for derived::DerivedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == derived::UNITLESS {
            return write!(f, "Unitless");
        }

        let components = [
            (self.base.kilogram, "kg"),
            (self.base.meter, "m"),
            (self.base.second, "s"),
            (self.base.mole, "mol"),
            (self.base.ampere, "A"),
            (self.base.kelvin, "K"),
            (self.base.candela, "cd"),
            (self.hertz, "Hz"),
            (self.newton, "N"),
            (self.pascal, "Pa"),
            (self.joule, "J"),
            (self.watt, "W"),
            (self.coulomb, "C"),
            (self.volt, "V"),
            (self.farad, "F"),
            (self.ohm, "Ω"),
            (self.siemens, "S"),
            (self.weber, "Wb"),
            (self.tesla, "T"),
            (self.henry, "H"),
            (self.lux, "lx"),
            (self.becquerel, "Bq"),
            (self.gray, "Gy"),
            (self.sievert, "Sv"),
            (self.katal, "kat"),
        ];

        let mut is_first = true;
        for (n, symbol) in components.iter() {
            let n = *n;
            if n > 0 {
                if !is_first {
                    write!(f, "⋅")?;
                }
                write!(f, "{symbol}{}", SignedSuperscript { n })?;
                is_first = false;
            }
        }

        let negatives = components.iter().filter(|(n, _)| *n < 0).count();
        if negatives != 0 {
            write!(f, "/")?;

            if negatives > 1 {
                write!(f, "(")?;
            }

            let mut is_first = true;
            for (n, symbol) in components.iter() {
                let n = -*n;
                if n > 0 {
                    if !is_first {
                        write!(f, "⋅")?;
                    }
                    write!(f, "{symbol}{}", SignedSuperscript { n })?;
                    is_first = false;
                }
            }

            if negatives > 1 {
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

impl<Number> fmt::Display for derived::DerivedValue<Number>
where
    Number: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.unit == derived::UNITLESS {
            write!(f, "{}", self.number)
        } else {
            write!(f, "{} {}", self.number, self.unit)
        }
    }
}

struct SignedSuperscript {
    n: i8,
}

impl fmt::Display for SignedSuperscript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.n == 0 {
            return write!(f, "⁰");
        }
        if self.n == 1 {
            return Ok(());
        }

        // Handle negatives
        let n = if self.n < 0 {
            write!(f, "⁻")?;
            -self.n as u8
        } else {
            self.n as u8
        };

        // Numbers are in the range of 1-128
        // so we can use a simple version of decimal conversion
        let hundreds = n / 100; // Will always be zero or 1
        let n = n - 100 * hundreds;
        let tens = n / 10; // Will be 0-9
        let n = n - 10 * tens;
        let ones = n; // Will be 0-9

        if hundreds != 0 {
            SuperscriptDigit { digit: hundreds }.fmt(f)?;
        }
        if hundreds != 0 || tens != 0 {
            SuperscriptDigit { digit: tens }.fmt(f)?;
        }
        SuperscriptDigit { digit: ones }.fmt(f)?;
        Ok(())
    }
}

struct SuperscriptDigit {
    digit: u8,
}

impl fmt::Display for SuperscriptDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.digit {
            0 => write!(f, "⁰"),
            1 => write!(f, "¹"),
            2 => write!(f, "²"),
            3 => write!(f, "³"),
            4 => write!(f, "⁴"),
            5 => write!(f, "⁵"),
            6 => write!(f, "⁶"),
            7 => write!(f, "⁷"),
            8 => write!(f, "⁸"),
            9 => write!(f, "⁹"),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_unit_display() {
        let unit = base::BaseUnit {
            meter: 1,
            second: -2,
            mole: 0,
            ampere: 0,
            kelvin: 0,
            candela: 0,
            kilogram: 1,
        };
        assert_eq!(String::from("kg⋅m/s²"), format!("{}", unit));
    }

    #[test]
    fn test_base_value_display() {
        use crate::base::SECOND;
        // Ints
        let one_second = base::BaseValue {
            unit: SECOND,
            number: 1u32,
        };
        assert_eq!(String::from("1 s"), format!("{}", one_second));
        let two_seconds = base::BaseValue {
            unit: SECOND,
            number: 2u32,
        };
        assert_eq!(String::from("2 s"), format!("{}", two_seconds));
        // Floats
        let one_second = base::BaseValue {
            unit: SECOND,
            number: 1f32,
        };
        assert_eq!(String::from("1 s"), format!("{}", one_second));
        let two_seconds = base::BaseValue {
            unit: SECOND,
            number: 2f32,
        };
        assert_eq!(String::from("2 s"), format!("{}", two_seconds));
    }

    #[test]
    fn test_derived_unit_display() {
        let unit = derived::DerivedUnit {
            farad: 1,
            sievert: -2,
            ..derived::UNITLESS
        };
        assert_eq!(String::from("F/Sv²"), format!("{}", unit));
    }

    #[test]
    fn test_derived_unit_debug() {
        let unit = derived::DerivedUnit {
            farad: 1,
            sievert: -2,
            ..derived::UNITLESS
        };
        assert_eq!(String::from("DerivedUnit(farad⋅sievert⁻²)"), format!("{:?}", unit));
    }

    #[test]
    fn test_derived_value_display() {
        use crate::derived::BECQUEREL;
        // Ints
        let one_second = derived::DerivedValue {
            unit: BECQUEREL,
            number: 1u32,
        };
        assert_eq!(String::from("1 Bq"), format!("{}", one_second));
        let two_seconds = derived::DerivedValue {
            unit: BECQUEREL,
            number: 2u32,
        };
        assert_eq!(String::from("2 Bq"), format!("{}", two_seconds));
        // Floats
        let one_second = derived::DerivedValue {
            unit: BECQUEREL,
            number: 1f32,
        };
        assert_eq!(String::from("1 Bq"), format!("{}", one_second));
        let two_seconds = derived::DerivedValue {
            unit: BECQUEREL,
            number: 2f32,
        };
        assert_eq!(String::from("2 Bq"), format!("{}", two_seconds));
    }
}
