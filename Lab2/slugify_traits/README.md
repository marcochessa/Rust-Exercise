# Slugify - Traits

Questo è un modulo Rust che fornisce funzionalità per la creazione di URL friendly stringhe (slug) da testo normale, con l'aggiunta di tratti per verificare se una stringa è già uno slug e per convertire una stringa in uno slug.

## Utilizzo

Aggiungi `slugify` al tuo file `Cargo.toml`:

```toml
[dependencies]
slugify = "0.1.0"
```

Quindi importa e utilizza il modulo `slugify` nel tuo codice:

```rust
use slugify::MySlug;

// Verifica se una stringa è uno slug
let s1 = String::from("hello-slice");
let s2 = "hello_string";
println!("{}", s1.is_slug()); // true
println!("{}", s2.is_slug()); // false

// Converti una stringa in uno slug
let s3: String = s1.to_slug();
let s4: String = s2.to_slug();
println!("s3: {}, s4: {}", s3, s4); // stampa: s3: hello-slice, s4: hello-string
```

## API

- `is_slug() -> bool`: Verifica se la stringa è uno slug.
- `to_slug() -> String`: Converte la stringa in uno slug.

## Esempi

Gli esempi di utilizzo del modulo sono disponibili nella documentazione del codice e nei test dell'unità.
