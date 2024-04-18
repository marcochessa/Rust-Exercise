# CircularBuffer

Questo è un semplice buffer circolare implementato in Rust, che consente l'inserimento, la lettura e l'accesso agli elementi in modo efficiente.

## Utilizzo

Aggiungi `circular_buffer` al tuo file `Cargo.toml`:

```toml
[dependencies]
circular_buffer = "0.1.0"
```

Quindi importa e utilizza il `CircularBuffer` nel tuo codice:

```rust
use circular_buffer::CircularBuffer;

// Crea un nuovo buffer circolare con una capacità di 10 elementi
let mut buffer: CircularBuffer<i32> = CircularBuffer::new(10);

// Scrivi un elemento nel buffer
buffer.write(42).unwrap();

// Leggi un elemento dal buffer
let item = buffer.read();

// Accedi agli elementi del buffer come se fossero un array
let first_element = buffer[0];
```

## API

- `new(capacity: usize) -> Self`: Crea un nuovo buffer circolare con la capacità specificata.
- `write(item: T) -> Result<(), Error>`: Scrive un elemento nel buffer. Restituisce un errore se il buffer è pieno.
- `read() -> Option<T>`: Legge e rimuove un elemento dal buffer. Restituisce `None` se il buffer è vuoto.
- `clear()`: Svuota il buffer.
- `size() -> usize`: Restituisce il numero di elementi presenti nel buffer.
- `overwrite(item: T)`: Scrive un elemento nel buffer, sovrascrivendo il più vecchio se il buffer è pieno.
- `make_contiguous()`: Rende il buffer contiguo, garantendo che gli elementi siano memorizzati in posizioni consecutive di memoria.

## Esempi

Gli esempi di utilizzo del buffer circolare sono disponibili nella documentazione del codice e nei test dell'unità.
