use std::collections::HashSet;

use crate::utils::primes::prime_number_sieve;

const LIMIT: i128 = 100_000_000_i128;

pub fn solve() -> i128 {
    let primes: Vec<i128> = prime_number_sieve(LIMIT as usize);

    let mut number_set = HashSet::new();

    for i in 0..primes.len() {
        for j in i..primes.len() {
            if primes[i] * primes[j] >= LIMIT {
                break;
            }

            number_set.insert(primes[i] * primes[j]);
        }
    }

    number_set.len() as i128
}
