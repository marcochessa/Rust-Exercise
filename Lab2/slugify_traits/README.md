# Slugify - Traits

This is a Rust module that provides functionality for creating URL-friendly strings (slugs) from normal text, with the addition of traits to check if a string is already a slug and to convert a string into a slug.

## Usage

Add `slugify` to your `Cargo.toml` file:

```toml
[dependencies]
slugify = "0.1.0"
```

Then import and use the `slugify` module in your code:

```rust
use slugify::MySlug;

// Check if a string is a slug
let s1 = String::from("hello-slice");
let s2 = "hello_string";
println!("{}", s1.is_slug()); // true
println!("{}", s2.is_slug()); // false

// Convert a string to a slug
let s3: String = s1.to_slug();
let s4: String = s2.to_slug();
println!("s3: {}, s4: {}", s3, s4); // Output: s3: hello-slice, s4: hello-string
```

## API

- `is_slug() -> bool`: Checks if the string is a slug.
- `to_slug() -> String`: Converts the string to a slug.

## Examples

Usage examples of the module are available in the code documentation and unit tests.
