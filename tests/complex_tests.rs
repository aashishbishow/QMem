#[cfg(test)]
mod integration_tests {
    use QMem::utils::complex::Complex;

    #[test]
    fn test_mul_and_div() {
        let c1 = Complex::new(3.0, 4.0);
        let c2 = Complex::new(1.0, 2.0);
        let product = c1.mul(&c2);
        
        // Use approximate equality for floating-point comparisons
        let div_result = product.div(&c2);
        assert!((div_result.real - c1.real).abs() < 1e-10);
        assert!((div_result.imag - c1.imag).abs() < 1e-10);
    }

    #[test]
    fn test_exp_and_log() {
        let c = Complex::new(2.0, 3.0);
        let log_exp = c.log().exp();
        
        // Use approximate equality due to floating-point imprecision
        assert!((log_exp.real - c.real).abs() < 1e-10);
        assert!((log_exp.imag - c.imag).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_and_pow() {
        let c = Complex::new(4.0, 0.0);
        let sqrt_pow = c.sqrt().pow(&Complex::new(2.0, 0.0));
        
        // Use approximate equality
        assert!((sqrt_pow.real - c.real).abs() < 1e-10);
        assert!((sqrt_pow.imag - c.imag).abs() < 1e-10);
    }

    #[test]
    fn test_trig_identities() {
        let c = Complex::new(1.0, 1.0);
        let sin_c = c.sin();
        let cos_c = c.cos();
        
        // sin²(z) + cos²(z)
        let sin_squared = sin_c.mul(&sin_c);
        let cos_squared = cos_c.mul(&cos_c);
        let sum = sin_squared.add(&cos_squared);
        
        // Print out intermediate values for debugging
        println!("c = {:?}", c);
        println!("sin(c) = {:?}", sin_c);
        println!("cos(c) = {:?}", cos_c);
        println!("sin²(c) = {:?}", sin_squared);
        println!("cos²(c) = {:?}", cos_squared);
        println!("sin²(c) + cos²(c) = {:?}", sum);
        
        // Check that sin²(z) + cos²(z) ≈ 1
        assert!((sum.real - 1.0).abs() < 1e-6, "Real part should be close to 1");
        assert!(sum.imag.abs() < 1e-6, "Imaginary part should be close to 0");
    }

    #[test]
    fn test_basic_operations() {
        let c1 = Complex::new(3.0, 4.0);
        let c2 = Complex::new(1.0, 2.0);
        
        // Test addition
        let add_result = c1.add(&c2);
        assert_eq!(add_result.real, 4.0);
        assert_eq!(add_result.imag, 6.0);
        
        // Test subtraction
        let sub_result = c1.sub(&c2);
        assert_eq!(sub_result.real, 2.0);
        assert_eq!(sub_result.imag, 2.0);
        
        // Test conjugate
        let conj_result = c1.conj();
        assert_eq!(conj_result.real, c1.real);
        assert_eq!(conj_result.imag, -c1.imag);
    }

    #[test]
    fn test_complex_magnitude() {
        let c = Complex::new(3.0, 4.0);
        
        // Test magnitude squared
        assert!((c.magnitude_squared() - 25.0).abs() < 1e-10);
        
        // Test magnitude
        assert!((c.magnitude() - 5.0).abs() < 1e-10);
    }
}