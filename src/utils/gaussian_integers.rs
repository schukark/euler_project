use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GaussianInteger {
    real: i64,
    imag: i64,
}

const IMAG_UNIT: GaussianInteger = GaussianInteger { real: 0, imag: 1 };
const ZERO: GaussianInteger = GaussianInteger { real: 0, imag: 0 };

impl GaussianInteger {
    pub fn new() -> GaussianInteger {
        GaussianInteger { real: 0, imag: 0 }
    }

    pub fn from_parts(real: i64, imag: i64) -> GaussianInteger {
        GaussianInteger { real, imag }
    }

    pub fn norm(&self) -> i64 {
        self.real.pow(2) + self.imag.pow(2)
    }

    pub fn conjugate(&self) -> GaussianInteger {
        GaussianInteger::from_parts(self.real, -self.imag)
    }

    pub fn gcd(a: GaussianInteger, b: GaussianInteger) -> GaussianInteger {
        if b == ZERO {
            return a;
        }
        if a % b == ZERO {
            return b;
        }

        GaussianInteger::gcd(b, a % b)
    }
}

impl Add for GaussianInteger {
    type Output = GaussianInteger;

    fn add(self, rhs: Self) -> Self::Output {
        GaussianInteger::from_parts(self.real + rhs.real, self.imag + rhs.imag)
    }
}

impl AddAssign for GaussianInteger {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for GaussianInteger {
    type Output = GaussianInteger;

    fn sub(self, rhs: Self) -> Self::Output {
        GaussianInteger::from_parts(self.real - rhs.real, self.imag - rhs.imag)
    }
}

impl SubAssign for GaussianInteger {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for GaussianInteger {
    type Output = GaussianInteger;

    fn mul(self, rhs: Self) -> Self::Output {
        GaussianInteger::from_parts(
            self.real * rhs.real - self.imag * rhs.imag,
            self.imag * rhs.real + self.real * rhs.imag,
        )
    }
}

impl MulAssign for GaussianInteger {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for GaussianInteger {
    type Output = GaussianInteger;

    fn div(self, rhs: Self) -> Self::Output {
        let tmp = self * rhs.conjugate();
        let denom = rhs.norm();
        GaussianInteger::from_parts(tmp.real / denom, tmp.imag / denom)
    }
}

impl DivAssign for GaussianInteger {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Rem for GaussianInteger {
    type Output = GaussianInteger;

    fn rem(self, rhs: Self) -> Self::Output {
        self - (self / rhs) * rhs
    }
}

impl RemAssign for GaussianInteger {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs
    }
}

impl Default for GaussianInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::gaussian_integers::GaussianInteger;

    #[test]
    fn test_gcd1() {
        assert_eq!(
            GaussianInteger::from_parts(2, 1),
            GaussianInteger::gcd(
                GaussianInteger::from_parts(5, 0),
                GaussianInteger::from_parts(2, 1)
            )
        );
    }

    #[test]
    fn test_gcd2() {
        assert_eq!(
            GaussianInteger::from_parts(4, -1),
            GaussianInteger::gcd(
                GaussianInteger::from_parts(17, 0),
                GaussianInteger::from_parts(13, 1)
            )
        );
    }
}
