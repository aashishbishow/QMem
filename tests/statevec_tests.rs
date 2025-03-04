#[cfg(test)]
mod integration_tests {
    use QMem::utils::statevec::StateVector;
    use QMem::utils::complex::Complex;

    #[test]
    fn test_hadamard_and_measure() {
        let mut sv = StateVector::new(1);
        sv.apply_hadamard(0);

        let outcome = sv.measure();
        assert!(outcome == 0 || outcome == 1);
    }

    #[test]
    fn test_random_bit_generation() {
        let mut sv = StateVector::new(2);
        let bits = sv.generate_random_bits(100, 2);

        // Check if random bits are either 0 or 1
        for bit in bits {
            assert!(bit == 0 || bit == 1);
        }
    }

    #[test]
    fn test_inner_product_and_fidelity() {
        let sv1 = StateVector::new(3);
        let sv2 = StateVector::new(3);
        let inner = sv1.inner_product(&sv2);

        assert_eq!(inner, Complex::new(1.0, 0.0));
        assert!((sv1.fidelity(&sv2) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_measure_and_collapse() {
        let mut sv = StateVector::new(2);
        let outcome = sv.measure();

        // Ensure measurement collapses the state
        assert_eq!(sv.amplitudes[outcome], Complex::new(1.0, 0.0));
        for (i, amp) in sv.amplitudes.iter().enumerate() {
            if i != outcome {
                assert_eq!(*amp, Complex::new(0.0, 0.0));
            }
        }
    }

    #[test]
    fn test_normalize_and_probability() {
        let mut sv = StateVector::new(3);
        sv.amplitudes[1] = Complex::new(1.0, 0.0);
        sv.normalize();

        let prob = sv.probability();
        assert!((prob - 1.0).abs() < 1e-10);
    }
} 
