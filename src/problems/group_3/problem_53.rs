use std::collections::BTreeMap;

use crate::utils::math::choose;

pub fn solve() -> i128 {
    let mut choose_map = BTreeMap::new();

    let mut count = 0;

    for n in 1..=100 {
        for k in 1..=100 {
            if choose(n, k, &mut choose_map) > 1_000_000 {
                count += 1;
            }
        }
    }

    count
}
