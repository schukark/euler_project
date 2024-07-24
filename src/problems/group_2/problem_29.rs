use std::collections::HashSet;

use crate::utils::primes::get_prime_exponents;

fn mult_array(powers: &mut Vec<(i64, i64)>, count: i64) {
    for number in powers {
        number.1 *= count;
    }
}

pub fn solve() -> i128 {
    let mut power_array = HashSet::new();

    for a in 2..=100 {
        for b in 2..=100 {
            let mut powers = get_prime_exponents(a);
            mult_array(&mut powers, b);

            power_array.insert(powers);
        }
    }

    power_array.len() as i128
}
