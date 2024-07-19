use crate::utils::primes::prime_number_sieve;

fn check_left_truncatable(number: i128, primes: &[i128]) -> bool {
    let number_string = number.to_string();

    for i in 0..number_string.len() {
        if primes
            .binary_search(&(number_string[i..]).parse::<i128>().unwrap())
            .is_err()
        {
            return false;
        }
    }

    true
}

fn check_right_truncatable(number: i128, primes: &[i128]) -> bool {
    let number_string = number.to_string();

    for i in 1..=number_string.len() {
        if primes
            .binary_search(&(number_string[..i]).parse::<i128>().unwrap())
            .is_err()
        {
            return false;
        }
    }

    true
}

pub fn solve() -> i128 {
    let mut result = 0;
    let primes = prime_number_sieve(1_000_000);

    for i in 10..1_000_000 {
        if primes.binary_search(&i).is_err() {
            continue;
        }

        if !check_left_truncatable(i, &primes) || !check_right_truncatable(i, &primes) {
            continue;
        }

        result += i;
    }

    result
}
