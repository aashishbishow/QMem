use std::f64::consts::FRAC_1_SQRT_2; 
use rand::rngs::OsRng;
use rand::TryRngCore;

use super::complex::Complex;

#[derive(Debug, Clone)]
pub struct StateVector {
    pub amplitudes: Vec<Complex>,
}

impl StateVector {
    // Create a new state vector for 'n' qubits, initialized to |0...0>
    pub fn new(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;
        let mut amplitudes = vec![Complex::new(0.0, 0.0); dim];
        amplitudes[0] = Complex::new(1.0, 0.0); // |0...0>
        Self { amplitudes }
    }

    // Apply Hadamard gate to a given qubit index
    pub fn apply_hadamard(&mut self, qubit: usize) {
        let dim = self.amplitudes.len();
        let mask = 1 << qubit;
        let mut new_amplitudes = self.amplitudes.clone();

        for i in 0..dim {
            // For each pair of states that differ only in the target qubit
            let j = i ^ mask; // XOR with mask to flip the target qubit
            
            // Skip processing each pair twice (process when i < j)
            if i < j {
                let (a, b) = (self.amplitudes[i], self.amplitudes[j]);

                new_amplitudes[i] = Complex::new(
                    (a.real + b.real) * FRAC_1_SQRT_2,
                    (a.imag + b.imag) * FRAC_1_SQRT_2,
                );

                new_amplitudes[j] = Complex::new(
                    (a.real - b.real) * FRAC_1_SQRT_2,
                    (a.imag - b.imag) * FRAC_1_SQRT_2,
                );
            }
        }
        
        self.amplitudes = new_amplitudes;
    }

    pub fn inner_product(&self, other: &StateVector) -> Complex {
        // Ensure dimensions match
        assert_eq!(self.amplitudes.len(), other.amplitudes.len(), 
                   "State vectors must have the same dimension");
        
        // <ψ|φ> = ∑ψᵢ*·φᵢ
        let mut result = Complex::new(0.0, 0.0);
        for i in 0..self.amplitudes.len() {
            // Multiply conjugate of self amplitude with other amplitude
            let conj_self = self.amplitudes[i].conj();
            result = result.add(&conj_self.mul(&other.amplitudes[i]));
        }
        
        result
    }
    
    // Calculate state fidelity: |<ψ|φ>|²
    pub fn fidelity(&self, other: &StateVector) -> f64 {
        let inner_prod = self.inner_product(other);
        inner_prod.magnitude_squared()
    }

    // Generate a quantum random bit using qubits
    pub fn quantum_random_bit(&mut self ) -> u8 {

        // Apply Hadamard to all qubits to create superposition
        let num_qubits = (self.amplitudes.len() as f64).log2() as usize;
        for qubit in 0..num_qubits {
            self.apply_hadamard(qubit);
        }

        // Measure the state - this collapse the wavefunction to one of the 2^n possible states
        let outcome = self.measure();

        // Use the least significant bit of the measurement outcome as our random bit
        (outcome & 1) as u8
    }

    // Generate multiple random bits using n qubits
    pub fn generate_random_bits(&mut self, count: usize, num_qubits: usize) -> Vec<u8> {
        let mut bits = Vec::with_capacity(count);

        for _ in 0..count {
            // Reset to |00000000> state before each measurement
            *self = StateVector::new(num_qubits);
            
            // Generate one random bit
            bits.push(self.quantum_random_bit());
        }
        
        bits
    }

    // Calculate the total probability (sum of magnitudes squared of all amplitudes)
    pub fn probability(&self) -> f64 {
        self.amplitudes
            .iter()
            .map(|amp| amp.magnitude_squared())
            .sum()
    }

    // Normalize the state vector so the sum of probabilities equals 1
    pub fn normalize(&mut self) {
        let total_prob = self.probability();

        // Only normalize if the probability is not zero with floating point precision
        if (total_prob - 1.0).abs() > 1e-10 { // Adjust for floating-point precision
            let normalization_factor = 1.0 / total_prob.sqrt();

            // Apply normalization to each amplitude
            for amp in &mut self.amplitudes {
                *amp = amp.scale(normalization_factor);
            }
        }
    }

    // Measure the state vector and return the index of the outcome qubit (collapses the state)
    pub fn measure(&mut self) -> usize {
        // Ensure the state is normalized before measurement
        self.normalize();

        // Generate a random value between 0 and 1
        let mut rng = OsRng;
        let rand_val =  generate_random_f64(&mut rng);
    
        // Calculate cumulative probabilities and find the outcome
        let mut cumulative = 0.0;
        for (index, amp) in self.amplitudes.iter().enumerate() {
            cumulative += amp.magnitude_squared();
            if rand_val <= cumulative {
                self.collapse(index);
                return index;
            }
        }

        fn generate_random_f64(rng: &mut OsRng) -> f64 {
            // Generate a random 64-bit unsigned integer
            let random_u64 = rng.try_next_u64().unwrap();
        
            // Convert to f64 in the range [0.0, 1.0)
            (random_u64 as f64) / (u64::MAX as f64)
        }

        // This should rarely happen due to normalization and floating-point precision but we collapse to the last state as fallback
        let last_index = self.amplitudes.len() - 1;
        self.collapse(last_index);
        last_index
    }



    // Collapse the state to the measured outcome
    pub fn collapse(&mut self, outcome: usize) {
        // Ensure outcome is within valid range
        assert!(outcome < self.amplitudes.len(), "Invalid outcome index");
        
        // Clear all amplitudes
        self.amplitudes.fill(Complex::new(0.0, 0.0));
        
        // Set the measured outcome to 1.0 (already normalized)
        self.amplitudes[outcome] = Complex::new(1.0, 0.0);
    }

    // Print the state vector in a human-readable format
    pub fn print(&self) {
        println!("StateVector:");
        for (index, amp) in self.amplitudes.iter().enumerate() {
            if amp.magnitude_squared() > 1e-10 { // Filter out near-zero amplitudes
                println!("|{:0width$b}>: {:.6} + {:.6}i", index, amp.real, amp.imag, width = (self.amplitudes.len() as f64).log2() as usize);
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_new() {
        let sv = StateVector::new(3);
        assert_eq!(sv.amplitudes.len(), 8);
        assert_eq!(sv.amplitudes[0], Complex::new(1.0, 0.0));
        for amp in &sv.amplitudes[1..] {
            assert_eq!(*amp, Complex::new(0.0, 0.0));
        }
    }

    #[test]
    fn test_apply_hadamard() {
        let mut sv = StateVector::new(1);
        sv.apply_hadamard(0);
        let expected_amp = Complex::new(FRAC_1_SQRT_2, 0.0);
        assert_eq!(sv.amplitudes[0], expected_amp);
        assert_eq!(sv.amplitudes[1], expected_amp);
    }

    #[test]
    fn test_inner_product() {
        let sv1 = StateVector::new(2);
        let sv2 = StateVector::new(2);
        let inner = sv1.inner_product(&sv2);
        assert_eq!(inner, Complex::new(1.0, 0.0));
    }

    #[test]
    fn test_fidelity() {
        let sv1 = StateVector::new(2);
        let sv2 = StateVector::new(2);
        assert!((sv1.fidelity(&sv2) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_probability() {
        let sv = StateVector::new(3);
        assert!((sv.probability() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_normalize() {
        let mut sv = StateVector::new(1);
        sv.amplitudes[0] = Complex::new(3.0, 4.0);
        sv.normalize();
        assert!((sv.probability() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_measure() {
        let mut sv = StateVector::new(3);
        let result = sv.measure();
        assert_eq!(result, 0);
        assert_eq!(sv.amplitudes[0], Complex::new(1.0, 0.0));
    }

    #[test]
    fn test_quantum_random_bit() {
        let mut sv = StateVector::new(3);
        let bit = sv.quantum_random_bit();
        assert!(bit == 0 || bit == 1);
    }

    #[test]
    fn test_generate_random_bits() {
        let mut sv = StateVector::new(3);
        let bits = sv.generate_random_bits(10, 3);
        assert_eq!(bits.len(), 10);
    }
}