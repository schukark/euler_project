use std::collections::HashSet;

use crate::utils::primes::prime_number_sieve;

fn get_all_rotations(number: i128) -> Vec<i128> {
    let mut result: Vec<i128> = Vec::new();

    let number_string = number.to_string();

    for i in 0..number_string.len() {
        result.push(
            (number_string[i..].to_owned() + &number_string[..i])
                .parse::<i128>()
                .unwrap(),
        );
    }

    result
}

pub fn solve(limit: i128) -> i128 {
    let primes = prime_number_sieve(limit as usize);
    let mut answer: HashSet<i128> = HashSet::new();

    for i in 2..limit {
        if answer.contains(&i) || primes.binary_search(&i).is_err() {
            continue;
        }

        if get_all_rotations(i)
            .iter()
            .any(|f| primes.binary_search(f).is_err())
        {
            continue;
        }

        get_all_rotations(i).iter().for_each(|f| {
            answer.insert(*f);
        });
    }

    answer.len() as i128
}
