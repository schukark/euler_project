fn pow_mod(mut base: i128, mut exp: i128, p: i128) -> i128 {
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (base * result) % p;
        }

        exp /= 2;
        base = (base * base) % p;
    }

    result
}

pub fn solve() -> i128 {
    let exp = pow_mod(2, 7830457, 10_i128.pow(10));

    (exp * 28433 + 1) % 10_i128.pow(10)
}
