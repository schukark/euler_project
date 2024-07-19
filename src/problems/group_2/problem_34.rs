pub fn solve() -> i128 {
    let mut result = 0;

    for number in 3..10_000_000 {
        let factorial_sum: i128 = number
            .to_string()
            .chars()
            .map(|a| (1..=a.to_digit(10).unwrap() as i128).product::<i128>())
            .sum();

        if factorial_sum != number {
            continue;
        }

        result += number;
    }

    result
}
