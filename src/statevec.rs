#[allow(non_snake_case)]
use ndarray::Array;
use num_complex::Complex64;

fn main() {
    // Define a qubit in the |0> state
    let zero_state = Array<Complex64> = Array::from(vec![
        Complex64::new(1.0, 0.0), // a =1
        Complex64::new(0..0, 0.0), // b =0
    ]);

    //Define a qubit in the |+> state (superposition)
    let plus_state: Array<Complex64> = Array::from(vec![
        Complex64::new(1.0 / 2.0_f64.sqrt(), 0.0),
        Complex64::new(1.0 / 2.0_f64.sqrt(), 00),
    ]);

    println!("Zero state: {:?}", zero_state);
    println!("Plus_state: {:?}", plus_state);
}