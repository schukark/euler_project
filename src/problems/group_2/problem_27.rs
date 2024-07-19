use crate::utils::primes::prime_number_sieve;

fn find_consectuive_amount(a: i128, b: i128, primes: &[i128]) -> u64 {
    let mut count = 0;
    let mut n = 0;

    loop {
        if primes.binary_search(&(n * n + a * n + b)).is_err() {
            break;
        }

        count += 1;
        n += 1;
    }

    count
}

#[test]
pub fn test_euler() {
    let primes = prime_number_sieve(100000);
    assert_eq!(find_consectuive_amount(1, 41, &primes), 40);
}

pub fn solve() -> i128 {
    let (mut best_prod, mut best_count) = (0, 0);

    let primes = prime_number_sieve(100_000_000);

    for a in -999..1000 {
        for b in -999..1000 {
            let count = find_consectuive_amount(a, b, &primes);
            if count > best_count {
                best_count = count;
                best_prod = a * b;
            }
        }
    }

    best_prod
}
