#![allow(non_snake_case)]
//! # Quantum Memory Simulator
//! 
//! A simple library for simulating quantum memory operations
//! including superposition, entanglement, and quantum gates.

use rand::Rng;
// use std::collections::{HashMap, HashSet};

/// Simulates a 64-bit quantum memory register with superposition and entanglement capabilities
pub struct QMem {
    /// Stores the collapsed bits (0s and 1s)
    /// Classical bit values
    state: u64,
    /// 1 if bit is in superposition (both 0 and 1), 0 otherwise
    superposition: u64,
    // Entangled pairs (index -> index)
    // entangled: HashMap<usize, usize>,
    // Entanglement group mapping
    // entangled_groups: HashMap<usize, HashSet<usize>>,
    probability: [f64; 64],
    //Probability of measuring 1 for qubits in superpositions

}

impl QMem {
    /// Creates a new quantum memory register with all 64 bits in superposition
    pub fn new() -> Self {
        QMem {
            state: 0,
            superposition: 0,
            // entangled: HashMap::new(),
            // entangled_groups: HashMap::new(),
            probability: [0.5; 64],
            // Default to equal probability (50% |0) + 50% |1)
        }
    }

    /// Sets a qubit to either 0, 1, or superposition
    ///
    /// * `index` - The index of the qubit (0-63)
    /// * `value` - Some(true) for 1, Some(false) for 0, None for superposition
    pub fn set_qubit(&mut self, index: usize, value: Option<bool>, prob:Option<f64> ) {
        if index >= 64 {
            panic!("Out of range");
        }

        match value {
            Some(bit) => {
                // Set classical bit
                if bit {
                    self.state |= 1<< index; // Set bit to 1
                } else {
                    self.state &= !(1 << index); // Set bit to 0
                }
                // Ensure no superposition
                self.superposition &= !(1 << index);
                
            }
            None => {
                // Enter superposition
                self.superposition |= 1 << index;
                self.probability[index] = prob.unwrap_or(0.5); // Set probability (default 50/50)
            }
        }

    }


    /// Applies the Hadamard Gate (H) to place a qubit in superposition
    ///
    /// * `index` - The index of the qubit (0-63)
    // pub fn hadamard(&mut self, index: usize) {
    //     self.set_qubit(index, None);
    // }

    /// Applies the CNOT Gate: If control is 1, flip the target bit
    ///
    /// * `control` - The control qubit index
    /// * `target` - The target qubit index
    // pub fn cnot(&mut self, control: usize, target: usize) {
    //     if self.measure(control) {
    //         let target_value = !self.measure(target); // Flip target
    //         self.set_qubit(target, Some(target_value));
    //     }
    // }

    /// Applies the Pauli-X Gate (NOT) to flip the value of a qubit
    ///
    /// * `index` - The index of the qubit to flip
    // pub fn pauli_x(&mut self, index: usize) {
    //     let current_value = self.measure(index);
    //     self.set_qubit(index, Some(!current_value));
    // }

    /// Applies the Swap Gate to exchange the values of two qubits
    ///
    /// * `a` - The first qubit index
    /// * `b` - The second qubit index
    // pub fn swap(&mut self, a: usize, b: usize) {
    //     if a == b {
    //         return; // No need to swap
    //     }
        
    //     let temp_a = self.measure(a);
    //     let temp_b = self.measure(b);
    //     self.set_qubit(a, Some(temp_b));
    //     self.set_qubit(b, Some(temp_a));
    // }

    /// Entangles two qubits so their states will always correlate
    ///
    /// * `a` - The first qubit index
    /// * `b` - The second qubit index
    // pub fn entangle(&mut self, a: usize, b: usize) {
    //     if a != b {
    //         self.entangled.insert(a, b);
    //         self.entangled.insert(b, a);
    //     }
    // }

    /// Entangles multiple qubits together in a group
    ///
    /// * `indices` - Array of qubit indices to entangle
    // pub fn entangle_group(&mut self, indices: &[usize]) {
    //     if indices.len() < 2 {
    //         return; // Need at least 2 qubits to entangle
    //     }

    //     let mut group = HashSet::new();

    //     // First collect all existing entanglements
    //     for &index in indices {
    //         group.insert(index);

    //         // Merge existing groups if any qubit is already entangled
    //         if let Some(existing) = self.entangled_groups.get(&index) {
    //             for &member in existing {
    //                 group.insert(member);
    //             }
    //         }
    //     }
        
    //     // Update entanglement for all involved qubits
    //     let group_clone = group.clone();
    //     for &index in &group {
    //         self.entangled_groups.insert(index, group_clone.clone());
    //     }
    // }

    /// Collapses all qubits in an entangled group to the same value
    ///
    /// * `index` - The index of the qubit that was measured
    /// * `value` - The value to collapse all entangled qubits to
    // fn collapse_entangled_group(&mut self, index: usize, value: bool) {
    //     if let Some(group) = self.entangled_groups.get(&index).cloned() {
    //         for &entangled_bit in &group {
    //             if entangled_bit != index && (self.superposition >> entangled_bit) & 1 == 1 {
    //                 // Avoid recursion by directly setting
    //                 self.superposition &= !(1 << entangled_bit);
    //                 if value {
    //                     self.state |= 1 << entangled_bit;
    //                 } else {
    //                     self.state &= !(1 << entangled_bit);
    //                 }
    //             }
    //         }
    //     }
    //     // Handle paired entanglement separately from group entanglement
    //     if let Some(&pair) = self.entangled.get(&index) {
    //         if (self.superposition >> pair) & 1 == 1 {
    //             self.superposition &= !(1 << pair);
    //             if value {
    //                 self.state |= 1 << pair;
    //             } else {
    //                 self.state &= !(1 << pair);
    //             }
    //         }
    //     }
    // }

    /// Measures a qubit, collapsing it if in superposition
    ///
    /// If in superposition, randomly chooses 0 or 1
    ///
    /// * `index` - The index of the qubit to measure
    /// * Returns the measured value (true for 1, false for 0)
    pub fn measure(&mut self, index: usize) -> bool {
        if index >= 64 {
            panic!("Index out of bound"); // Prevent out-of-bounds access
        }
        
        // If qubit is in superposition, collapse based on probability
        if (self.superposition & (1 << index)) != 0{
            let outcome = rand::rng().random_bool(self.probability[index]);
            self.set_qubit(index, Some(outcome), None);
            outcome
            } else {
                // If not in superposition, return its classical value
                (self.state & (1 << index)) != 0
            }
        }
    

    /// Runs Bell's Test to check if entangled qubits collapse together
    ///
    /// * `a` - The first qubit index
    /// * `b` - The second qubit index
    /// * Returns a tuple with the measured values and whether they are correlated
    // pub fn bells_test(&mut self, a: usize, b: usize) -> (bool, bool, bool) {
    //     let a_measured = self.measure(a);
    //     let b_measured = self.measure(b);
    //     let correlated = a_measured == b_measured;
        
    //     println!("\nRunning Bell's Test on qubits {} and {}:", a, b);
    //     println!("Qubit {}: {}, Qubit {}: {}", a, a_measured, b, b_measured);
    //     if correlated {
    //         println!("✅ Correlated (Entanglement Verified)");
    //     } else {
    //         println!("❌ Not Correlated (Possible Decoherence)");
    //     }
        
    //     (a_measured, b_measured, correlated)
    // }

    /// Print the current state of the quantum memory
    ///
    /// Displays the state, superposition, and entanglement information
    pub fn print(&self) {
        println!("State: {:064b}", self.state);
        println!("Superposition: {:064b}", self.superposition);
        println!("Probability: {:?}", &self.probability[..8]); // Display first 8 for brevity
        // println!("Entanglements: {:?}", self.entangled);
        // println!("Entanglement groups: {:?}", self.entangled_groups);
    }

    /// Returns the current state value of the quantum register
    pub fn get_state(&self) -> u64 {
        self.state
    }

    /// Returns the current superposition mask of the quantum register
    pub fn get_superposition(&self) -> u64 {
        self.superposition
    }

    // /// Returns a reference to the entanglement pairs
    // pub fn get_entangled_pairs(&self) -> &HashMap<usize, usize> {
    //     &self.entangled
    // }

    // /// Returns a reference to the entanglement groups
    // pub fn get_entangled_groups(&self) -> &HashMap<usize, HashSet<usize>> {
    //     &self.entangled_groups
    // }
}


// Implementing Default trait for QMem
impl Default for QMem {
    fn default() -> Self {
        Self::new()
    }
}

fn main(){
    let  qmem = QMem::new();
    qmem.print();
}