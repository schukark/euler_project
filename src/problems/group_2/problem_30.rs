// if n (number of digits) >= 6
// sum of powers of digits <= n * 9 ^ 5 < 10^5 * n < 10^n (for n >= 6)

pub fn solve() -> i128 {
    let mut result = 0;

    for i in 10..1_000_000 {
        let sum_digits: i128 = i
            .to_string()
            .chars()
            .map(|a| a.to_digit(10).unwrap().pow(5) as i128)
            .sum();

        if sum_digits != i as i128 {
            continue;
        }

        result += i as i128;
    }

    result
}
