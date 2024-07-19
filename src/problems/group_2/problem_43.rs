use crate::utils::math::next_permutation;

pub fn solve() -> i128 {
    let mut count = 0;

    let mut digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let divisors = [1, 2, 3, 5, 7, 11, 13, 17];

    while next_permutation(&mut digits) {
        if digits[0] == 0 {
            continue;
        }

        if digits.iter().enumerate().all(|(index, _value)| {
            index >= 8
                || (digits[index] * 100 + digits[index + 1] * 10 + digits[index + 2])
                    % divisors[index]
                    == 0
        }) {
            count += digits
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<i128>()
                .unwrap();
        }
    }

    count
}
