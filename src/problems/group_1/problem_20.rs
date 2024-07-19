use num_bigint::BigUint;

pub fn solve() -> u64 {
    let mut result = BigUint::new(vec![1]);

    for i in 2..=100 {
        result = BigUint::new(vec![i]) * &result;
    }

    result
        .to_string()
        .chars()
        .map(|a| a.to_digit(10).unwrap() as u64)
        .sum()
}
