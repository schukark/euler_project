use std::collections::HashSet;

fn sum_factorial_digit(number: u128) -> u128 {
    let mut answer = 0;

    for digit in number.to_string().chars() {
        answer += (1..=digit.to_digit(10).unwrap() as u128).product::<u128>();
    }

    answer
}

fn iterate(number: u128, current_chain: &mut HashSet<u128>) -> u128 {
    if current_chain.contains(&number) {
        return 0;
    }
    current_chain.insert(number);

    let tmp = sum_factorial_digit(number);

    iterate(tmp, current_chain) + 1
}

pub fn solve(loop_cnt: u128, limit: u128) -> i128 {
    let mut answer = 0;

    for current_number in 1..limit {
        let mut current_chain = HashSet::new();

        if iterate(current_number, &mut current_chain) == loop_cnt {
            answer += 1;
        }
    }

    answer
}

#[allow(dead_code)]
fn test(number: u128, expected: u128) {
    let mut current_chain = HashSet::new();

    assert_eq!(iterate(number, &mut current_chain), expected);
}

#[test]
pub fn test_69() {
    test(69, 5);
}

#[test]
pub fn test_145() {
    test(145, 1);
}

#[test]
pub fn test_871() {
    test(871, 2);
}

#[test]
pub fn test_872() {
    test(872, 2);
}

#[test]
pub fn test_78() {
    test(78, 4);
}

#[test]
pub fn test_540() {
    test(540, 2);
}
