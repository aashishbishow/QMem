use QMem::complex::Complex;

#[test]
fn test_complex_new() {
    let z = Complex::new(3.0, 4.0);
    assert_eq!(z.real, 3.0);
    assert_eq!(z.imag, 4.0);
}