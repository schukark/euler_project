use std::collections::HashSet;

use crate::utils::primes::prime_number_sieve;

const LIMIT: i32 = 50_000_000;

pub fn solve() -> i128 {
    let primes: Vec<i32> = prime_number_sieve((LIMIT as f64).sqrt().ceil() as usize);
    let prime_squares = primes.iter().map(|p| p * p).collect::<Vec<_>>();
    let prime_cubes = primes
        .iter()
        .map(|p| p.pow(3))
        .take_while(|d| *d < LIMIT)
        .collect::<Vec<_>>();
    let prime_fourth = primes
        .iter()
        .map(|p| p.pow(4))
        .take_while(|d| *d < LIMIT)
        .collect::<Vec<_>>();

    let mut answer_hashset: HashSet<i32> = HashSet::new();

    for sqr in &prime_squares {
        for cube in &prime_cubes {
            for fourth in &prime_fourth {
                let value = sqr + cube + fourth;

                if value >= LIMIT {
                    continue;
                }

                answer_hashset.insert(value);
            }
        }
    }

    answer_hashset.len() as i128
}
