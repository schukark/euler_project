use crate::utils::math::partition;

pub fn solve(number: i128) -> i128 {
    partition(number, 1) - 1
}

#[test]
fn test_6() {
    assert_eq!(partition(6, 1), 11);
}

#[test]
fn test_5() {
    assert_eq!(partition(5, 1), 7);
}

#[test]
fn test_4() {
    assert_eq!(partition(4, 1), 5);
}
