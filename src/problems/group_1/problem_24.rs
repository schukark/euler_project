use crate::utils::math::next_permutation;

pub fn solve() -> u64 {
    let mut nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for _i in 1..1_000_000 {
        next_permutation(&mut nums);
    }

    nums.map(|a| a.to_string()).join("").parse::<u64>().unwrap()
}
