use rand::Rng;

struct QMem {
    state: u64,      // Stores the collapsed bits (0s and 1s)
    superposition: u64, // 1 if bit is in superposition (both 0 and 1), 0 otherwise
}

impl QMem {
    // Initialize: All bits in superposition (both 0 and 1)
    fn new() -> Self {
        QMem {
            state: 0,
            superposition: u64::MAX, // All 64 bits in superposition
        }
    }

    // Set a bit to either 0, 1, or superposition
    fn set_bit(&mut self, index: usize, value: Option<bool>) {
        if let Some(v) = value {
            // Collapse to a fixed value (0 or 1)
            self.superposition &= !(1 << index); // Clear the superposition bit
            if v {
                self.state |= 1 << index; // Set to 1
            } else {
                self.state &= !(1 << index); // Set to 0
            }
        } else {
            // Set to superposition (both 0 and 1)
            self.superposition |= 1 << index;
        }
    }

    // Measure (collapse) a bit – if in superposition, randomly choose 0 or 1
    fn measure(&mut self, index: usize) -> bool {
        if (self.superposition >> index) & 1 == 1 {
            // Randomly collapse to 0 or 1 if in superposition
            let value = rand::rng().random_bool(0.5);
            self.set_bit(index, Some(value));
            value
        } else {
            // Already collapsed, return the value
            (self.state >> index) & 1 == 1
        }
    }

    // Print the current state of the array
    fn print(&self) {
        println!("State: {:064b}", self.state);
        println!("Superposition: {:064b}", self.superposition);
    }
}

fn main() {
    let mut q_array = QMem::new();

    println!("Initial state:");
    q_array.print();

    // Collapse and print each bit
    println!("\nMeasuring all bits:");
    for i in 0..64 {
        let result = q_array.measure(i);
        println!("Index {} collapsed to: {}", i, result as u8);
    }

    println!("\nFinal state after measurement:");
    q_array.print();
}
