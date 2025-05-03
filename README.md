# Physical Units

A library for working with values that have physical units in a simple sound way.

## Base Units

Any SI unit can be represented as the powers of the seven SI base units that multiply to construct it.

* Mass - kilogram (kg)
* Distance - meter (m)
* Time - second (s)
* Amount - mole (mol)
* Current - ampere (A)
* Temperature - kelvin (K)
* Luminosity - candela (cd)

For example, 1 newton (1 N) is equal to one kilogram meter per second squared (1 kg m / s²),
which can be represented a `(1, 1, -2, 0, 0, 0, 0)` where each value corresponds to the exponent
of an SI base unit.

The `BaseUnit` type encodes a type in this way and the `BaseValue<Number>` type wraps a number type
in a struct with a `BaseUnit` and provides addition, subtraction, multiplication, and division.

`BaseUnit` has a typical derived `Debug` implementation and a `Display` implementation
that produces output like `kg m / s²`.

## Derived Units

TODO

## Inspiration

Inspired by the video [Seven Dimensions: Why mass is a vector (and so is everything else) (kind of)](https://www.youtube.com/watch?v=bI-FS7aZJpY), which proposes treating units as a vector space and applying linear algebra concepts to it.
