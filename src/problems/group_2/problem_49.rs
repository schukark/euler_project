use crate::utils::primes::prime_number_sieve;

fn _same_digits(number1: i128, number2: i128) -> bool {
    let mut digits = vec![0; 10];
    number1
        .to_string()
        .chars()
        .for_each(|a| digits[a.to_digit(10).unwrap() as usize] += 1);

    number2
        .to_string()
        .chars()
        .for_each(|a| digits[a.to_digit(10).unwrap() as usize] -= 1);

    digits == vec![0; 10]
}

fn same_digits(numbers: &[i128]) -> bool {
    for i in 1..numbers.len() {
        if !_same_digits(numbers[i], numbers[i - 1]) {
            return false;
        }
    }

    true
}

pub fn solve() -> i128 {
    let primes = prime_number_sieve(10_000);

    for middle in 1000..10_000 {
        for step in 1..i128::min(middle, 9_999 - middle) {
            if !same_digits(&[middle - step, middle, middle + step]) {
                continue;
            }

            if [middle - step, middle, middle + step]
                .iter()
                .any(|a| primes.binary_search(a).is_err())
            {
                continue;
            }

            if middle == 4817 {
                continue;
            }

            return ((middle - step).to_string()
                + &middle.to_string()
                + &(middle + step).to_string())
                .parse::<i128>()
                .unwrap();
        }
    }

    0
}
