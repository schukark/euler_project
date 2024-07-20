pub fn solve() -> i128 {
    let mut n = 1;

    let mut best_area = 0;
    let mut best_count = 0;

    loop {
        let mut m = 1;

        if n * (n + 1) / 2 > 2_000_000 {
            break;
        }

        loop {
            let current_count = n * (n + 1) / 2 * m * (m + 1) / 2;

            if i128::abs(current_count - 2_000_000) < i128::abs(best_count - 2_000_000) {
                best_count = current_count;
                best_area = n * m;
            }

            if current_count > 2_000_000 {
                break;
            }

            m += 1;
        }

        n += 1;
    }

    best_area
}
