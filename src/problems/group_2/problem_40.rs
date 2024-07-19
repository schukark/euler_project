pub fn solve() -> i128 {
    let mut cur = 1;
    let mut cur_string = String::new();

    while cur_string.len() < 1_000_000 {
        cur_string += &cur.to_string();
        cur += 1;
    }

    let mut result = 1;

    assert_eq!(cur_string.chars().nth(11).unwrap(), '1');

    for i in [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000] {
        result *= cur_string
            .chars()
            .nth(i as usize - 1)
            .unwrap()
            .to_digit(10)
            .unwrap() as i128;
    }

    result
}
