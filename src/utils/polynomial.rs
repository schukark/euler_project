use std::ops::{Add, Mul, MulAssign, Sub};

use super::fraction::Fraction;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Polynomial {
    /// Lowest degree first
    coefs: Vec<Fraction>,
}

impl Polynomial {
    pub fn new(coefs: &[Fraction]) -> Polynomial {
        Polynomial {
            coefs: coefs.to_vec(),
        }
    }

    pub fn eval(&self, x: Fraction) -> Fraction {
        self.coefs
            .iter()
            .rev()
            .fold(0.into(), |acc: Fraction, coef| acc * x + *coef)
    }

    pub fn fit(x: &[Fraction], y: &[Fraction]) -> Polynomial {
        let mut answer = Polynomial::new(&[]);

        assert!(x.len() == y.len());

        for i in 0..x.len() {
            let mut current = Polynomial::new(&[1.into()]);
            for j in 0..x.len() {
                if i == j {
                    continue;
                }

                let factor = (x[i] - x[j]).inverse();
                current *= Polynomial::new(&[-x[j] * factor, factor]);
            }

            answer = answer + current * Polynomial::new(&[y[i]]);
        }

        answer
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Vec::new();

        let mut i = 0;

        while i < self.coefs.len() || i < rhs.coefs.len() {
            let mut cur: Fraction = 0.into();

            if i < self.coefs.len() {
                cur += self.coefs[i];
            }
            if i < rhs.coefs.len() {
                cur += rhs.coefs[i];
            }

            i += 1;

            result.push(cur);
        }

        Polynomial::new(&result)
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Vec::new();

        let mut i = 0;

        while i < self.coefs.len() || i < rhs.coefs.len() {
            let mut cur: Fraction = 0.into();

            if i < self.coefs.len() {
                cur += self.coefs[i];
            }
            if i < rhs.coefs.len() {
                cur -= rhs.coefs[i];
            }

            i += 1;

            result.push(cur);
        }

        Polynomial::new(&result)
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut digits = Vec::new();

        for i in 0..self.coefs.len() {
            for j in 0..rhs.coefs.len() {
                let new_exp = i + j;
                let new_val = self.coefs[i] * rhs.coefs[j];

                if digits.len() <= new_exp {
                    digits.push(new_val);
                } else {
                    digits[new_exp] += new_val;
                }
            }
        }

        Polynomial::new(&digits)
    }
}

impl MulAssign for Polynomial {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::fraction::Fraction;

    use super::Polynomial;

    #[test]
    fn test_evaluation() {
        let poly = Polynomial::new(&[1.into(), 2.into(), 3.into()]);
        assert_eq!(poly.eval(2.into()), 17.into());
    }

    #[test]
    fn test_poly_addition() {
        let poly1 = Polynomial::new(&[1.into(), 2.into(), 3.into()]);
        let poly2 = Polynomial::new(&[std::convert::Into::<Fraction>::into(-1), 5.into()]);

        assert_eq!(
            poly1 + poly2,
            Polynomial::new(&[0.into(), 7.into(), 3.into()])
        );
    }

    #[test]
    fn test_poly_subtraction() {
        let poly1 = Polynomial::new(&[1.into(), 2.into(), 3.into()]);
        let poly2 = Polynomial::new(&[std::convert::Into::<Fraction>::into(-1), 5.into()]);

        assert_eq!(
            poly1 - poly2,
            Polynomial::new(&[2.into(), (-3).into(), 3.into()])
        );
    }

    #[test]
    fn test_poly_multiplication() {
        let poly1 = Polynomial::new(&[1.into(), 2.into()]);
        let poly2 = Polynomial::new(&[3.into(), (-1).into()]);

        assert_eq!(
            poly1 * poly2,
            Polynomial::new(&[3.into(), 5.into(), (-2).into()])
        );
    }

    #[test]
    fn test_poly_fit() {
        assert_eq!(
            Polynomial::fit(&[1.into(), 2.into()], &[1.into(), 8.into()]),
            Polynomial::new(&[(-6).into(), 7.into()])
        );
    }

    #[test]
    fn test_poly_fit2() {
        assert_eq!(
            Polynomial::fit(
                &[1.into(), 2.into(), 3.into()],
                &[1.into(), 8.into(), 27.into()]
            ),
            Polynomial::new(&[6.into(), (-11).into(), 6.into()])
        );
    }
}
