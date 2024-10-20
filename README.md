# SL (Steam Locomotive) in Rust

This project is a Rust implementation of the classic "SL" (Steam Locomotive) program, originally written in C. It displays a steam locomotive animation using the ncurses library. The original version was created by Toyoda Masashi, and this implementation has been converted to Rust with some modifications.

## Features

- Displays a moving steam locomotive animation in the terminal.
- Supports multiple types of locomotives, including D51 and C51.
- Supports additional options to modify the animation, such as accidents or flying locomotives.

## Dependencies

- **Rust**: This project requires Rust to build and run.
- **ncurses**: The ncurses library is used to handle terminal graphics.

To install the necessary dependencies, run the following command:

```sh
sudo apt-get install libncurses5-dev
```

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/jacoloves/sl-rust.git
   cd sl-rust
   ```

2. Build the project using Cargo:

   ```sh
   cargo build --release
   ```

3. Run the executable:

   ```sh
   cargo run --release -- [options]
   ```

## Usage

The program displays an animated steam locomotive that moves across the terminal. You can specify different options to change the animation behavior.

### Options

- `-a` : Add people shouting for help during an accident.
- `-F` : Make the locomotive "fly" diagonally upwards.
- `-l` : Show the SL logo.
- `-c` : Display the C51 locomotive instead of the default D51.

### Example

Run the steam locomotive animation with the accident feature:

```sh
cargo run --release -- -a
```

## Code Structure

- **sl.rs**: The main Rust source file that defines the animation logic.
- **sl module**: Contains various constants and functions for drawing the different parts of the locomotive, including smoke, wheels, and carriages.

### Key Functions

- `my_mvaddstr(y: i32, x: i32, s: &str)`: A helper function to draw a string at a specific position.
- `add_sl(x: i32)`: Draws the SL locomotive.
- `add_D51(x: i32)`: Draws the D51 locomotive.
- `add_C51(x: i32)`: Draws the C51 locomotive.
- `add_smoke(y: i32, x: i32)`: Draws smoke animation.
- `option(args: &str)`: Parses command-line options.

## Notes

- The program uses `ncurses` for terminal manipulation, which means it must run in a compatible terminal environment.
- Some parts of the code use `unsafe` blocks to manage direct memory access, particularly for handling dynamic smoke effects.

## License

This project is based on the original SL by Toyoda Masashi and is provided for educational and nostalgic purposes. 

Licensed under the MIT License. See [LICENSE](LICENSE) for more details.

