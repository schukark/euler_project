use crate::utils::primes::prime_number_sieve;

pub fn solve() -> i128 {
    let primes = prime_number_sieve(1_000_000);

    for number in 4..1_000_000 {
        let mut sq = 1;
        let mut is_representable = false;

        if number % 2 == 0 || primes.binary_search(&number).is_ok() {
            continue;
        }

        while 2 * sq * sq < number {
            if primes.binary_search(&(number - 2 * sq * sq)).is_ok() {
                is_representable = true;
                break;
            }

            sq += 1;
        }

        if !is_representable {
            return number;
        }
    }

    i128
::MAX
}
