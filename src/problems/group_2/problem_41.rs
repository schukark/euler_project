use crate::utils::primes::prime_number_sieve;

fn check_pandigital(number: i128, n: usize) -> bool {
    let mut digits = vec![0; n + 1];
    digits[0] = 1;

    if number
        .to_string()
        .chars()
        .any(|f| f.to_digit(10).unwrap() as usize > n)
    {
        return false;
    }

    number
        .to_string()
        .chars()
        .for_each(|a| digits[a.to_digit(10).unwrap() as usize] += 1);

    digits == vec![1; n + 1]
}

pub fn solve() -> i128 {
    assert!(check_pandigital(2143, 4));

    let mut result = 0;

    // sum of 1 + .. + 9 = 45, which is divisible by 9 => not a prime
    // sum of 1 + .. + 8 = 36 which is divisible by 9 => not a prime
    // the search space is now up to 10 ^ 7 - 1
    let primes = prime_number_sieve(10_000_000);

    for i in 2..10_000_000 {
        if primes.binary_search(&i).is_err() {
            continue;
        }

        let n = i.to_string().len();

        if !check_pandigital(i, n) {
            continue;
        }

        result = i;
    }

    result
}
