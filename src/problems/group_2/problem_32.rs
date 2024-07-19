fn check_pandigital(mult1: i128, mult2: i128, prod: i128) -> bool {
    let mut digits = [0; 10];
    digits[0] = 1;

    mult1
        .to_string()
        .chars()
        .for_each(|f| digits[f.to_digit(10).unwrap() as usize] += 1);

    mult2
        .to_string()
        .chars()
        .for_each(|f| digits[f.to_digit(10).unwrap() as usize] += 1);

    prod.to_string()
        .chars()
        .for_each(|f| digits[f.to_digit(10).unwrap() as usize] += 1);

    digits == [1; 10]
}

pub fn solve() -> i128 {
    let mut result = 0;
    for product in 1000..10000 {
        for mult1 in 1..100 {
            if product % mult1 != 0 {
                continue;
            }

            if check_pandigital(mult1, product / mult1, product) {
                result += product;
                break;
            }
        }
    }

    result
}
