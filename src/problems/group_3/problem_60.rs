use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;

use crate::utils::primes::prime_number_sieve;

pub fn solve(tuple_size: usize) -> i128 {
    let mut tuples = helper(tuple_size);
    tuples.sort_by_key(|x| x.iter().sum::<u32>());

    tuples.first().unwrap().iter().sum::<u32>() as i128
}

fn helper(tuple_size: usize) -> Vec<Vec<u32>> {
    assert!(tuple_size > 0);
    if tuple_size == 1 {
        return PRIMES.iter().map(|&x| vec![x]).collect::<Vec<_>>();
    }

    let mut answer = Vec::new();

    let prev_tuple = helper(tuple_size - 1);

    'outer: for tuple in prev_tuple {
        let mut sets = Vec::new();

        for &elem in &tuple {
            if !PRIME_MAP.contains_key(&elem) {
                continue 'outer;
            }

            sets.push(PRIME_MAP.get(&elem).unwrap());
        }

        let candidates = sets
            .first()
            .unwrap()
            .iter()
            .filter(|x| sets.iter().all(|s| s.contains(x)))
            .copied()
            .collect::<HashSet<_>>();

        if candidates.is_empty() {
            continue;
        }

        for candidate in candidates {
            let mut new_tuple = tuple.clone();
            new_tuple.push(candidate);

            answer.push(new_tuple);
        }
    }

    answer
}

lazy_static! {
    static ref LIMIT: usize = 1e4 as usize;
    static ref PRIMES: Vec<u32> = prime_number_sieve(*LIMIT);
    static ref PRIME_MAP: HashMap<u32, HashSet<u32>> = construct_prime_map();
}

#[inline]
fn concat_u64(a: u32, b: u32) -> u64 {
    let mut pow10 = 1u64;
    let mut tmp = b as u64;
    while tmp > 0 {
        pow10 *= 10;
        tmp /= 10;
    }
    (a as u64) * pow10 + (b as u64)
}

fn is_prime_u64(n: u64) -> bool {
    if n <= *LIMIT as u64 {
        PRIMES.binary_search(&(n as u32)).is_ok()
    } else {
        // paste in a standard deterministic Miller–Rabin for u64
        miller_rabin::is_prime(&n, 16)
    }
}

fn construct_prime_map() -> HashMap<u32, HashSet<u32>> {
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (i, &p) in PRIMES.iter().enumerate() {
        for &q in &PRIMES[i + 1..] {
            let pq = concat_u64(p, q);
            let qp = concat_u64(q, p);

            if is_prime_u64(pq) && is_prime_u64(qp) {
                map.entry(p).or_default().insert(q);
                map.entry(q).or_default().insert(p);
            }
        }
    }

    println!("Constructed prime map!");

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(792, solve(4));
    }
}
