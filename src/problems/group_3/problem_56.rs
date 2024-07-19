use num_bigint::BigUint;

fn calculate_digit_sum(number: &BigUint) -> i128 {
    number
        .to_string()
        .chars()
        .map(|digit| digit.to_digit(10).unwrap() as i128)
        .sum::<i128>()
}

pub fn solve() -> i128 {
    let mut max_sum = 0;

    for a in 1..100 {
        for b in 1..100 {
            let cur_num = BigUint::new(vec![a]).pow(b);

            let digit_sum = calculate_digit_sum(&cur_num);

            if digit_sum > max_sum {
                max_sum = digit_sum;
            }
        }
    }

    max_sum
}
