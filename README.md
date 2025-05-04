<div align="center">
  <h1>Physical Units</h1>

  <p>
    <strong>A library for working with values that have physical units in a simple sound way</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/physical-units"><img src="https://img.shields.io/crates/v/physical-units.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/physical-units"><img src="https://img.shields.io/crates/d/physical-units.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/physical-units"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>
</div>

## Base Units

Any SI unit can be represented as the powers of the seven SI base units that multiply to construct it.

| Name | Symbol | Quantity |
|------|--------|----------|
| kilogram | kg | mass |
| meter | m | distance |
| second | s | time |
| mole | mol | amount |
| ampere | A | current |
| kelvin | K | temperature |
| candela | cd | luminosity |

For example, 1 newton (1 N) is equal to one kilogram meter per second squared (1 kg⋅m/s²),
which can be represented a `(1, 1, -2, 0, 0, 0, 0)` where each value corresponds to the exponent
of an SI base unit. This is how the `BaseUnit` type represents units.

## Derived Units

In addition to the base units, there are also SI derived units
which can be useful for talking about common combinations of base units.

| Name | Symbol | Quantity | SI Base Units |
|------|--------|----------|---------------|
| hertz | Hz | frequency | s⁻¹ |
| newton | N | force, weight | kg⋅m⋅s⁻² |
| pascal | Pa | pressure, stress | kg⋅m⁻¹⋅s⁻² |
| joule | J | energy, work, heat | kg⋅m²⋅s⁻² |
| watt | W | power, radiant flux | kg⋅m²⋅s⁻³ |
| coulomb | C | electrical charge | s⋅A |
| volt | V | voltage, electrical potential difference | kg⋅m²⋅s⁻³⋅A⁻¹ |
| farad | F | electrical capacitance | kg⁻¹⋅m⁻2⋅s³⋅A² |
| ohm | Ω | electrical resistance | kg⋅m²⋅s⁻³⋅A⁻² |
| siemens | S | electrical conductance | kg⁻¹⋅m⁻²⋅s³⋅A² |
| weber | Wb | magnetic flux | kg⋅m²⋅s⁻²⋅A⁻¹ |
| tesla | T | magnetic induction | kg⋅s⁻²⋅A⁻¹ | 
| henry | H | electrical inductance | kg⋅m²⋅s⁻²⋅A⁻² |
| lux | lx | illuminance | cd⋅m⁻² |
| becquerel | Bq | radioactivity | s⁻¹ |
| gray | Gy | absorbed dose (of ionizing radiation) | m²⋅s⁻² |
| sievert | Sv | equivalent dose (of ionizing radiation) | m²⋅s⁻² |
| katal | kat | catalytic activity | s⁻¹⋅mol | 

(based on [Wikipedia: SI derived unit - Special names](https://en.wikipedia.org/wiki/SI_derived_unit#Special_names))

These units can all be represented in `BaseUnit` as powers of SI base units,
but the `derived` module also provides a representation that keeps track
of specifically which base and derived units were specified.

Units represented this way are encoded using 7 base exponents
and 18 **additional** exponents for the derived units!
This is **NOT** a minimal encoding of the unit information but
the redundancy allows us to distinguish between `"N"` and `"kg⋅m/s²"`.

## Arithmetic

Both `BaseValue` and `DerivedValue` support basic arithmetic including addition, subtraction, multiplication, and division.

* **Addition** and **subtraction** require the values to have the same units and if they are, not produces a `UnitMismatch` error.
* **Multiplication** and **division** don't require the values to have the same units and infers the correct units for the resulting value.

## Comparison

All of the types (`BaseUnit`, `BaseValue`, `DerivedUnit`, `DerivedValue`) support comparison
and, in the case of the derived ones, perform the necessary conversions to check that they are equal
even if the representations differ.

## Limitations

The exponent for each unit is an 8-bit signed integer (`i8`) which can
encode values in the range `-128..128` (inclusive..exclusive).
This means that fractional unit exponents can't be represented and
if you attempt to multiply or divide `BaseUnit`, `BaseValue`, `DerivedUnit`, or `DerivedValue`
such that any unit exponent leaves this range, you will get a runtime panic.

## Simplifying

The `DerivedUnit` type implements a `simplify()` function that attempts to express
the unit in terms of the fewest exponents using a naive greedy algorithm
which applies identities which reduce the sum of absolute exponents.

## Formatting

All of the unit types have a pretty-printed `Display` implementation.

The `Display` implementation operates at the level of abstraction of the unit type it's for.
* `BaseUnit` - `"kg⋅m/s²"`
* `DerivedUnit` - `"N"` OR `"kg⋅m/s²"` (depending on how it was constructed)

They also have a `Debug` implementation that uses the full names of units and doesn't use `/` or parenthesis.

* `BaseUnit` - `"BaseUnit(kilogram⋅meter⋅second⁻²)"`
* `DerivedUnit` - `"DerivedUnit(newton)"` OR `"DerivedUnit(kilogram⋅meter⋅second⁻²)"`


## Parsing

> TODO: Implement and document parsing for each unit representation

## Read-eval-print-loop (REPL)

> TODO: Implement and document a simple REPL for evaluating expressions with units

## Inspiration

Inspired by the video [Seven Dimensions: Why mass is a vector (and so is everything else) (kind of)](https://www.youtube.com/watch?v=bI-FS7aZJpY), which proposes treating units as a vector space and applying linear algebra concepts to it.
