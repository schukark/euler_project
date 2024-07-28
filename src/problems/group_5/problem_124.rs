use crate::utils::primes::prime_number_sieve;

fn radical(mut n: i32, primes: &[i32]) -> i32 {
    let mut i = 0;
    let mut answer = 1;

    while i < primes.len() && primes[i] * primes[i] <= n {
        if n % primes[i] != 0 {
            i += 1;
            continue;
        }

        while n % primes[i] == 0 {
            n /= primes[i];
        }

        answer *= primes[i];
        i += 1;
    }

    if n > 1 && primes.binary_search(&n).is_ok() {
        answer *= n;
    }

    answer
}

#[test]
fn test_radical() {
    let primes = prime_number_sieve(10);
    assert_eq!(radical(9, &primes), 3);
    assert_eq!(radical(8, &primes), 2);
}

pub fn solve() -> i128 {
    const LIMIT: i32 = 100_000;

    let primes = prime_number_sieve(LIMIT as usize);
    let mut result = (0..=LIMIT)
        .map(|x| radical(x, &primes))
        .enumerate()
        .collect::<Vec<_>>();
    result
        .sort_by(|(idx_a, value_a), (idx_b, value_b)| value_a.cmp(value_b).then(idx_a.cmp(idx_b)));

    let result = result
        .iter()
        .map(|(idx, _value)| idx)
        .copied()
        .collect::<Vec<_>>();

    result[10_000] as i128
}
