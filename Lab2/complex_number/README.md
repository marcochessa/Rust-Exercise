# ComplexNumber

This is a Rust module that provides an implementation of complex numbers, with features such as construction, access to real and imaginary components, modulus calculation, comparison, and arithmetic operations.

## Usage

Add `solution` to your `Cargo.toml` file:

```toml
[dependencies]
solution = "0.1.0"
```

Then import and use the `solution` module in your code:

```rust
use solution::ComplexNumber;

// Create a new complex number
let num = ComplexNumber::new(3.0, 4.0);

// Get the real and imaginary parts
let real_part = num.real();
let imag_part = num.imag();
println!("Real part: {}, Imaginary part: {}", real_part, imag_part);

// Calculate the modulus of the complex number
let modulus = num.modulus();
println!("Modulus: {}", modulus);
```

## API

- `new(real: f64, imag: f64) -> Self`: Creates a new complex number with the specified real and imaginary parts.
- `from_real(real: f64) -> Self`: Creates a new complex number with only the specified real part.
- `real() -> f64`: Returns the real part of the complex number.
- `imag() -> f64`: Returns the imaginary part of the complex number.
- `to_tuple() -> (f64, f64)`: Returns a tuple containing the real and imaginary parts of the complex number.
- `modulus() -> f64`: Calculates and returns the modulus of the complex number.
- Supported arithmetic operators: `+`, `+=` for addition of complex numbers and scalars.

## Examples

Usage examples of complex numbers are available in the code documentation and unit tests.
