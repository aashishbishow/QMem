use QMem::utils::complex::Complex;


#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_mul_and_div() {
        let c1 = Complex::new(3.0, 4.0);
        let c2 = Complex::new(1.0, 2.0);
        let product = c1.mul(&c2);
        assert_eq!(product.div(&c2), c1);
    }

    #[test]
    fn test_exp_and_log() {
        let c = Complex::new(2.0, 3.0);
        assert_eq!(c.log().exp(), c);
    }

    #[test]
    fn test_sqrt_and_pow() {
        let c = Complex::new(4.0, 0.0);
        assert_eq!(c.sqrt().pow(&Complex::new(2.0, 0.0)), c);
    }

    #[test]
    fn test_trig_identities() {
        let c = Complex::new(1.0, 1.0);
        let sin_squared = c.sin().mul(&c.sin());
        let cos_squared = c.cos().mul(&c.cos());
        let sum = sin_squared.add(&cos_squared);
        assert!((sum.real - 1.0).abs() < 1e-6);
        assert!((sum.imag).abs() < 1e-6);
    }
}
