use crate::utils::primes::prime_number_sieve;

pub fn solve(limit: i128) -> i128 {
    let primes: Vec<i128> = prime_number_sieve(1_000_000);

    let mut result = 1;

    for prime in primes {
        if result * prime <= limit {
            result *= prime;
        } else {
            break;
        }
    }

    result
}
