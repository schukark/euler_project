use crate::utils::primes::count_div;

pub fn solve(limit: u64) -> u64 {
    let mut sum = 1;
    let mut adder = 1;

    loop {
        if count_div(sum) > limit {
            return sum;
        }

        adder += 1;
        sum += adder;
    }
}
