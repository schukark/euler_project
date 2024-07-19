pub fn solve(limit: u64) -> i128 {
    let mut lookup: Vec<u64> = vec![0; limit as usize + 1];

    for n in 1..=limit {
        for m in n + 1..=limit {
            if (m - n) % 2 == 0 || gcd::binary_u64(n, m) != 1 {
                continue;
            }

            let perimeter = 2 * m * (m + n);

            if perimeter > limit {
                break;
            }

            for i in 1..=(limit / perimeter) {
                lookup[(perimeter * i) as usize] += 1;
            }
        }
    }

    let mut result = 0;

    for value in lookup.iter() {
        if *value == 1 {
            result += 1;
        }
    }

    result
}

#[test]
pub fn test_solve() {
    assert_eq!(solve(48), 6);
}
