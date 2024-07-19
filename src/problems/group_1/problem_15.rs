use std::collections::BTreeMap;

use crate::utils::math::choose;

pub fn solve(width: u64, height: u64) -> u64 {
    let mut map = BTreeMap::new();
    choose(width + height, height, &mut map)
}
