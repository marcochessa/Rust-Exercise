# Naval Battle 

Welcome to the Naval Battle repository! This project is a Rust application designed to manage a board game resembling the classic game of Battleship. Users can create new game boards and strategically place boats on these boards via a command-line interface (CLI).

## Features

- **Board Management:**
  - Create new game boards with customizable boat configurations.
  - Add boats of various sizes and orientations to existing boards.

- **Error Handling:**
  - Gracefully handles errors such as boat overlap and out-of-bounds placement.
  - Provides informative error messages to assist users.

- **Command-Line Interface (CLI):**
  - Uses the Clap library for parsing command-line arguments.
  - Supports commands for creating new boards (`new`) and adding boats (`add`).

## Usage

### Creating a New Board

```
$ naval-battle new <file> <boats>
```

- `<file>`: Path to the file where the new board will be saved.
- `<boats>`: Number of boats for each size category (1, 2, 3, and 4), separated by commas (e.g., `6,4,3,2`).

### Adding a Boat to a Board

```
$ naval-battle add <file> <boat> <start_pos>
```

- `<file>`: Path to the file of the existing board.
- `<boat>`: Boat type and length in the format `Hx` (horizontal) or `Vx` (vertical), where `x` is the length (e.g., `H3`).
- `<start_pos>`: Start position of the boat in the format `(row, col)`, with the origin at `(1,1)` (e.g., `(2,3)`).

## Getting Started

To get started with the Naval Battle Project, follow these steps:

1. Clone the repository to your local machine:

    ```
   $ git clone https://github.com/your_username/naval-battle.git
    ```

2. Navigate to the project directory:

   ```
    $ cd naval-battle
   ```

3. Build the project using Cargo:

   ```
    $ cargo build --release
   ```

4. Run the application and explore its functionalities:

   ```
    $ cargo run -- <command> <arguments>
   ```
