use num_bigint::BigUint;

fn is_palindromic(number: &BigUint) -> bool {
    number
        .to_string()
        .chars()
        .eq(number.to_string().chars().rev())
}

fn iterate(num: i128, limit: i128) -> i128 {
    let mut count = 0;
    let mut cur_num = num.to_string().parse::<BigUint>().unwrap();

    loop {
        cur_num = cur_num.clone()
            + cur_num
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<BigUint>()
                .unwrap();
        count += 1;
        if is_palindromic(&cur_num) || count > limit {
            break;
        }
    }

    count
}

#[test]
pub fn test_iteration() {
    assert_eq!(iterate(47, 50), 1);
    assert_eq!(iterate(349, 50), 3);
    assert!(iterate(196, 50) > 50);
}

pub fn solve() -> i128 {
    const LIMIT: i128 = 50_i128;
    let mut count = 0;

    for i in 1..10_000 {
        if iterate(i, LIMIT) > LIMIT {
            count += 1;
        }
    }

    count
}
