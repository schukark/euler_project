fn pow_mod(base: i128, exp: i128, modulus: i128) -> i128 {
    if exp == 0 {
        1
    } else if exp % 2 == 1 {
        return (base * pow_mod(base, exp - 1, modulus)) % modulus;
    } else {
        let result = pow_mod(base, exp / 2, modulus);
        return (result * result) % modulus;
    }
}

pub fn solve(limit: i128) -> i128 {
    let mut result = 0;
    const MOD: i128 = 10_i128.pow(10);

    for i in 1..=limit {
        result = (result + pow_mod(i, i, MOD)) % MOD;
    }

    result
}

#[test]
pub fn solve_10() {
    assert_eq!(solve(10), 405071317);
}
