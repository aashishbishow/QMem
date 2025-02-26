```markdown
# QMem

**QMem** is a quantum-inspired memory simulation library written in Rust. It allows bits to exist in a state of superposition—simultaneously 0 and 1—until measured. This unique approach lets you experiment with quantum-like behavior using a simple, intuitive API.

---

## Features

- **Quantum-Inspired Simulation**: Bits can be in a definite state (0 or 1) or remain in superposition.
- **Simple API**: Easily set, measure, and display the state of your quantum-inspired memory.
- **64-Bit Memory**: Simulate a memory array of 64 bits with efficient, low-level manipulation.
- **Randomized Collapse**: When measured, bits in superposition randomly collapse to 0 or 1.
- **Rust-Powered Performance**: Built with Rust for speed, safety, and concurrency.

---

## Installation

Add QMem as a dependency in your `Cargo.toml`:

```toml
[dependencies]
qmem = "0.1.3"
```

Then import it in your Rust project:

```rust
use QMem::QMem;
```

---

## Usage

Here's a simple example demonstrating how to use QMem:

```rust
use QMem::QMem;

fn main() {
    // Create a new QMem instance with all bits in superposition.
    let mut q_array = QMem::new();

    // Set bit 0 to a definite value (1).
    q_array.set_bit(0, Some(true));

    // Set bit 1 to a definite value (0).
    q_array.set_bit(1, Some(false));

    // Bit 2 remains in superposition (default state).

    // Measure bits:
    let bit0 = q_array.measure(0); // Always returns true (1).
    let bit1 = q_array.measure(1); // Always returns false (0).
    let bit2 = q_array.measure(2); // Randomly returns true or false.

    println!("Bit 0: {}", bit0);
    println!("Bit 1: {}", bit1);
    println!("Bit 2: {}", bit2);

    // Print the current state of the memory.
    q_array.print();
}
```

---

## API Reference

### `QMem::new() -> Self`
Creates a new `QMem` instance with all 64 bits initialized in superposition.

### `set_bit(&mut self, index: usize, value: Option<bool>)`
Sets the bit at the specified index:
- `Some(true)` sets the bit to **1**.
- `Some(false)` sets the bit to **0**.
- `None` leaves the bit in superposition.

### `measure(&mut self, index: usize) -> bool`
Measures the bit at the given index:
- If the bit is already in a definite state, returns that value.
- If the bit is in superposition, randomly collapses it to 0 or 1 and returns the result.

### `print(&self)`
Prints the current state of the memory, displaying:
- The definite state of each bit (0s and 1s).
- Which bits are still in superposition.

---

## Implementation Details

QMem manages two 64-bit integers internally:
- **state**: Holds the current collapsed values of the bits.
- **superposition**: Uses a bitmask to indicate which bits remain in superposition (a 1 means the bit is still in superposition).

The randomness during measurement is handled by the `rand` crate (version 0.9).

---

## Contributing

Contributions are very welcome! If you have ideas, bug fixes, or improvements, please feel free to open an issue or submit a pull request on [GitHub](https://github.com/aashishbishow/QMem).

---

## License

This project is licensed under the [MIT License](LICENSE).

---

## Authors

- **Aashish BishowKarma** – Initial work  
  [GitHub Profile](https://github.com/aashishbishow)

---

Enjoy exploring quantum-inspired memory simulation with QMem!
```