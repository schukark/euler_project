fn _check_same_digits(number1: &i128, number2: &i128) -> bool {
    let mut digits = [0; 10];

    number1
        .to_string()
        .chars()
        .map(|a| a.to_digit(10).unwrap() as usize)
        .for_each(|a| digits[a] += 1);
    number2
        .to_string()
        .chars()
        .map(|a| a.to_digit(10).unwrap() as usize)
        .for_each(|a| digits[a] -= 1);

    digits == [0; 10]
}

fn check_same_digits(numbers: &[i128]) -> bool {
    numbers
        .iter()
        .all(|a| _check_same_digits(a, numbers.first().unwrap()))
}

pub fn solve() -> i128 {
    let mut cur_num = 1;

    loop {
        if check_same_digits(&(1..=5).map(|a| a * cur_num).collect::<Vec<_>>()) {
            return cur_num;
        }

        cur_num += 1;
    }
}
