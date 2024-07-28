use std::collections::{BTreeMap, HashSet};

use crate::utils::{math::choose, primes::prime_number_sieve};

fn is_square_free(n: u64, primes: &[u64]) -> bool {
    let mut i = 0;

    while i < primes.len() && primes[i].pow(2) <= n {
        if n % primes[i].pow(2) == 0 {
            return false;
        }

        i += 1;
    }

    true
}

pub fn solve() -> i128 {
    let primes = prime_number_sieve(1_000_000);
    let mut map = BTreeMap::new();

    let mut distinct = HashSet::new();

    for n in 0..51 {
        for k in 1..=n {
            let tmp = choose(n, k, &mut map);

            if is_square_free(tmp, &primes) {
                distinct.insert(tmp);
            }
        }
    }

    distinct.iter().sum::<u64>() as i128
}
