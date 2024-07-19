fn check_quasi_cancellation(num: i128, denom: i128) -> bool {
    if num % 10 == 0 && denom % 10 == 0 {
        return false;
    }

    if num * (denom / 10) == denom * (num / 10) && (denom / 10) == (num / 10) {
        return true;
    }
    if num * (denom % 10) == denom * (num / 10) && (denom / 10) == (num % 10) {
        return true;
    }
    if num * (denom / 10) == denom * (num % 10) && (denom % 10) == (num / 10) {
        return true;
    }
    if num * (denom % 10) == denom * (num % 10) && (denom % 10) == (num % 10) {
        return true;
    }

    false
}

pub fn solve() -> i128 {
    let (mut numerator, mut denominator) = (1, 1);

    for num in 10..100 {
        for denom in num + 1..100 {
            if !check_quasi_cancellation(num, denom) {
                continue;
            }

            numerator *= num;
            denominator *= denom;
        }
    }
    let binary_u64 =
        gcd::binary_u64(i128::abs(numerator) as u64, i128::abs(denominator) as u64) as i128;

    denominator / binary_u64
}
