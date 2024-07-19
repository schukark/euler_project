use num_bigint::BigUint;

fn digit_sum(s: &str) -> u64 {
    s.chars().map(|a| a.to_digit(10).unwrap() as u64).sum()
}

pub fn solve(limit: u64) -> u64 {
    let mut result = BigUint::new(vec![1]);

    for _i in 0..limit {
        result = &result + &result;
    }

    digit_sum(&result.to_string())
}
