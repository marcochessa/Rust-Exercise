# CircularBuffer

This is a simple circular buffer implemented in Rust, allowing efficient insertion, reading, and access to elements.

## Usage

Add `circular_buffer` to your `Cargo.toml` file:

```toml
[dependencies]
circular_buffer = "0.1.0"
```

Then import and use `CircularBuffer` in your code:

```rust
use circular_buffer::CircularBuffer;

// Create a new circular buffer with a capacity of 10 elements
let mut buffer: CircularBuffer<i32> = CircularBuffer::new(10);

// Write an item to the buffer
buffer.write(42).unwrap();

// Read an item from the buffer
let item = buffer.read();

// Access buffer elements as if they were an array
let first_element = buffer[0];
```

## API

- `new(capacity: usize) -> Self`: Creates a new circular buffer with the specified capacity.
- `write(item: T) -> Result<(), Error>`: Writes an item to the buffer. Returns an error if the buffer is full.
- `read() -> Option<T>`: Reads and removes an item from the buffer. Returns `None` if the buffer is empty.
- `clear()`: Empties the buffer.
- `size() -> usize`: Returns the number of elements present in the buffer.
- `overwrite(item: T)`: Writes an item to the buffer, overwriting the oldest one if the buffer is full.
- `make_contiguous()`: Makes the buffer contiguous, ensuring that elements are stored in consecutive memory positions.

## Examples

Usage examples of the circular buffer are available in the code documentation and unit tests.
