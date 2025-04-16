const LOWEST: i128 = 1020304050607080900i128.isqrt();
const HIGHEST: i128 = 1929394959697989990i128.isqrt();

pub(crate) fn solve() -> i128 {
    for i in LOWEST..=HIGHEST {
        if !check_form(i * i) {
            continue;
        }

        return i;
    }

    i128::MAX
}

fn check_form(num: i128) -> bool {
    let str_num = num.to_string().chars().collect::<Vec<_>>();

    str_num
        .iter()
        .enumerate()
        .filter(|(idx, _x)| idx % 2 == 0)
        .all(|(idx, x)| idx / 2 + 1 == x.to_digit(10).unwrap() as usize || idx == 18 && *x == '0')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_form() {
        assert!(check_form(1122334455667788990));
    }
}
