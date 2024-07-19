use crate::utils::math::sum_divisors;

fn check_abundant(number: u64) -> bool {
    sum_divisors(number) > number
}

#[test]
pub fn abudnant_1() {
    assert!(check_abundant(12));
}

#[test]
pub fn abudnant_2() {
    assert!(!check_abundant(6));
}

pub fn solve() -> u64 {
    let limit = 28123;
    let mut result = 0;

    for i in 1..=limit {
        let mut can_sum_to = false;
        for j in 1..i {
            if check_abundant(j) && check_abundant(i - j) {
                can_sum_to = true;
                break;
            }
        }

        if !can_sum_to {
            result += i;
        }
    }

    result
}
