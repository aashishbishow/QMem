use super::complex::Complex;

#[derive(Debug, Clone)]
pub struct DensityMatrix {
    pub matrix: Vec<Vec<Complex>>,
    pub size: usize,
}

impl DensityMatrix {
    // Create a new density matrix for 'n' qubits, initialized to |0...0><0...0|
    pub fn new(num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut matrix = vec![vec![Complex::new(0.0, 0.0); size]; size];
        matrix[0][0] = Complex::new(1.0, 0.0); // |0...0><0...0| state
        Self { matrix, size }
    }
}