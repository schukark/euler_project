use crate::utils::primes::prime_number_sieve;

pub fn solve(limit: i128) -> i128 {
    let (mut prime, mut length) = (0, 0);
    let primes = prime_number_sieve(limit as usize);
    let mut prefix = Vec::new();
    prefix.push(0);

    primes
        .iter()
        .for_each(|value| prefix.push(prefix.last().unwrap() + value));

    for i in 0..prefix.len() {
        for j in i + 1..prefix.len() - 1 {
            let cur_sum = prefix[j + 1] - prefix[i];

            if primes.binary_search(&cur_sum).is_err() {
                continue;
            }

            if length < j - i + 1 {
                length = j - i + 1;
                prime = cur_sum;
            }
        }
    }

    prime
}

#[test]
pub fn solve_100() {
    assert_eq!(solve(100), 41);
}

#[test]
pub fn solve_1000() {
    assert_eq!(solve(1000), 953);
}
