//! # Quantum Memory Simulator
//! 
//! A simple library for simulating quantum memory operations
//! including superposition, entanglement, and quantum gates.

use rand::Rng;
use std::collections::{HashMap, HashSet};

/// Simulates a 64-bit quantum memory register with superposition and entanglement capabilities
pub struct QMem {
    /// Stores the collapsed bits (0s and 1s)
    state: u64,
    /// 1 if bit is in superposition (both 0 and 1), 0 otherwise
    superposition: u64,
    /// Entangled pairs (index -> index)
    entangled: HashMap<usize, usize>,
    /// Entanglement group mapping
    entangled_groups: HashMap<usize, HashSet<usize>>,
}

impl QMem {
    /// Creates a new quantum memory register with all 64 bits in superposition
    pub fn new() -> Self {
        QMem {
            state: 0,
            superposition: u64::MAX, // All 64 bits in superposition
            entangled: HashMap::new(),
            entangled_groups: HashMap::new(),
        }
    }

    /// Sets a qubit to either 0, 1, or superposition
    ///
    /// * `index` - The index of the qubit (0-63)
    /// * `value` - Some(true) for 1, Some(false) for 0, None for superposition
    pub fn set_bit(&mut self, index: usize, value: Option<bool>) {
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

    /// Applies the Hadamard Gate (H) to place a qubit in superposition
    ///
    /// * `index` - The index of the qubit (0-63)
    pub fn hadamard(&mut self, index: usize) {
        self.set_bit(index, None);
    }

    /// Applies the CNOT Gate: If control is 1, flip the target bit
    ///
    /// * `control` - The control qubit index
    /// * `target` - The target qubit index
    pub fn cnot(&mut self, control: usize, target: usize) {
        if self.measure(control) {
            let target_value = !self.measure(target); // Flip target
            self.set_bit(target, Some(target_value));
        }
    }

    /// Applies the Pauli-X Gate (NOT) to flip the value of a qubit
    ///
    /// * `index` - The index of the qubit to flip
    pub fn pauli_x(&mut self, index: usize) {
        let current_value = self.measure(index);
        self.set_bit(index, Some(!current_value));
    }

    /// Applies the Swap Gate to exchange the values of two qubits
    ///
    /// * `a` - The first qubit index
    /// * `b` - The second qubit index
    pub fn swap(&mut self, a: usize, b: usize) {
        if a == b {
            return; // No need to swap
        }
        
        let temp_a = self.measure(a);
        let temp_b = self.measure(b);
        self.set_bit(a, Some(temp_b));
        self.set_bit(b, Some(temp_a));
    }

    /// Entangles two qubits so their states will always correlate
    ///
    /// * `a` - The first qubit index
    /// * `b` - The second qubit index
    pub fn entangle(&mut self, a: usize, b: usize) {
        if a != b {
            self.entangled.insert(a, b);
            self.entangled.insert(b, a);
        }
    }

    /// Entangles multiple qubits together in a group
    ///
    /// * `indices` - Array of qubit indices to entangle
    pub fn entangle_group(&mut self, indices: &[usize]) {
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
        let group_clone = group.clone();
        for &index in &group {
            self.entangled_groups.insert(index, group_clone.clone());
        }
    }

    /// Collapses all qubits in an entangled group to the same value
    ///
    /// * `index` - The index of the qubit that was measured
    /// * `value` - The value to collapse all entangled qubits to
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

    /// Measures a qubit, collapsing it if in superposition
    ///
    /// If in superposition, randomly chooses 0 or 1
    ///
    /// * `index` - The index of the qubit to measure
    /// * Returns the measured value (true for 1, false for 0)
    pub fn measure(&mut self, index: usize) -> bool {
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

    /// Runs Bell's Test to check if entangled qubits collapse together
    ///
    /// * `a` - The first qubit index
    /// * `b` - The second qubit index
    /// * Returns a tuple with the measured values and whether they are correlated
    pub fn bells_test(&mut self, a: usize, b: usize) -> (bool, bool, bool) {
        let a_measured = self.measure(a);
        let b_measured = self.measure(b);
        let correlated = a_measured == b_measured;
        
        println!("\nRunning Bell's Test on qubits {} and {}:", a, b);
        println!("Qubit {}: {}, Qubit {}: {}", a, a_measured, b, b_measured);
        if correlated {
            println!("✅ Correlated (Entanglement Verified)");
        } else {
            println!("❌ Not Correlated (Possible Decoherence)");
        }
        
        (a_measured, b_measured, correlated)
    }

    /// Print the current state of the quantum memory
    ///
    /// Displays the state, superposition, and entanglement information
    pub fn print(&self) {
        println!("State: {:064b}", self.state);
        println!("Superposition: {:064b}", self.superposition);
        println!("Entanglements: {:?}", self.entangled);
        println!("Entanglement groups: {:?}", self.entangled_groups);
    }

    /// Returns the current state value of the quantum register
    pub fn get_state(&self) -> u64 {
        self.state
    }

    /// Returns the current superposition mask of the quantum register
    pub fn get_superposition(&self) -> u64 {
        self.superposition
    }

    /// Returns a reference to the entanglement pairs
    pub fn get_entangled_pairs(&self) -> &HashMap<usize, usize> {
        &self.entangled
    }

    /// Returns a reference to the entanglement groups
    pub fn get_entangled_groups(&self) -> &HashMap<usize, HashSet<usize>> {
        &self.entangled_groups
    }
}

// Implementing Default trait for QMem
impl Default for QMem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_qmem() {
        let qmem = QMem::new();
        assert_eq!(qmem.state, 0);
        assert_eq!(qmem.superposition, u64::MAX);
        assert!(qmem.entangled.is_empty());
        assert!(qmem.entangled_groups.is_empty());
    }

    #[test]
    fn test_set_bit() {
        let mut qmem = QMem::new();
        qmem.set_bit(0, Some(true));
        assert_eq!(qmem.state & 1, 1);
        assert_eq!(qmem.superposition & 1, 0);
    }

    #[test]
    fn test_entangle_and_measure() {
        let mut qmem = QMem::new();
        qmem.entangle(0, 1);
        qmem.set_bit(0, Some(true));
        assert_eq!(qmem.measure(1), true);
    }
}