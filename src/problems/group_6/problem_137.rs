use std::collections::BTreeSet;

/// # Solution
/// Generating function for the fibonacci numbers (considering that first numbers are 1, 1 and not 0, 1) is:
///
/// f(x) = x / (1 - x - x ^ 2)
///
/// f(x) = n => n - nx - nx ^ 2 = x => nx ^ 2 + (n + 1) * x - n = 0
///
/// D = (n + 1) ^ 2 + 4n ^ 2 = 5n ^ 2 + 2n + 1
///
/// x is rational only if the discriminant is a perfect square of a rational number (and a whole number as well, because n is an integer)
///
/// 5n ^ 2 + 2 * n + 1 = k ^ 2
///
/// (n + 1) ^ 2 + (2n) ^ 2 = k ^ 2
///
/// Thus, (n + 1, 2n, k) should be a Pythagorean triple
///
/// ## First case
/// n + 1 = (p ^ 2 - q ^ 2) * t, 2n = 2 * p * q * t, k = (p ^ 2 + q ^ 2) * t
///
/// 1 = (n + 1) - n = t * (p ^ 2 - q ^ 2) - p * q * t = t * (p ^ 2 - pq - q ^ 2)
///
/// Considering p, q, t are natural numbers, then t = 1, p ^ 2 - pq - q ^ 2 = 1
///
/// p ^ 2 - p * q - q ^ 2 = 1
///
/// Using the results from [https://www.quora.com/How-do-I-solve-this-equation-x-2+xy-y-2-1]
///
/// The solutions are in the form (F_{2k - 1}, F_{2k}), k >= 1
///
/// ## Second case
/// n + 1 = t * 2 * p * q, 2n = (p ^ 2 - q ^ 2) * t
///
/// 2 = t * (4pq - p ^ 2 + q ^ 2) =>
///
/// ### Case a
/// t = 1
///
/// p ^ 2 - 4pq - q ^ 2 = -2 => (p - 2q) ^ 2 - 5q^2 = -2 - when taken mod 5, it is easy to see, there are no solutions
///
/// ### Case b
/// t = 2
/// p ^ 2 - 4pq - q ^ 2 = -1 => (p - 2q) ^ 2 - 5q^2 = -1 - tied to Pell's equation

const LIMIT: i32 = 15;

pub fn solve() -> i128 {
    let mut answer: BTreeSet<i128> = BTreeSet::new();

    //case 1
    let (mut a, mut b) = (1, 2);

    for _ in 0..LIMIT {
        answer.insert(a * b);

        (a, b) = (b, a + b);
        (a, b) = (b, a + b);
    }

    //case 2
    let (mut a, mut b) = (2, 1);

    for _ in 0..LIMIT {
        // p - 2q = a, q = b
        let q = b;
        let p = 2 * q + a;

        let n = 4 * p * q - 1;
        answer.insert(n);

        //(a, b) = (a * 4 + 5 * a * 1 + 2 * 5 * b * 2, b * 4 + 5 * b * 1 + 2 * a * 2 * 1);
        (a, b) = (9 * a + 20 * b, 9 * b + 4 * a);
    }

    // println!("{:#?}", answer);

    *answer.iter().nth(LIMIT as usize - 1).unwrap()
}
