fn is_pentagonal(number: i128) -> bool {
    // k = n(3n - 1) / 2 => 3n^2 - n - 2k = 0
    // n = (1 +- sqrt(1 + 24k)) / 2

    let n = (f64::sqrt(1.0_f64 + 24.0_f64 * number as f64) + 1.0_f64) / 6.0_f64;
    f64::abs(n - n.round()) < 1e-5
}

pub fn solve() -> i128 {
    assert!([1, 5, 12, 22, 35, 51, 70, 92, 117, 145]
        .iter()
        .all(|a| is_pentagonal(*a)));

    let mut result = i128::MAX;

    for n in 1..10_000 {
        for m in n + 1..10_000 {
            let first = n * (3 * n - 1) / 2;
            let second = m * (3 * m - 1) / 2;

            if is_pentagonal(first + second) && is_pentagonal(second - first) {
                result = i128::min(result, second - first);
            }
        }
    }

    result
}
