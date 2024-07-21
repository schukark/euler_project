use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

#[derive(Copy, Clone, Debug)]
pub struct Fraction {
    pub numerator: i128,
    pub denominator: i128,
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.numerator * other.denominator == self.denominator * other.numerator
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.numerator * other.denominator).partial_cmp(&(self.denominator * other.numerator))
    }
}

impl Eq for Fraction {}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Fraction {
    pub fn new(numerator: i128, denominator: i128) -> Fraction {
        let mut new_numerator = numerator
            / gcd::binary_u128(i128::abs(numerator) as u128, i128::abs(denominator) as u128)
                as i128;
        let mut new_denominator = denominator
            / gcd::binary_u128(i128::abs(numerator) as u128, i128::abs(denominator) as u128)
                as i128;

        if denominator < 0 {
            new_numerator *= -1;
            new_denominator *= -1;
        }

        Fraction {
            numerator: new_numerator,
            denominator: new_denominator,
        }
    }

    pub fn inverse(&self) -> Fraction {
        assert!(self.numerator != 0);

        Fraction {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.numerator * rhs.denominator + self.denominator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.numerator * rhs.denominator - self.denominator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Self::Output {
        Fraction::new(-self.numerator, self.denominator)
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl From<i32> for Fraction {
    fn from(value: i32) -> Self {
        Fraction::new(value as i128, 1)
    }
}
