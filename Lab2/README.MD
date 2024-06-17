# Lab 2

## Exercise 1 - Slugify with Trait

### Objectives:
- Polymorphism in Rust
- Extending standard library types
- Generic default trait implementations
- Enforcing trait dependencies on generic types

Extend the project from Exercise 1 (slugifying strings) by defining and implementing a `MySlug` trait for strings and slices. This trait should define methods that allow the following code to compile and produce the indicated results:

```rust
let s1 = String::from("Hello String");
let s2 = "hello-slice";
println!("{}", s1.is_slug()); // false
println!("{}", s2.is_slug()); // true

let s3: String = s1.to_slug();
let s4: String = s2.to_slug();
println!("s3:{} s4:{}", s3, s4); // prints: s3:hello-string s4:hello-slice
```

### Solution Steps

1. **Add `is_slug` function to the existing module**:
   Define a trait `MySlug`:
    ```rust
    pub trait MySlug {
        fn is_slug(&self) -> bool;
        fn to_slug(&self) -> String;
    }
    ```

2. **Provide separate implementations for `String` and `&str`**:
    ```rust
    impl MySlug for String {
        fn is_slug(&self) -> bool {
            // Implementation here
        }

        fn to_slug(&self) -> String {
            // Implementation here
        }
    }

    impl MySlug for &str {
        fn is_slug(&self) -> bool {
            // Implementation here
        }

        fn to_slug(&self) -> String {
            // Implementation here
        }
    }
    ```

3. **Combine the implementations into a single generic one**:
    ```rust
    impl<T> MySlug for T
    where
        T: AsRef<str>,
    {
        fn is_slug(&self) -> bool {
            let s = self.as_ref();
            // Check if `s` is a valid slug
        }

        fn to_slug(&self) -> String {
            let s = self.as_ref();
            // Convert `s` to a slug
        }
    }
    ```

### Notes:
- The trait `AsRef<str>` allows the implementation to work for any type that can be converted to a string slice, thus making the trait available for all such types, including user-defined types.

## Exercise 2 - ComplexNumber

### Objectives:
- Extended overview of standard library traits
- Creating a module in Rust

Implement a `ComplexNumber` type that supports basic arithmetic operators (+, -, etc.), can be copied, cloned, compared with itself and a real number, and used within Rust standard collections (vectors, hashmaps, dequeues).

### Implementation Steps

1. **Create a new Rust project** (e.g., `cnumbers`):
    ```sh
    cargo new cnumbers
    ```

2. **Create the `tests` directory** in the project root if it doesn't exist.

3. **Copy the test file** (`complex_numbers.rs`) into the `tests` directory.

4. **Comment out all tests except the first one** to ensure initial compilation.

5. **In `lib.rs`**, create a module `solution` and define the `ComplexNumber` struct:
    ```rust
    pub mod solution {
        #[derive(Copy, Clone, PartialEq, Debug)]
        pub struct ComplexNumber {
            pub real: f64,
            pub imag: f64,
        }

        impl ComplexNumber {
            pub fn new(real: f64, imag: f64) -> Self {
                ComplexNumber { real, imag }
            }
        }
    }
    ```

6. **Implement the necessary traits** to pass the tests one by one:
    - Arithmetic traits (`Add`, `Sub`, etc.)
    - Other traits as required by the tests

7. **Uncomment subsequent tests** and implement corresponding trait functions, following the order in the test file to handle dependencies correctly.

8. **Run the tests** for the module:
    ```sh
    cargo test --package cnumbers --test complex_numbers
    ```

### Example of a trait implementation for addition:
```rust
use std::ops::Add;

impl Add for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, other: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}
```

## Exercise 3 - Circular Buffer

Implement a circular buffer with a fixed and finite size, allowing element insertion at the tail and extraction from the head. Handle conditions when the buffer is full or empty and ensure proper wrapping of head and tail pointers.
