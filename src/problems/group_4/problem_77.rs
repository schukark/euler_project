use std::collections::HashSet;

use crate::utils::primes::prime_number_sieve;

pub fn solve() -> i128 {
    let primes = prime_number_sieve(1_000_000);
    let mut cur_set = vec![vec![vec![]], vec![]];
    for i in 2.. {
        let cur_result = generate_new_set(&primes, &cur_set, i);
        cur_set.push(cur_result.iter().cloned().collect::<Vec<_>>());

        if cur_set.last().unwrap().len() > 5000 {
            return i as i128;
        }
    }

    0
}

fn generate_new_set(
    primes: &[usize],
    previous: &[Vec<Vec<usize>>],
    new_index: usize,
) -> HashSet<Vec<usize>> {
    primes
        .iter()
        .take_while(|&&x| x <= new_index)
        .flat_map(|prime| {
            previous[new_index - prime]
                .iter()
                .map(|prev_set| {
                    let mut cloned = prev_set.clone();
                    cloned.push(*prime);
                    cloned.sort();
                    cloned
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>()
}
