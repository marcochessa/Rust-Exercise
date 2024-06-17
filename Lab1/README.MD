# Lab 1

## Exercise 1: Slugify

### Objectives
- String handling and slicing
- Difference between “characters” (char) and “bytes” (u8) in Rust
- Reading command-line arguments

### Description
A “slug” is a string converted into a readable format, consisting only of the characters `[a-z][0-9]-`. The conversion rules are:
- Accented characters are converted to their non-accented equivalents.
- Everything is converted to lowercase.
- All other characters are converted to “-”.
- Consecutive “-” are reduced to a single “-”.
- A final “-” is not allowed unless it is the only character in the string.

### Implementation

#### Solution Steps

1. Create a new package named `slugify` and define the `slugify` function:
    ```rust
    fn slugify(s: &str) -> String {}
    ```

2. Define a function `conv` to convert accented letters:
    ```rust
    fn conv(c: char) -> char {}
    ```
   Use this conversion table:
    ```rust
    const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";
    ```

3. Write unit tests for the created functions:
    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_accented_letter_conversion() { /* ... */ }
        #[test]
        fn test_non_accented_letter_conversion() { /* ... */ }
        // More tests here...
    }
    ```

4. Make the function callable from the command line:
    - Add the `clap` library to the project:
      ```toml
      [dependencies]
      clap = { version = "4.5.3", features = ["derive"] }
      ```
    - Define a struct `Args`:
      ```rust
      #[derive(Parser, Debug)]
      struct Args {
          slug_in: String,
      }
      ```
    - Parse the parameters:
   
    ```rust
      let args = Args::parse();
      let slug = slugify(&args.slug_in);
      println!("slug: {}", slug);
    ```

### Example Invocation
```sh
cargo run -- "What will this slug be???"
```
Expected result:
```sh
slug: what-will-this-slug-be
```

## Exercise 2:  Naval Battle - Board Management

### Objectives
- Array usage
- String parsing
- Mutability
- Use of structs and enums
- Handling return values and errors
- File read/write

### Description
A program must manage the construction of a 20x20 battleship board saved to a file.

### File Format
- **Line 1:** Four integers separated by spaces indicating the number of ships of lengths 1, 2, 3, and 4 respectively.
- **Lines 2-21:** 20 rows of 20 characters with “ ” (space) for empty cells and “B” for cells with ships.

### Example Usage
```sh
cargo run -- new board.txt 4,3,2,1
cargo run -- add board.txt 3V 10,10
```

### Implementation

#### Structure
```rust
const BSIZE: usize = 20;

pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}

pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}

pub enum Boat {
    Vertical(usize),
    Horizontal(usize),
}

impl Board {
    pub fn new(boats: &[u8]) -> Board { /* ... */ }
    pub fn from(s: String) -> Board { /* ... */ }
    pub fn add_boat(self, boat: Boat, pos: (usize, usize)) -> Result<Board, Error> { /* ... */ }
    pub fn to_string(&self) -> String { /* ... */ }
}
```

### Preparatory Exercises

1. **File Handling**
    - Read a file and repeat the text 10 times.
    - Handle file not found errors.

2. **Enums with Values**
    - Define an `enum Error` and a function `print_error`.

3. **Error Handling**
    - Implement a function `mul` that returns `Result`.

4. **Builder Pattern**
    - Define methods `size`, `count`, `to_string`, `grow`, `inc` for the struct `Node`.



### Bonus Exercises

1. **Robot Simulator**
2. **Clock**
