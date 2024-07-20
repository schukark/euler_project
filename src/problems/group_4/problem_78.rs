use crate::utils::math::partition_mod;

pub fn solve() -> i128 {
    let mut i = 1;

    loop {
        if partition_mod(i, 1, 1_000_000) == 0 {
            return i;
        }

        i += 1;
    }
}
