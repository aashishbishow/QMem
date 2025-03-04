use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    real: f64,
    imag: f64,
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
            self.real.cosh() * self.imag.sin(),
        )
    }   

    pub fn cos(&self) -> Complex {
        Complex::new(
            self.real.cos() * self.imag.cosh(),
            -self.real.sinh() * self.imag.sin(),
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
mod tests {
    use super::*;

    #[test]
    fn test_complex_new() {
        let z = Complex::new(3.0, 4.0);
        assert_eq!(z.real, 3.0);
        assert_eq!(z.imag, 4.0);
    }
}