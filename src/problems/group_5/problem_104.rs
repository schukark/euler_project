use num_bigint::BigUint;

fn check_digits(digits: &str) -> bool {
    digits
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .fold([0; 10], |mut acc, f| {
            if acc[0] == 0 {
                acc[0] = 1;
            }

            acc[f] += 1;
            acc
        })
        == [1; 10]
}

#[test]
fn test_check_digits() {
    assert!(check_digits("123456789"));
}

#[test]
fn test_check_digits2() {
    assert!(!check_digits("0123456789"));
}

fn check_pandigital(number: &BigUint) -> bool {
    check_digits(&number.to_string()[..9])
        && check_digits(&number.to_string()[(number.to_string().len() - 9)..])
}

pub fn solve() -> i128 {
    let (mut a, mut b): (BigUint, BigUint) = (1_u32.into(), 1_u32.into());
    let mut k = 2;

    loop {
        if b.to_string().len() >= 10 && check_pandigital(&b) {
            return k;
        }

        (a, b) = (b.clone(), a + &b);
        k += 1;
    }
}
