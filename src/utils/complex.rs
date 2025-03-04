use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn conj(&self) -> Complex {
        Complex::new(
            self.real, -self.imag
        )
    }

    pub fn mul(&self, other: &Complex) -> Complex {
        Complex::new(
        self.real * other.real - self.imag * other.imag,
        self.real * other.imag + self.imag * other.real,
        )
    }

    pub fn scale(&self, factor:f64) -> Complex {
        Complex::new(self.real * factor, self.imag * factor)
    }

    pub fn add(&self, other: &Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }

    pub fn sub(&self, other: &Complex) -> Complex {
        Complex::new(self.real - other.real, self.imag - other.imag)
    }

    pub fn div(&self, other: &Complex) -> Complex {
        let denominator = other.magnitude_squared();
        Complex::new(
            (self.real * other.real + self.imag * other.imag) / denominator,
            (self.imag * other.real - self.real * other.imag) / denominator,
        )
    }

    pub fn exp(&self) -> Complex {
        let exp_real = self.real.exp();
        Complex::new(exp_real * self.imag.cos(), exp_real * self.imag.sin())
    }

    pub fn log(&self) -> Complex {
        Complex::new(self.magnitude().ln(), self.arg())
    }

    pub fn arg(&self) -> f64 {
        self.imag.atan2(self.real)
    }

    pub fn pow(&self, other: &Complex) -> Complex {
        self.log().mul(&other).exp()
    }

    pub fn sin(&self) -> Complex {
        Complex::new(
            self.real.sin() * self.imag.cosh(),
            self.real.cos() * self.imag.sinh()
        )
    }   

    pub fn cos(&self) -> Complex {
        Complex::new(
            self.real.cos() * self.imag.cosh(),
            -self.real.sin() * self.imag.sinh()
        )
    }   

    pub fn tan(&self) -> Complex {
        self.sin().div(&self.cos())
    }

    pub fn sqrt(&self) -> Complex {
        let r = self.magnitude();
        let theta = self.arg();
        Complex::new(
            r.sqrt() * theta.cos(),
            r.sqrt() * theta.sin(),
        )
    }
}

// Implement Display for Complex
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{:.6} + {:.6}i", self.real, self.imag)
        } else {
            write!(f, "{:.6} - {:.6}i", self.real, -self.imag)
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.real, 3.0);
        assert_eq!(c.imag, 4.0);
    }

    #[test]
    fn test_magnitude_squared() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.magnitude_squared(), 25.0);
    }

    #[test]
    fn test_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.magnitude(), 5.0);
    }

    #[test]
    fn test_conjugate() {
        let c = Complex::new(3.0, 4.0);
        let conj = c.conj();
        assert_eq!(conj, Complex::new(3.0, -4.0));
    }

    #[test]
    fn test_add() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        assert_eq!(c1.add(&c2), Complex::new(4.0, 6.0));
    }

    #[test]
    fn test_sub() {
        let c1 = Complex::new(5.0, 6.0);
        let c2 = Complex::new(3.0, 4.0);
        assert_eq!(c1.sub(&c2), Complex::new(2.0, 2.0));
    }

    #[test]
    fn test_mul() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        assert_eq!(c1.mul(&c2), Complex::new(-5.0, 10.0));
    }

    #[test]
    fn test_div() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        assert_eq!(c1.div(&c2), Complex::new(0.44, 0.08));
    }

    #[test]
    fn test_scale() {
        let c = Complex::new(1.0, -1.0);
        assert_eq!(c.scale(2.0), Complex::new(2.0, -2.0));
    }

    #[test]
    fn test_exp() {
        let c = Complex::new(0.0, std::f64::consts::PI);
        assert_eq!(c.exp(), Complex::new(-1.0, 0.0));
    }

    #[test]
    fn test_log() {
        let c = Complex::new(1.0, 0.0);
        assert_eq!(c.log(), Complex::new(0.0, 0.0));
    }

    #[test]
    fn test_sqrt() {
        let c = Complex::new(4.0, 0.0);
        assert_eq!(c.sqrt(), Complex::new(2.0, 0.0));
    }

    #[test]
    fn test_pow() {
        let c = Complex::new(2.0, 0.0);
        let exp = Complex::new(2.0, 0.0);
        assert_eq!(c.pow(&exp), Complex::new(4.0, 0.0));
    }

    #[test]
    fn test_sin() {
        let c = Complex::new(0.0, 0.0);
        assert_eq!(c.sin(), Complex::new(0.0, 0.0));
    }

    #[test]
    fn test_cos() {
        let c = Complex::new(0.0, 0.0);
        assert_eq!(c.cos(), Complex::new(1.0, 0.0));
    }

    #[test]
    fn test_tan() {
        let c = Complex::new(0.0, 0.0);
        assert_eq!(c.tan(), Complex::new(0.0, 0.0));
    }

}
