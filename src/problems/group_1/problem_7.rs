use crate::utils::primes::check_primes;

pub fn solve(limit: u64) -> u64 {
    let mut count = 0;
    let mut current_num = 2;

    let mut primes: Vec<u64> = Vec::new();

    while count < limit {
        if check_primes(current_num, &mut primes) {
            count += 1;
        }
        current_num += 1;
    }

    current_num - 1
}
