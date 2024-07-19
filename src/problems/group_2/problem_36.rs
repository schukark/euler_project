fn is_palindromic(number: i128) -> bool {
    number
        .to_string()
        .chars()
        .eq(number.to_string().chars().rev())
}

fn is_palindromic_binary(number: i128) -> bool {
    format!("{number:b}").eq(&format!("{number:b}").chars().rev().collect::<String>())
}

pub fn solve() -> i128 {
    let mut result = 0;

    for number in 1..1_000_000 {
        if !is_palindromic(number) || !is_palindromic_binary(number) {
            continue;
        }

        if number % 2 == 0 && number % 10 == 0 {
            continue;
        }

        result += number;
    }

    result
}
