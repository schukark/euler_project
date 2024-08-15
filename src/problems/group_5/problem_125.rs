use std::collections::HashSet;

fn is_palindromic(number: i128) -> bool {
    number
        .to_string()
        .chars()
        .eq(number.to_string().chars().rev())
}

pub fn solve() -> i128 {
    const LIMIT: i128 = 10_i128.pow(8);

    let mut dp = [0; LIMIT.isqrt() as usize + 1];

    for i in 1..=LIMIT.isqrt() {
        dp[i as usize] = dp[i as usize - 1] + i * i;
    }

    let mut distinct_numbers = HashSet::new();

    for i in 0..dp.len() {
        for j in i + 2..dp.len() {
            let number = dp[j] - dp[i];

            if !is_palindromic(number) || number >= LIMIT {
                continue;
            }

            distinct_numbers.insert(number);
        }
    }

    distinct_numbers
        .iter()
        .copied()
        .reduce(|a, b| a + b)
        .unwrap()
}
