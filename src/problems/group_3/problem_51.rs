use crate::utils::primes::prime_number_sieve;

fn replace(number: i128, positions: &[i128]) -> Vec<i128> {
    let number_string = number.to_string();
    let mut answer = Vec::new();

    for digit in 0..=9 {
        if positions.binary_search(&0).is_ok() && digit == 0 {
            continue;
        }

        let mut cur = 0;
        for i in 0..number_string.len() {
            if positions.binary_search(&(i as i128)).is_err() {
                cur =
                    10 * cur + number_string.chars().nth(i).unwrap().to_digit(10).unwrap() as i128;
            } else {
                cur = 10 * cur + digit;
            }
        }

        if !answer.is_empty() && *answer.last().unwrap() == cur {
            continue;
        }

        answer.push(cur);
    }

    answer
}

#[test]
pub fn test_replace() {
    assert_eq!(
        replace(56003, &[2, 3]),
        vec![56003, 56113, 56223, 56333, 56443, 56553, 56663, 56773, 56883, 56993]
    );

    assert_eq!(replace(13, &[0]), vec![13, 23, 33, 43, 53, 63, 73, 83, 93]);
}

fn check_positions(number: i128, positions: &[i128]) -> bool {
    for i in 1..positions.len() {
        if number
            .to_string()
            .chars()
            .nth(positions[i] as usize)
            .unwrap()
            .to_digit(10)
            .unwrap()
            != number
                .to_string()
                .chars()
                .nth(positions[0] as usize)
                .unwrap()
                .to_digit(10)
                .unwrap()
        {
            return false;
        }
    }

    true
}

#[test]
pub fn test_check_positions() {
    assert!(check_positions(56003, &[2, 3]));
    assert!(!check_positions(56003, &[2, 4]));
}

pub fn solve(prime_count: i128) -> i128 {
    let primes: Vec<i128> = prime_number_sieve(1_000_000);
    for prime in &primes {
        for mask in 1..2_u32.pow(prime.to_string().len() as u32) {
            // mask to generate positions
            // after generating, replace to acquire the vector of numbers
            // check if at least 8 of them are primes
            // output

            let positions = gen_positions(mask);

            if !check_positions(*prime, &positions) {
                continue;
            }

            let mut count = 0;
            replace(*prime, &positions).iter().for_each(|a| {
                if primes.contains(a) {
                    count += 1;
                }
            });

            if count >= prime_count {
                let answer = replace(*prime, &positions)
                    .iter()
                    .filter(|a| primes.contains(a))
                    .copied()
                    .collect::<Vec<i128>>();
                return *answer.first().unwrap();
            }
        }
    }

    0
}

fn gen_positions(mask: u32) -> Vec<i128> {
    let mut mask_mut = mask;
    let mut cur_index = 0;
    let mut positions = Vec::new();

    while mask_mut > 0 {
        if mask_mut & 1 == 1 {
            positions.push(cur_index);
        }

        cur_index += 1;
        mask_mut >>= 1;
    }
    positions
}

#[test]
pub fn test_positions() {
    assert_eq!(gen_positions(19), vec![0, 1, 4]); //19 = 1 + 2 + 0 * 4 + 0 * 8 + 1 * 16
}

#[test]
pub fn test_primes_6() {
    assert_eq!(solve(6), 13);
}

// #[test]
// pub fn test_primes_7() {
//     assert_eq!(solve(7), 56003);
// }
