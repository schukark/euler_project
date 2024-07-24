use crate::utils::primes::get_prime_exponents;

#[test]
pub fn test_prime_count() {
    assert_eq!(get_prime_exponents(14).len(), 2);
    assert_eq!(get_prime_exponents(15).len(), 2);
    assert_eq!(get_prime_exponents(644).len(), 3);
    assert_eq!(get_prime_exponents(645).len(), 3);
    assert_eq!(get_prime_exponents(646).len(), 3);
}

pub fn solve(count: i64) -> i128 {
    let mut cur_start = 2;

    loop {
        let mut is_correct = true;

        for i in 1..=count {
            if get_prime_exponents(cur_start + i - 1).len() as i64 != count {
                cur_start += i;
                is_correct = false;
                break;
            }
        }

        if is_correct {
            return cur_start as i128;
        }
    }
}

#[test]
pub fn solve_2() {
    assert_eq!(solve(2), 14);
}

#[test]
pub fn solve_3() {
    assert_eq!(solve(3), 644);
}
