use num_bigint::BigUint;
use num_traits::FromPrimitive;

pub fn solve() -> i128 {
    (1..=100_u64)
        .filter(|&x| x.isqrt().pow(2) != x)
        .map(calculate_digit_sum)
        .map(|x| x as i128)
        .sum()
}

fn calculate_digit_sum(number: u64) -> u64 {
    let mut new_num = BigUint::from_u64(number).unwrap();
    new_num *= BigUint::from_u64(10).unwrap().pow(200);

    new_num
        .sqrt()
        .to_string()
        .chars()
        .take(100)
        .map(|x| x.to_digit(10).unwrap() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::problems::group_4::problem_80::calculate_digit_sum;

    #[test]
    fn test_example() {
        assert_eq!(calculate_digit_sum(2), 475);
    }
}
