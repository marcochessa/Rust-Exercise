## Implementation Details

### Circular Buffer

The `circbuf` module defines a generic structure for the circular buffer with the following main components:

- **Struct `CircularBuffer<T>`**: Manages the circular buffer logic, including methods for reading and writing elements.
- **Methods**:
    - `new(capacity: usize)`: Creates a new circular buffer.
    - `write(&mut self, element: T) -> Result<(), Error<T>>`: Writes an element to the buffer.
    - `read(&mut self) -> Result<T, Error<T>>`: Reads an element from the buffer.
    - `clear(&mut self)`: Clears the buffer.
    - `overwrite(&mut self, element: T)`: Overwrites an element in the buffer if full.
    - `make_contiguous(&mut self)`: Makes the buffer contiguous.

### Thread-Safe Circular Buffer

The `syncbuf` module defines a thread-safe version of the circular buffer using a mutex:

- **Struct `SyncBuffer<T>`**:
    - **Field**: `buf` (Mutex<CircularBuffer<T>>).
    - **Methods**:
        - `new(capacity: usize) -> Self`: Creates a new thread-safe circular buffer.
        - `write(&self, element: T) -> Result<(), circbuf::Error<T>>`: Writes an element to the buffer.
        - `read(&self) -> Result<T, circbuf::Error<T>>`: Reads an element from the buffer.

### Blocking Thread-Safe Circular Buffer

The `blockingsyncbuf` module extends `SyncBuffer` by adding a `Condvar` for blocking on read and write operations:

- **Struct `BlockingSyncBuf<T>`**:
    - **Fields**: `buf` (Mutex<CircularBuffer<T>>), `cv` (Condvar).
    - **Methods**:
        - `new(capacity: usize) -> Self`: Creates a new blocking buffer.
        - `read_blocking(&self) -> T`: Reads an element from the buffer, blocking if empty.
        - `write_blocking(&self, element: T)`: Writes an element to the buffer, blocking if full.
