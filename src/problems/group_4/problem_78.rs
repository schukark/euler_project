use std::collections::HashMap;

use crate::utils::math::partition_fast_mod;

pub fn solve() -> i128 {
    let mut lookup = HashMap::new();
    let mut number = 1;

    loop {
        if partition_fast_mod(number, &mut lookup, 1_000_000) == 0 {
            return number as i128;
        }

        number += 1;
    }
}
