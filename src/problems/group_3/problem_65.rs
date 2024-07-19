use num_bigint::BigUint;

fn nth_term(index: u128) -> u128 {
    if index == 0 {
        return 2;
    } else if index % 3 == 2 {
        return (index / 3 + 1) * 2;
    }

    1
}

pub fn solve(index: u128) -> i128 {
    let mut numerator = (
        BigUint::parse_bytes(b"2", 10).unwrap(),
        BigUint::parse_bytes(b"3", 10).unwrap(),
    );
    let mut denominator = (
        BigUint::parse_bytes(b"1", 10).unwrap(),
        BigUint::parse_bytes(b"1", 10).unwrap(),
    );

    for i in 2..index {
        numerator = (numerator.1.clone(), numerator.0 + numerator.1 * nth_term(i));
        denominator = (
            denominator.1.clone(),
            denominator.0 + denominator.1 * nth_term(i),
        );
    }

    numerator
        .1
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i128)
        .sum::<i128>()
}
