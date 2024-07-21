//! FIX THE MATH, PROBABLY INCORRECT ASSUMPTIONS ABOUT WHOLENESS OF SOME NUMBERS

pub fn solve() -> i128 {
    const LIMIT: i64 = 1_000_000_000;
    let mut result = 0;

    // A = (m^2 + n^2) * k;
    // B = 2 * (m^2 - n^2) * k;
    // H = 2mn * k
    // P = 4km^2 <= 10^9 => m <= sqrt(10^9/4)=10^4 * sqrt(10)/2

    let maximum: i64 = (LIMIT as f64 / 4.0_f64).sqrt() as i64 + 1;

    for n in 1..=maximum {
        for m in n + 1..=maximum {
            let perimeter = (4 * m * m) as i128;

            if perimeter > LIMIT as i128 {
                break;
            }

            // a = b + 1 => (m^2 + n^2)k = 2(m^2 - n^2)k + 1 =>
            // => k * (3n^2 - m^2) = 1 => k = 1 or 3m^2 - n^2 = 1

            // a + 1 = b => k(m^2 + n^2) + 1 = 2k(m^2 - n^2) =>
            // => k(m^2 - 3n^2) = 1 => k = 1 or m^2 - 3n^2 = 1

            if 3 * m * m - n * n == 1 || m * m - 3 * n * n == 1 {
                result +=
                    perimeter * (LIMIT as i128 / perimeter) * (LIMIT as i128 / perimeter + 1) / 2;
            } else {
                result += perimeter;
            }
        }
    }

    result
}
