/// # Solution
/// Let a, b, c be the sides of the right triangle:
/// It is known that all pythagorean triples could be represented in the following ways:
///
///     a = k * (m ^ 2 - n ^ 2)
///     b = k * 2 * m * n
///     c = k * (m ^ 2 + n ^ 2)
///
/// Where (m, n) = 1, m and n can't be both odd
///
/// The area of the middle square in the first picture is:
///
///     S = c ^ 2 - 2 * a * b
///     l = sqrt(S) = sqrt(c ^ 2 - 2 * a * b)
///
/// S = k ^ 2 * ((m ^ 2 + n ^ 2) ^ 2 - 2 * 2 * m * n * (m ^ 2 - n ^ 2)) =
///
/// = k ^ 2 * (m ^ 2 - 2 * m * n - n ^ 2) ^ 2
///
/// That means that the length of the side equals k * |m ^ 2 - 2 * m * n - n ^ 2 |
///
/// # Implementation
///
/// Generate n indefinitely, until the perimeter of the smallest triangle with the current n exceeds the limit
///
/// it is doesn't matter which k is chosen, if k = 1 is a solution, then all k are solutions, then it is sufficient
/// to just add floor(limit / perimeter) to the result
pub fn solve() -> i128 {
    const LIMIT: i32 = 10_i32.pow(8);
    let mut result = 0;

    for n in 1.. {
        if 2 * (n + 1) * (2 * n + 1) >= LIMIT {
            break;
        }

        for m in n + 1.. {
            if n % 2 == 1 && m % 2 == 1 {
                continue;
            }

            let perimeter = 2 * m * (m + n);

            if perimeter >= LIMIT {
                break;
            }

            if gcd::binary_u32(m as u32, n as u32) != 1 {
                continue;
            }

            if (m * m + n * n) % i32::abs(m * m - 2 * m * n - n * n) != 0 {
                continue;
            }

            println!(
                "a: {}, b: {}, c: {}",
                m.pow(2) - n.pow(2),
                2 * m * n,
                m.pow(2) + n.pow(2)
            );

            result += LIMIT / perimeter;
        }
    }

    result as i128
}
