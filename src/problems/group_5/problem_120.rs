/// # Solution
///
/// r = (a - 1)^n + (a + 1)^n mod a^2
///
/// r = n(-1)^(n-1)a + (-1)^n + na + 1^n
///
/// if n is even: r = 2, if n is odd: r = 2na
///
/// It is obvious, that n being odd will produce maximal results
///
/// 2na = ka^2 + r => r is divisible by a
///
/// if a is even, then 2na mod a^2 will be optimal if 2na is the last even multiple of 2a before a^2 => 2na = a^2 - 2a
///
/// if a is odd, then 2na mod a^2 will be optimal if 2na is the last multiplte of a before a^2 => 2na = a^2 - a

pub fn solve() -> i128 {
    (3..=1000).map(solve_a).sum::<i32>() as i128
}

fn solve_a(a: i32) -> i32 {
    match a % 2 {
        0 => a * (a - 2),
        1 => a * (a - 1),
        _ => panic!(),
    }
}

#[test]
fn test_example() {
    assert_eq!(solve_a(7), 42);
}
