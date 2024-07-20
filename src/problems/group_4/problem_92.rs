fn ends_in_89(number: i64, lookup: &mut [Option<bool>]) -> bool {
    let mut current = number;

    while current != 1 && current != 89 {
        if lookup[current as usize].is_some() {
            return lookup[current as usize].unwrap();
        }

        current = current
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i64)
            .map(|d| d.pow(2))
            .sum();
    }

    lookup[number as usize] = Some(current == 89);
    current == 89
}

#[test]
fn test_85() {
    let mut lookup = vec![None; LIMIT as usize];
    assert!(ends_in_89(85, &mut lookup));
}

const LIMIT: i64 = 10_000_000;

pub fn solve() -> i128 {
    let mut lookup = vec![None; LIMIT as usize];
    (1..LIMIT)
        .map(|x| ends_in_89(x, &mut lookup))
        .filter(|d| *d)
        .count() as i128
}
