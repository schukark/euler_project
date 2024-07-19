use num_bigint::BigUint;

use crate::utils::file_ops::read_file;

pub fn solve() -> u64 {
    let read_to_string = read_file("src/txts/prob_13.txt")
        .iter()
        .map(|f| f.parse::<BigUint>().unwrap())
        .collect::<Vec<_>>();

    let mut result = read_to_string[0].clone();

    (1..read_to_string.len()).for_each(|i| {
        result = &result + &read_to_string[i];
    });

    result.to_string()[..10].to_owned().parse::<u64>().unwrap()
}
