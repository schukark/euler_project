use crate::utils::primes::check_primes;

pub fn solve(limit: u64) -> u64 {
    let mut primes: Vec<u64> = Vec::new();
    let mut sum: u64 = 0;

    for i in 2..limit {
        if check_primes(i, &mut primes) {
            sum += i;
        }
    }

    sum
}
