fn check_pandigital(number: i128) -> bool {
    let mut digits = [0; 10];
    digits[0] = 1;

    number
        .to_string()
        .chars()
        .for_each(|symbol| digits[symbol.to_digit(10).unwrap() as usize] += 1);

    digits == [1; 10]
}

pub fn solve() -> i128 {
    let mut result = 0;

    for number in 9..1_000_000 {
        let mut cur_string = number.to_string();
        for n in 2..=9 {
            cur_string += &(number * n).to_string();

            if cur_string.len() > 9 {
                break;
            }

            if !check_pandigital(cur_string.parse::<i128>().unwrap()) {
                continue;
            }

            result = i128::max(result, cur_string.parse::<i128>().unwrap());
            break;
        }
    }

    result
}
