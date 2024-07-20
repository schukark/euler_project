use crate::utils::primes::totient_sieve;

pub fn solve(limit: usize) -> i128 {
    let sieve = totient_sieve(limit);

    sieve.iter().skip(2).sum()
}
