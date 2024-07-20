use crate::utils::primes::totient_sieve;

fn same_digits(number1: i128, number2: i128) -> bool {
    number1
        .to_string()
        .chars()
        .fold([0; 10], |mut acc: [u32; 10], x| {
            acc[x.to_digit(10).unwrap() as usize] += 1;
            acc
        })
        == number2
            .to_string()
            .chars()
            .fold([0; 10], |mut acc: [u32; 10], x| {
                acc[x.to_digit(10).unwrap() as usize] += 1;
                acc
            })
}

#[test]
fn test_same_digits() {
    assert!(same_digits(87109, 79180));
}

pub fn solve(limit: usize) -> i128 {
    let sieve = totient_sieve(limit);

    let mut best_number = 2;

    for i in 2..=limit {
        if !same_digits(i as i128, sieve[i]) {
            continue;
        }

        if best_number * sieve[i] > i as i128 * sieve[best_number as usize] {
            best_number = i as i128;
        }
    }

    best_number
}
