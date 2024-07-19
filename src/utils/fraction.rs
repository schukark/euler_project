use std::{fmt::Display, ops::Mul};

#[derive(Copy, Clone)]
pub struct Fraction {
    pub numerator: u128,
    pub denominator: u128,
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
    pub fn new(numerator: u128, denominator: u128) -> Fraction {
        let new_numerator = numerator / gcd::binary_u128(numerator, denominator);
        let new_denominator = denominator / gcd::binary_u128(numerator, denominator);

        Fraction {
            numerator: new_numerator,
            denominator: new_denominator,
        }
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

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
