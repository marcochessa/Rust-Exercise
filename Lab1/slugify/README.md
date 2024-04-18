# Slugify

Questo è un semplice programma in Rust per la creazione di URL friendly stringhe (slug) da testo normale. Converte caratteri speciali, accenti e spazi in caratteri alfanumerici e trattini.

## Dipendenze

- [Clap](https://crates.io/crates/clap): Una libreria per gestire gli argomenti della riga di comando in Rust.

## Utilizzo

Assicurati di avere Rust e Cargo installati nel tuo sistema. Puoi installarli seguendo le [istruzioni ufficiali](https://www.rust-lang.org/learn/get-started).

1. Clona questo repository sul tuo computer.
2. Naviga nella directory del progetto.
3. Compila il programma eseguendo `cargo build --release`.
4. Esegui il programma con `cargo run -- <stringa>` dove `<stringa>` è la stringa da slugificare.

## Esempi

```bash
    cargo run -- "Ciao mondo"    # Output: ciao-mondo
    cargo run -- "àéè"           # Output: aee
    cargo run -- "C#a#o"         # Output: c-a-o
```

## Test

Il progetto include anche una suite di test per verificare il corretto funzionamento delle funzioni.

Per eseguire i test, esegui il comando:

```bash
    cargo test
```
