#[allow(non_snake_case)]
use core::num;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::f64::consts::FRAC_1_SQRT_2;  // For Hadamard gate
use rand::rngs::OsRng;
use rand::{random, Rng, TryRngCore};
use rand::RngCore;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    fn magnitude_squared(&self) ->f64 {
        self.real * self.real + self.imag * self.imag
    }

    fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    fn conj(&self) -> Complex {
        Complex::new(self.real, -self.imag)
    }

    fn mul(&self, other: &Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }

    fn scale(&self, factor: f64) -> Complex {
        Complex::new(self.real * factor, self.imag * factor)
    }

    fn add(&self, other: &Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }

    fn sub(self, other: Self) -> Complex {
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
    


}



// Implement Display for pretty printing
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{:.6} + {:.6}i", self.real, self.imag)
        } else {
            write!(f, "{:.6} - {:.6}i", self.real, -self.imag)
        }
    }
}

#[derive(Debug, Clone)]
struct StateVector {
    amplitudes: Vec<Complex>,
}

impl StateVector {
    // Create a new state vector for 'n' qubits, initialized to |0...0>
    fn new(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;
        let mut amplitudes = vec![Complex::new(0.0, 0.0); dim];
        amplitudes[0] = Complex::new(1.0, 0.0); // |0...0>
        Self { amplitudes }
    }

    // Apply Hadamard gate to a given qubit index
    fn apply_hadamard(&mut self, qubit: usize) {
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

    fn inner_product(&self, other: &StateVector) -> Complex {
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
    fn fidelity(&self, other: &StateVector) -> f64 {
        let inner_prod = self.inner_product(other);
        inner_prod.magnitude_squared()
    }

    // Generate a quantum random bit using qubits
    fn quantum_random_bit(&mut self ) -> u8 {

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
    fn generate_random_bits(&mut self, count: usize, num_qubits: usize) -> Vec<u8> {
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
    fn probability(&self) -> f64 {
        self.amplitudes
            .iter()
            .map(|amp| amp.magnitude_squared())
            .sum()
    }

    // Normalize the state vector so the sum of probabilities equals 1
    fn normalize(&mut self) {
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
    fn measure(&mut self) -> usize {
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
    fn collapse(&mut self, outcome: usize) {
        // Ensure outcome is within valid range
        assert!(outcome < self.amplitudes.len(), "Invalid outcome index");
        
        // Clear all amplitudes
        self.amplitudes.fill(Complex::new(0.0, 0.0));
        
        // Set the measured outcome to 1.0 (already normalized)
        self.amplitudes[outcome] = Complex::new(1.0, 0.0);
    }

    // Print the state vector in a human-readable format
    fn print(&self) {
        println!("StateVector:");
        for (index, amp) in self.amplitudes.iter().enumerate() {
            if amp.magnitude_squared() > 1e-10 { // Filter out near-zero amplitudes
                println!("|{:0width$b}>: {:.6} + {:.6}i", index, amp.real, amp.imag, width = (self.amplitudes.len() as f64).log2() as usize);
            }
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_1_SQRT_2;  // For Hadamard gate
    use rand::rngs::OsRng;
    use rand::{random, Rng};

    #[test]
    fn test_complex() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        assert_eq!(a.magnitude_squared(), 5.0);
        assert_eq!(a.conj(), Complex::new(1.0, -2.0));
        assert_eq!(a.mul(&b), Complex::new(-5.0, 10.0));
        assert_eq!(a.scale(2.0), Complex::new(2.0, 4.0));
        assert_eq!(a.add(&b), Complex::new(4.0, 6.0));
    }

    #[test]
    fn test_complex_magnitude_squared() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.magnitude_squared(), 9.0 + 16.0);
    }

    #[test]
    fn test_complex_conj() {
        let c = Complex::new(1.0, 2.0);
        let conj_c = c.conj();
        assert_eq!(conj_c.real, 1.0);
        assert_eq!(conj_c.imag, -2.0);
    }

    #[test]
    fn test_complex_mul() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        let result = c1.mul(&c2);
        assert_eq!(result.real, -5.0);
        assert_eq!(result.imag, 10.0);
    }

    #[test]
    fn test_complex_scale() {
        let c = Complex::new(2.0, 3.0);
        let scaled = c.scale(2.0);
        assert_eq!(scaled.real, 4.0);
        assert_eq!(scaled.imag, 6.0);
    }

    #[test]
    fn test_complex_add() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        let result = c1.add(&c2);
        assert_eq!(result.real, 4.0);
        assert_eq!(result.imag, 6.0);
    }

    #[test]
    fn test_new_statevector() {
        let sv = StateVector::new(3);
        assert_eq!(sv.amplitudes.len(), 8); // 2^3 = 8
        assert_eq!(sv.amplitudes[0], Complex::new(1.0, 0.0)); // |000⟩ state
    }

    #[test]
    fn test_apply_hadamard() {
        let mut state = StateVector::new(1);
        state.apply_hadamard(0);
        
        // After applying Hadamard to |0>, the state should be (|0> + |1>)/sqrt(2)
        let expected = vec![
            Complex::new(FRAC_1_SQRT_2, 0.0),
            Complex::new(FRAC_1_SQRT_2, 0.0),
        ];
        
        for (amp, expected_amp) in state.amplitudes.iter().zip(expected.iter()) {
            assert!((amp.real - expected_amp.real).abs() < 1e-10);
            assert!((amp.imag - expected_amp.imag).abs() < 1e-10);
        }
    }

    #[test]
    fn test_apply_hadamard_on_first_qubit() {
        let mut sv = StateVector::new(1);

        // Apply Hadamard gate to the first qubit (index 0)
        sv.apply_hadamard(0);

        // Expected amplitudes after Hadamard on |0> should be (|0> + |1>) / sqrt(2)
        let expected_amp = Complex::new(FRAC_1_SQRT_2, 0.0);

        assert!((sv.amplitudes[0].real - expected_amp.real).abs() < 1e-10);
        assert!((sv.amplitudes[1].real - expected_amp.real).abs() < 1e-10);
    }

    #[test]
    fn test_apply_hadamard_on_second_qubit() {
        let mut sv = StateVector::new(2);

        // Apply Hadamard gate to the second qubit (index 1)
        sv.apply_hadamard(1);

        // Expected amplitudes: (|00> + |10>) / sqrt(2)
        assert!((sv.amplitudes[0].real - FRAC_1_SQRT_2).abs() < 1e-10);
        assert!((sv.amplitudes[2].real - FRAC_1_SQRT_2).abs() < 1e-10);
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
        let bits = sv.generate_random_bits(5, 3);
        assert_eq!(bits.len(), 5);
        for &bit in &bits {
            assert!(bit == 0 || bit == 1);
        }
    }

    #[test]
    fn test_probability() {
        let mut sv = StateVector::new(3);
        assert_eq!(sv.probability(), 1.0);
        sv.amplitudes[0] = Complex::new(0.5, 0.5);
        assert_eq!(sv.probability(), 0.5);
    }

    #[test]
    fn test_normalize() {
        let mut state = StateVector::new(2);
        state.amplitudes = vec![
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ];
        
        state.normalize();
        let norm = (3.0f64.powi(2) + 4.0f64.powi(2)).sqrt();
        let expected = vec![
            Complex::new(3.0 / norm, 0.0),
            Complex::new(4.0 / norm, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ];
        
        for (amp, expected_amp) in state.amplitudes.iter().zip(expected.iter()) {
            assert!((amp.real - expected_amp.real).abs() < 1e-10);
            assert!((amp.imag - expected_amp.imag).abs() < 1e-10);
        }
    }

    #[test]
    fn test_normalize_01() {
        let mut sv = StateVector::new(1);

        // Manually set amplitudes (not normalized)
        sv.amplitudes[0] = Complex::new(3.0, 4.0); // Magnitude = 5
        sv.amplitudes[1] = Complex::new(0.0, 0.0);

        // Normalize the state
        sv.normalize();

        let mag_sq = sv.probability();

        // The state should be normalized, so the sum of magnitudes squared should be 1
        assert!((mag_sq - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_apply_hadamard_and_normalize() {
        let mut sv = StateVector::new(1);

        // Apply Hadamard gate and then normalize
        sv.apply_hadamard(0);
        sv.normalize();

        let mag_sq = sv.probability();

        // The state should still be normalized
        assert!((mag_sq - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_measure() {
        let mut sv = StateVector::new(3);
        let outcome = sv.measure();
        assert!(outcome < 8);
    }

    #[test]
    fn test_collapse() {
        let mut sv = StateVector::new(3);
        sv.collapse(3);
        assert_eq!(sv.amplitudes[3], Complex::new(1.0, 0.0));
        assert_eq!(sv.probability(), 1.0);
    }

    #[test]
    fn test_print() {
        let mut sv = StateVector::new(3);
        sv.amplitudes[0] = Complex::new(0.5, 0.5);
        sv.amplitudes[7] = Complex::new(0.5, -0.5);
        sv.print();
    }
}
