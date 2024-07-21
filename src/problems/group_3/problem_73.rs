use crate::utils::fraction::Fraction;

pub fn solve(left: Fraction, right: Fraction, limit: i128) -> i128 {
    let mut result = 0;

    for n in 2..=limit {
        let mut lower_bound = (left.numerator * n + left.denominator - 1) / left.denominator;
        let mut upper_bound = right.numerator * n / right.denominator;

        if Fraction::new(lower_bound, n) == left {
            lower_bound += 1;
        }

        if Fraction::new(upper_bound, n) == right {
            upper_bound -= 1;
        }

        for i in lower_bound..=upper_bound {
            if gcd::binary_u128(i as u128, n as u128) == 1 {
                result += 1;
            }
        }
    }

    result
}
