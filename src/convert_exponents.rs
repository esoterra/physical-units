//! This module lets us turn int exponents into fractional

use crate::{base::BaseUnit, exponents::FractionalExponent};


impl From<BaseUnit> for BaseUnit<FractionalExponent> {
    fn from(value: BaseUnit) -> Self {
        BaseUnit {
          kilogram: FractionalExponent::from_int(value.kilogram),
          meter: FractionalExponent::from_int(value.meter),
          second: FractionalExponent::from_int(value.second),
          mole: FractionalExponent::from_int(value.mole),
          ampere: FractionalExponent::from_int(value.ampere),
          kelvin: FractionalExponent::from_int(value.kelvin),
          candela: FractionalExponent::from_int(value.candela)
        }
    }
}