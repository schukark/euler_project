mod prob_21 {
    pub fn sum_divisors(number: u64) -> u64 {
        let mut sum = 1;
        let mut i = 2;

        while i * i <= number {
            if number % i != 0 {
                i += 1;
                continue;
            }

            sum += i + number / i;

            if i * i == number {
                sum -= i;
            }

            i += 1;
        }

        sum
    }

    pub fn solve(limit: u64) -> u64 {
        let mut result = 0;

        for i in 2..limit {
            let sum_div = sum_divisors(i);
            if sum_div == i {
                continue;
            }

            if sum_divisors(sum_div) == i {
                result += i;
            }
        }

        result
    }
}

mod prob_22 {
    use std::fs;

    fn sort_names() -> Vec<String> {
        let mut collect = fs::read_to_string("src/txts/prob_22.txt")
            .unwrap()
            .split(',')
            .map(|a| a.trim_end_matches('\"').trim_start_matches('\"').to_owned())
            .collect::<Vec<String>>();
        collect.sort();
        collect
    }

    fn calculate_score(name: &str) -> u64 {
        name.chars().map(|a| (a as u8 - b'A') as u64 + 1).sum()
    }

    pub fn solve() -> u64 {
        sort_names()
            .iter()
            .enumerate()
            .map(|(a, b)| calculate_score(b) * ((a + 1) as u64))
            .sum()
    }

    #[test]
    pub fn test_score() {
        assert_eq!(calculate_score("COLIN"), 53);
    }

    #[test]
    pub fn test_sort() {
        assert_eq!(sort_names()[937], "COLIN");
    }
}

mod prob_23 {
    use super::prob_21::sum_divisors;

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
}

mod prob_24 {
    pub fn next_permutation(nums: &mut [i32]) -> bool {
        use std::cmp::Ordering;
        // or use feature(array_windows) on nightly
        let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
            Some(i) => i,
            None => {
                nums.reverse();
                return false;
            }
        };

        let swap_with = nums[last_ascending + 1..]
            .binary_search_by(|n| i32::cmp(&nums[last_ascending], n).then(Ordering::Less))
            .unwrap_err(); // cannot fail because the binary search will never succeed
        nums.swap(last_ascending, last_ascending + swap_with);
        nums[last_ascending + 1..].reverse();
        true
    }

    pub fn solve() -> String {
        let mut nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for _i in 1..1_000_000 {
            next_permutation(&mut nums);
        }

        nums.map(|a| a.to_string()).join("")
    }
}

mod prob_25 {
    use num_bigint::BigUint;

    pub fn solve() -> u64 {
        let mut result = 2;

        let (mut a, mut b) = (BigUint::new(vec![1]), BigUint::new(vec![1]));

        while b.to_string().len() < 1000 {
            (a, b) = (b.clone(), a + b);
            result += 1;
        }

        result
    }
}

pub fn main() {
    println!("Problem 21: {}", prob_21::solve(10000));
    println!("Problem 22: {}", prob_22::solve());
    println!("Problem 23: {}", prob_23::solve());
    println!("Problem 24: {}", prob_24::solve());
    println!("Problem 25: {}", prob_25::solve());
}
