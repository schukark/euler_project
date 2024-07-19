use crate::utils::math::sum_divisors;

pub fn solve(limit: u64) -> u64 {
    let mut result = 0;

    for i in 2..limit {
        let sum_div = sum_divisors(i);
        if sum_div == i {
            continue;
        }

        if sum_divisors(sum_div) == i {
            result += i;
        }
    }

    result
}
