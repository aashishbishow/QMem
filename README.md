# QMem

A quantum-inspired memory simulation library in Rust that allows bits to exist in superposition.

## Overview

QMem is a Rust implementation of a quantum-inspired memory structure that simulates quantum bits (qubits). Unlike classical bits that can only be 0 or 1, QMem provides an abstraction where bits can exist in a superposition state (both 0 and 1 simultaneously) until they are measured.

## Features

- 64-bit quantum-inspired memory simulation
- Bits can be in a definite state (0 or 1) or in superposition
- Measurement of superposition bits causes them to collapse to a random definite state
- Simple API for setting and measuring bits

## Usage

### Basic Example

```rust
use qmem::QMem;

fn main() {
    // Create a new QMem instance with all bits in superposition
    let mut q_array = QMem::new();
    
    // Set bit 0 to a definite value of 1
    q_array.set_bit(0, Some(true));
    
    // Set bit 1 to a definite value of 0
    q_array.set_bit(1, Some(false));
    
    // Leave bit 2 in superposition
    // (No need to explicitly call set_bit as all bits start in superposition)
    
    // Measure bits
    let bit0 = q_array.measure(0); // Will always return true (1)
    let bit1 = q_array.measure(1); // Will always return false (0)
    let bit2 = q_array.measure(2); // Will randomly return true or false
    
    println!("Bit 0: {}", bit0);
    println!("Bit 1: {}", bit1);
    println!("Bit 2: {}", bit2);
}
```

### Printing the State

QMem provides a `print` method to visualize the current state:

```rust
let mut q_array = QMem::new();
q_array.print();
// Output:
// State: 0000000000000000000000000000000000000000000000000000000000000000
// Superposition: 1111111111111111111111111111111111111111111111111111111111111111
```

## API Reference

### `QMem::new() -> Self`

Creates a new QMem instance with all 64 bits in superposition.

### `set_bit(&mut self, index: usize, value: Option<bool>)`

Sets a bit at the specified index:
- `Some(true)` - Sets the bit to 1
- `Some(false)` - Sets the bit to 0
- `None` - Places the bit in superposition

### `measure(&mut self, index: usize) -> bool`

Measures a bit at the specified index:
- If the bit is in a definite state, returns that state
- If the bit is in superposition, randomly collapses it to 0 or 1 and returns the result

### `print(&self)`

Displays the current state of the memory, showing:
- The definite states of all bits (0s and 1s)
- Which bits are in superposition

## Implementation Details

QMem uses two 64-bit integers to track the state:
- `state`: Stores the collapsed bit values (0 or 1)
- `superposition`: Bits set to 1 indicate superposition, bits set to 0 indicate a collapsed state