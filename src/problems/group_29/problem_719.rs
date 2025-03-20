use std::thread;

/// Checks if the number is an S number
/// <br>
/// An S number is a number that can be broken down into several parts which sum to the square root of the number itself
/// <br>
/// Example: sqrt(81) = 8 + 1, sqrt(8281) = 82 + 8 + 1
fn s_check(number: i128) -> bool {
    s_check_(number, number, &[])
}

fn s_check_(number: i128, target: i128, cur_vec: &[i128]) -> bool {
    if number == 0 {
        if cur_vec.len() < 2 {
            return false;
        }
        let sum = cur_vec.iter().sum::<i128>();
        sum * sum == target
    } else {
        let mut mask = 10;

        while mask <= 10 * number {
            let mut new_vec = cur_vec.to_owned();
            new_vec.push(number % mask);
            if s_check_(number / mask, target, &new_vec) {
                return true;
            }
            mask *= 10;
        }

        false
    }
}

const THREAD_COUNT: i128 = 18;

fn t_func(limit: i128) -> i128 {
    let mut results = Vec::new();

    for thread in 1..=THREAD_COUNT {
        results.push(thread::spawn(move || {
            (thread..=limit.isqrt())
                .step_by(THREAD_COUNT as usize)
                .filter(|x| s_check(x * x))
                .map(|x| x * x)
                .sum::<i128>()
        }));
    }

    results
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum::<i128>()
}

pub fn solve() -> i128 {
    t_func(10i128.pow(12))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s_check81() {
        assert!(s_check(81));
    }

    #[test]
    fn test_s_check8281() {
        assert!(s_check(8281));
    }

    #[test]
    fn test_s_check6724() {
        assert!(s_check(6724));
    }

    #[test]
    fn test_s_check9801() {
        assert!(s_check(9801));
    }

    #[test]
    fn test_t_func_1() {
        assert_eq!(41333, t_func(10i128.pow(4)));
    }
}
