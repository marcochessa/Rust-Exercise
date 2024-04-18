# ComplexNumber

Questo è un modulo in Rust che fornisce un'implementazione di numeri complessi, con funzionalità come costruzione, accesso ai componenti reali e immaginari, calcolo del modulo, confronto, e operazioni aritmetiche.

## Utilizzo

Aggiungi `solution` al tuo file `Cargo.toml`:

```toml
[dependencies]
solution = "0.1.0"
```

Quindi importa e utilizza il modulo `solution` nel tuo codice:

```rust
use solution::ComplexNumber;

// Crea un nuovo numero complesso
let num = ComplexNumber::new(3.0, 4.0);

// Ottieni il valore reale e immaginario
let real_part = num.real();
let imag_part = num.imag();
println!("Parte reale: {}, Parte immaginaria: {}", real_part, imag_part);

// Calcola il modulo del numero complesso
let modulus = num.modulus();
println!("Modulo: {}", modulus);
```

## API

- `new(real: f64, imag: f64) -> Self`: Crea un nuovo numero complesso con la parte reale e la parte immaginaria specificate.
- `from_real(real: f64) -> Self`: Crea un nuovo numero complesso con solo la parte reale specificata.
- `real() -> f64`: Restituisce la parte reale del numero complesso.
- `imag() -> f64`: Restituisce la parte immaginaria del numero complesso.
- `to_tuple() -> (f64, f64)`: Restituisce una tupla contenente la parte reale e la parte immaginaria del numero complesso.
- `modulus() -> f64`: Calcola e restituisce il modulo del numero complesso.
- Operatori aritmetici supportati: `+`, `+=` per l'addizione di numeri complessi e scalari.

## Esempi

Gli esempi di utilizzo dei numeri complessi sono disponibili nella documentazione del codice e nei test dell'unità.