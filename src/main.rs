use rand::Rng;
use std::collections::{HashMap, HashSet};

struct QMem {
    state: u64,      // Stores the collapsed bits (0s and 1s)
    superposition: u64, // 1 if bit is in superposition (both 0 and 1), 0 otherwise
    entangled: HashMap<usize, usize>, // Entangled pairs (index -> index)
    entangled_groups: HashMap<usize, HashSet<usize>>, // Entanglement map
}

impl QMem {
    // Initialize: All bits in superposition (both 0 and 1)
    fn new() -> Self {
        QMem {
            state: 0,
            superposition: u64::MAX, // All 64 bits in superposition
            entangled: HashMap::new(),
            entangled_groups: HashMap::new(),
        }
    }

    // Set a bit to either 0, 1, or superposition
    fn set_bit(&mut self, index: usize, value: Option<bool>) {
        if index >= 64 {
            return; // Prevent out-of-bounds access
        }

        if let Some(v) = value {
            // Collapse to a fixed value (0 or 1)
            self.superposition &= !(1 << index); // Clear the superposition bit
            if v {
                self.state |= 1 << index; // Set to 1
            } else {
                self.state &= !(1 << index); // Set to 0
            }

            // Collapse entangled groups if they exist
            self.collapse_entangled_group(index, v);
        } else {
            // Set to superposition (both 0 and 1)
            self.superposition |= 1 << index;
        }
    }

    // Hadamard Gate (H): Places a bit in superposition
    fn hadamard(&mut self, index: usize) {
        self.set_bit(index, None);
    }

    // CNOT Gate: If control is 1, flip the target bit
    fn cnot(&mut self, control: usize, target: usize) {
        if self.measure(control) {
            let target_value = !self.measure(target); // Flip target
            self.set_bit(target, Some(target_value));
        }
    }

    // Pauli-X Gate (NOT): Flips the value of a qubit
    fn pauli_x(&mut self, index: usize) {
        let current_value = self.measure(index);
        self.set_bit(index, Some(!current_value));
    }

    // Swap Gate: Exchanges the values of two qubits
    fn swap(&mut self, a: usize, b: usize) {
        if a == b {
            return; // No need to swap
        }
        
        let temp_a = self.measure(a);
        let temp_b = self.measure(b);
        self.set_bit(a, Some(temp_b));
        self.set_bit(b, Some(temp_a));
    }

    // Entangle two bits – their states will always correlate
    fn entangle(&mut self, a: usize, b: usize) {
        if a != b {
            self.entangled.insert(a, b);
            self.entangled.insert(b, a);
        }
    }

    // Multi-group entanglement: Entangle multiple qubits together
    fn entangle_group(&mut self, indices: &[usize]) {
        if indices.len() < 2 {
            return; // Need at least 2 qubits to entangle
        }

        let mut group = HashSet::new();

        // First collect all existing entanglements
        for &index in indices {
            group.insert(index);

            // Merge existing groups if any qubit is already entangled
            if let Some(existing) = self.entangled_groups.get(&index) {
                for &member in existing {
                    group.insert(member);
                }
            }
        }
        
        // Update entanglement for all involved qubits
        // Now update the entanglement for all qubits in the group
        let group_clone = group.clone();
        for &index in &group {
            self.entangled_groups.insert(index, group_clone.clone());
        }
    }

    // Collapse entangled group to the same value
    fn collapse_entangled_group(&mut self, index: usize, value: bool) {
        if let Some(group) = self.entangled_groups.get(&index).cloned() {
            for &entangled_bit in &group {
                if entangled_bit != index && (self.superposition >> entangled_bit) & 1 == 1 {
                    // Avoid recursion by directly setting
                    self.superposition &= !(1 << entangled_bit);
                    if value {
                        self.state |= 1 << entangled_bit;
                    } else {
                        self.state &= !(1 << entangled_bit);
                    }
                }
            }
        }
        // Handle paired entanglement separately from group entanglement
        if let Some(&pair) = self.entangled.get(&index) {
            if (self.superposition >> pair) & 1 == 1 {
                self.superposition &= !(1 << pair);
                if value {
                    self.state |= 1 << pair;
                } else {
                    self.state &= !(1 << pair);
                }
            }
        }
    }

    // Measure (collapse) a bit – if in superposition, randomly choose 0 or 1
    fn measure(&mut self, index: usize) -> bool {
        if index >= 64 {
            return false; // Prevent out-of-bounds access
        }
        
        if (self.superposition >> index) & 1 == 1 {
            // Randomly collapse to 0 or 1 if in superposition
            let mut rng = rand::rng();
            let value = rng.random_bool(0.5);
            self.set_bit(index, Some(value));
            value
        } else {
            // Already collapsed, return the value
            (self.state >> index) & 1 == 1
        }
    }

    // Run Bell's Test: Check if entangled qubits collapse together
    fn bells_test(&mut self, a: usize, b: usize) {
        println!("\nRunning Bell’s Test on qubits {} and {}:", a, b);
        let a_measured = self.measure(a);
        let b_measured = self.measure(b);
        println!("Qubit {}: {}, Qubit {}: {}", a, a_measured, b, b_measured);
        if a_measured == b_measured {
            println!("✅ Correlated (Entanglement Verified)");
        } else {
            println!("❌ Not Correlated (Possible Decoherence)");
        }
    }

    // Print the current state of the array
    fn print(&self) {
        println!("State: {:064b}", self.state);
        println!("Superposition: {:064b}", self.superposition);
        println!("Entanglements: {:?}", self.entangled);
        println!("Entanglement groups: {:?}", self.entangled_groups);
    }
}

fn main() {
    let mut q_array = QMem::new();

    println!("Initial state:");
    q_array.print();

    // Entangle bit pairs (e.g., 0 <-> 63, 1 <-> 62)
    q_array.entangle(0, 63);
    q_array.entangle(1, 62);
    q_array.entangle(2, 61);

    println!("\nAfter Entangling Bits:");
    q_array.print();

    // Create a group entanglement between qubits 3, 4, 5
    println!("\nCreating group entanglement between qubits 3, 4, 5:");
    q_array.entangle_group(&[3, 4, 5]);
    q_array.print();

    // Apply Hadamard to qubit 0 (superposition)
    println!("\nApplying Hadamard to qubit 0:");
    q_array.hadamard(0);
    q_array.print();

    // Apply Pauli-X (NOT Gate) to qubit 0
    println!("\nApplying Pauli-X (NOT) to qubit 0:");
    q_array.pauli_x(0);
    q_array.print();

    // Swap qubit 0 and qubit 1
    println!("\nSwapping qubit 0 and qubit 1:");
    q_array.swap(0, 1);
    q_array.print();


    // Entangle qubit 0 and 1 using Hadamard + CNOT
    println!("\nEntangling qubit 0 and qubit 1:");
    q_array.hadamard(0); // Put qubit 0 in superposition
    q_array.cnot(0, 1);  // Entangle qubit 0 and 1
    q_array.entangle(0, 1);
    q_array.print();

    // Create a larger group entanglement
    println!("\nCreating group entanglement between qubits 10, 11, 12, 13:");
    q_array.entangle_group(&[10, 11, 12, 13]);
    q_array.print();

    // Measure qubits 0 and 1
    println!("\nMeasuring qubits 0 and 1 (entangled state):");
    let q0 = q_array.measure(0);
    let q1 = q_array.measure(1);
    println!("Qubit 0: {}, Qubit 1: {}", q0, q1);

    // Run Bell’s Test
    q_array.bells_test(0, 1);
    println!("Bells Tests");
    q_array.print();

    // Collapse and print each bit
    println!("\nMeasuring all bits:");
    for i in 0..64 {
        let result = q_array.measure(i);
        println!("Index {} collapsed to: {}, Entangled pair collapsed to: {}", i, result as u8, q_array.measure(63 - i) as u8);
    }

    println!("\nFinal state after measurement:");
    q_array.print();
}
