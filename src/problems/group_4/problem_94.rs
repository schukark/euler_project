pub fn solve() -> i128 {
    const LIMIT: i64 = 1_000_000_000;
    let mut result = 0;

    for n in 1.. {
        if 4 * (n + 1) * (n + 1) > LIMIT && 2 * (2 * n + 1).pow(2) > LIMIT {
            break;
        }

        for m in n + 1.. {
            let p1 = 4 * m * m;
            let p2 = 2 * (m + n).pow(2);

            if p1 > LIMIT && p2 > LIMIT {
                break;
            }

            if p1 <= LIMIT && i64::abs(m.pow(2) - 3 * n.pow(2)) == 1 {
                result += p1 * (LIMIT / p1) * (LIMIT / p1 + 1) / 2;
            }

            if p2 <= LIMIT && i64::abs(m.pow(2) - 4 * m * n + n.pow(2)) == 1 {
                result += p2 * (LIMIT / p2) * (LIMIT / p2 + 1) / 2;
            }
        }
    }

    result as i128
}
