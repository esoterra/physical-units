use crate::base::BaseValue;
use crate::derived::DerivedValue;
use crate::exponents::UnitExponent;

macro_rules! impl_sqrt {
    ($type:ident, $sqrt:ident) => {
        impl<ExponentType> BaseValue<$type, ExponentType>
        where
            ExponentType: UnitExponent
        {
            pub fn sqrt(self) -> Self {
              Self {
                unit: self.unit.root(2),
                number: self.number.$sqrt()
              }
            }
        }

        impl<ExponentType> DerivedValue<$type, ExponentType>
        where
            ExponentType: UnitExponent
        {
            pub fn sqrt(self) -> Self {
              Self {
                unit: self.unit.root(2),
                number: self.number.$sqrt()
              }
            }
        }
    };
}

impl_sqrt!(f32, sqrt);
impl_sqrt!(f64, sqrt);

impl_sqrt!(i8, isqrt);
impl_sqrt!(i16, isqrt);
impl_sqrt!(i32, isqrt);
impl_sqrt!(i64, isqrt);

impl_sqrt!(u8, isqrt);
impl_sqrt!(u16, isqrt);
impl_sqrt!(u32, isqrt);
impl_sqrt!(u64, isqrt);

#[cfg(test)]
mod tests {
  use crate::{base::{self, BaseUnit}, exponents::FractionalExponent};
  use super::*;

  fn test_sqrts<Unit: Into<BaseUnit<FractionalExponent>>>(input: u8, input_unit: Unit, output: u8, output_unit: Unit) {
    let input_unit = input_unit.into();
    let output_unit = output_unit.into();

    let input_f32 = BaseValue { unit: input_unit, number: input as f32 };
    let output_f32 = BaseValue { unit: output_unit, number: output as f32 };
    assert_eq!(output_f32, input_f32.sqrt());

    let input_f64 = BaseValue { unit: input_unit, number: input as f64 };
    let output_f64 = BaseValue { unit: output_unit, number: output as f64 };
    assert_eq!(output_f64, input_f64.sqrt());

    let input_u8 = BaseValue { unit: input_unit, number: input as u8 };
    let output_u8 = BaseValue { unit: output_unit, number: output as u8 };
    assert_eq!(output_u8, input_u8.sqrt());
  }

  #[test]
  fn simple_sqrts() {
    test_sqrts(100, base::METER * base::METER, 10, base::METER);
  }
}