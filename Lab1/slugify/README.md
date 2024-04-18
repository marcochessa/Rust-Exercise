# Slugify

This is a simple Rust program for creating URL-friendly strings (slugs) from normal text. It converts special characters, accents, and spaces into alphanumeric characters and dashes.

## Dependencies

- [Clap](https://crates.io/crates/clap): A library for handling command-line arguments in Rust.

## Usage

Make sure you have Rust and Cargo installed on your system. You can install them by following the [official instructions](https://www.rust-lang.org/learn/get-started).

1. Clone this repository to your computer.
2. Navigate to the project directory.
3. Compile the program by running `cargo build --release`.
4. Run the program with `cargo run -- <string>` where `<string>` is the string to slugify.

## Examples

```bash
    cargo run -- "Ciao mondo"    # Output: ciao-mondo
    cargo run -- "àéè"           # Output: aee
    cargo run -- "C#a#o"         # Output: c-a-o
```

## Testing

The project also includes a test suite to verify the correct functionality of the functions.

To run the tests, execute the command:

```bash
    cargo test
```
