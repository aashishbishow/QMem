#[allow(non_snake_case)]
use std::f64::consts::FRAC_1_SQRT_2;  // For Hadamard gate
use rand::rngs::OsRng;
use rand::{random, Rng};

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
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_new_statevector() {
        let sv = StateVector::new(3);
        assert_eq!(sv.amplitudes.len(), 8); // 2^3 = 8
        assert_eq!(sv.amplitudes[0], Complex::new(1.0, 0.0)); // |000⟩ state
    }
}
