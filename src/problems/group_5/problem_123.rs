use crate::utils::primes::prime_number_sieve;

/// (p_n - 1) ^ n + (p_n + 1) ^ n mod p_n^2 =
/// (-1) ^ n + (-1) ^ (n - 1) * n * p_n + 1 ^ n + n * p_n
/// if n is even, then it equals 2
/// if n is odd, then it equals 2n * p_n
///
/// The remainder exceeds 10 ^ 10 => n * p_n >= 10 ^ 10 / 2
pub fn solve() -> i128 {
    let primes: Vec<i128> = prime_number_sieve(100_000_000);
    let new_primes = primes
        .iter()
        .enumerate()
        .map(|(idx, value)| (idx, (value * (idx as i128 + 1)) % value.pow(2)))
        .filter(|(idx, _value)| idx % 2 == 0)
        .collect::<Vec<(usize, i128)>>();

    println!("{:?}", &new_primes[..10]);

    let index = new_primes
        .iter()
        .find(|(_idx, value)| *value >= 10_i128.pow(10) / 2)
        .unwrap()
        .0;

    index as i128 + 1
}
