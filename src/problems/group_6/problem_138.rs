use std::collections::BTreeSet;

pub fn solve() -> i128 {
    let mut values = BTreeSet::new();

    for sum in 1.. {
        //m > n => sum - n > n => n < sum / 2
        for n in 1..=sum / 2 {
            let m = sum - n;

            if m % 2 == 1 && n % 2 == 1 {
                continue;
            }

            if gcd::binary_u32(n as u32, m as u32) != 1 {
                continue;
            }

            if i32::abs(m * m - n * n - 4 * m * n) != 1 {
                continue;
            }

            let length = m * m + n * n;

            if values.len() == 12 && length > *values.last().unwrap() {
                break;
            }

            values.insert(length);

            if values.len() > 12 {
                values.pop_last();
            }
        }
    }

    values.iter().sum::<i32>() as i128
}
