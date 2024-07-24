use crate::utils::primes::{get_factorization_fast, prime_number_sieve};

fn count_solutions(number: i32, primes: &[i32]) -> i32 {
    ((get_factorization_fast(number, primes)
        .iter()
        .map(|(_prime, exponent)| *exponent * 2 + 1)
        .product::<i32>()
        + 1)
        / 2) as i32
}

pub fn solve() -> i128 {
    let primes = prime_number_sieve(1_000_000);

    (1..)
        .enumerate()
        .map(|(idx, value)| (idx + 1, count_solutions(value, &primes)))
        .find(|(_idx, value)| *value > 10_i32.pow(3))
        .unwrap()
        .0 as i128
}
