use crate::utils::primes::prime_number_sieve;

fn check_number_divisors(n: i64, primes: &[i64]) -> bool {
    let mut d: i64 = 1;

    while d.pow(2) <= n {
        if n % d != 0 {
            d += 1;
            continue;
        }

        if primes.binary_search(&(d + n / d)).is_err() {
            return false;
        }

        d += 1;
    }

    true
}

#[test]
fn test_example() {
    let primes = prime_number_sieve(100);
    assert!(check_number_divisors(30, &primes));
}

pub fn solve() -> i128 {
    const LIMIT: i32 = 100_000_000;
    let primes = prime_number_sieve(LIMIT as usize + 1);

    primes
        .iter()
        .map(|&x| (x - 1, check_number_divisors(x - 1, &primes)))
        .filter(|(_x, boolean)| *boolean)
        .map(|(x, _value)| x)
        .sum::<i64>() as i128
}
