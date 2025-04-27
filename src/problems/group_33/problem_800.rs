use crate::utils::primes::prime_number_sieve;

const LIMIT: usize = 800800;

pub(crate) fn solve() -> i128 {
    let primes: Vec<u64> = prime_number_sieve(10usize.pow(8));

    let mut answer = 0;

    for &p in &primes {
        for &q in &primes {
            if p <= q {
                continue;
            }

            let cur = p as f64 * (q as f64).ln() + q as f64 * (p as f64).ln();

            if cur > LIMIT as f64 * (LIMIT as f64).ln() {
                break;
            }

            answer += 1;
        }
    }

    answer
}
