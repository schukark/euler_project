use crate::utils::primes::get_prime_exponents;

fn count_solutions(number: i32) -> i32 {
    ((get_prime_exponents(number as i128)
        .iter()
        .map(|(_prime, exponent)| *exponent * 2 + 1)
        .product::<i128>()
        + 1)
        / 2) as i32
}

#[test]
fn test_example() {
    assert_eq!(count_solutions(4), 3);
    println!("{}", count_solutions(1013));
}

pub fn solve() -> i128 {
    (1..)
        .enumerate()
        .map(|(idx, value)| (idx + 1, count_solutions(value)))
        .find(|(_idx, value)| *value > 10_i32.pow(3))
        .unwrap()
        .0 as i128
}
