use crate::utils::primes::prime_number_sieve;

pub fn solve(ratio: f32) -> i128 {
    // TR diagonal: 4n^2-2n+1
    // TL diagonal: 4n^2+1
    // BL diagonal: 4n^2+2n+1

    let primes = prime_number_sieve(1_000_000_000);

    let mut i = 1;
    let mut count = 0;

    loop {
        if primes.binary_search(&(i * (4 * i - 2) + 1)).is_ok() {
            count += 1;
        }

        if primes.binary_search(&(4 * i * i + 1)).is_ok() {
            count += 1;
        }

        if primes.binary_search(&(4 * i * i + 2 * i + 1)).is_ok() {
            count += 1;
        }

        if count as f32 / (4.0_f32 * i as f32 + 1.0_f32) < ratio {
            return 2 * i + 1;
        }

        i += 1;
    }
}
