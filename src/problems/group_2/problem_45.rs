fn is_triangular(number: i128) -> bool {
    // k = n(n + 1) / 2 => n^2 + n - 2k = 0
    // n = (-1 +- sqrt(1 + 8k)) / 2

    let n = (f64::sqrt(1.0_f64 + 8.0_f64 * number as f64) - 1.0_f64) / 2.0_f64;
    f64::abs(n - n.round()) < 1e-5
}

fn is_pentagonal(number: i128) -> bool {
    // k = n(3n - 1) / 2 => 3n^2 - n - 2k = 0
    // n = (1 +- sqrt(1 + 24k)) / 6

    let n = (f64::sqrt(1.0_f64 + 24.0_f64 * number as f64) + 1.0_f64) / 6.0_f64;
    f64::abs(n - n.round()) < 1e-5
}

pub fn solve() -> i128 {
    let mut cur_hex = 144;

    loop {
        let cur_num = cur_hex * (2 * cur_hex - 1);

        if is_triangular(cur_num) && is_pentagonal(cur_num) {
            return cur_num;
        }
        cur_hex += 1;
    }
}
