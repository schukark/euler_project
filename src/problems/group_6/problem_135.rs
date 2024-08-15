/// # Solution
///
/// x = y + k, z = y - k
///
/// y^2 - 4ky + n = 0
///
/// D/4 = 4k^2 - n = q^2
///
/// y = 2k +- sqrt(D/4)
///
/// (x, y, z) = (3k, 2k, k) +- sqrt(D/4) * (1, 1, 1)
///
/// n = (2k - q) * (2k + q)
///
/// 2k - q = a, 2k + q = n / a
use std::collections::HashSet;

fn count_number_of_solutions(number: i64) -> i64 {
    let mut solutions = HashSet::new();

    let mut current: i64 = 1;
    while current.pow(2) <= number {
        if number % current != 0 {
            current += 1;
            continue;
        }

        if (number / current - current) % 2 != 0 || (number / current + current) % 4 != 0 {
            current += 1;
            continue;
        }

        let (k, q) = (
            (current + number / current) / 4,
            (number / current - current) / 2,
        );

        if k > q {
            solutions.insert((3 * k - q, 2 * k - q, k - q));
        }

        solutions.insert((3 * k + q, 2 * k + q, k + q));
        current += 1;
    }

    solutions.len() as i64
}

pub fn solve() -> i128 {
    (1..1_000_000)
        .filter(|&x| count_number_of_solutions(x) == 10)
        .count() as i128
}

#[cfg(test)]
mod tests {
    use crate::problems::group_6::problem_135::count_number_of_solutions;

    #[test]
    fn test_count_27() {
        assert_eq!(2, count_number_of_solutions(27));
    }

    #[test]
    fn test_count_1155() {
        assert_eq!(10, count_number_of_solutions(1155));
    }
}
