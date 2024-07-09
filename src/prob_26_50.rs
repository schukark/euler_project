mod prob_26 {
    fn reduce_2_5(number: u64) -> u64 {
        let mut number = number;
        while number % 2 == 0 {
            number /= 2;
        }

        while number % 5 == 0 {
            number /= 5;
        }

        number
    }

    fn find_loop_len(denominator: u64) -> usize {
        let new_num = reduce_2_5(denominator);

        if new_num == 1 {
            return 0;
        }

        let mut numerator = 10;

        let mut cycle = Vec::new();

        let num_len = denominator.to_string().len();

        loop {
            cycle.push(numerator / new_num);
            numerator %= new_num;
            numerator *= 10;

            if cycle.len() > num_len && cycle[..num_len] == cycle[cycle.len() - num_len..] {
                return cycle.len() - num_len;
            }
        }
    }

    #[test]
    pub fn test_6() {
        assert_eq!(find_loop_len(6), 1);
    }

    #[test]
    pub fn test_7() {
        assert_eq!(find_loop_len(7), 6);
    }

    #[test]
    pub fn test_11() {
        assert_eq!(find_loop_len(11), 2);
    }

    pub fn solve(limit: u64) -> u64 {
        let (mut best_cycle, mut best_denom) = (0, 1);
        for i in 1..limit {
            let tmp = find_loop_len(i);

            if tmp > best_cycle {
                best_cycle = tmp;
                best_denom = i;
            }
        }

        best_denom
    }
}

mod prob_27 {
    use std::collections::HashSet;

    pub fn prime_number_sieve(limit: usize) -> HashSet<i64> {
        let mut primes: HashSet<i64> = HashSet::new();

        let mut sieve: Vec<bool> = vec![true; limit + 1];
        sieve[1] = false;

        let mut i = 2;

        while i * i <= limit {
            if !sieve[i] {
                i += 1;
                continue;
            }

            let mut j = i * i;
            while j <= limit {
                sieve[j] = false;
                j += i;
            }
            i += 1;
        }

        sieve.iter().enumerate().for_each(|(index, value)| {
            if !*value {
                return;
            }

            primes.insert(index as i64);
        });

        primes
    }

    fn find_consectuive_amount(a: i64, b: i64, primes: &HashSet<i64>) -> u64 {
        let mut count = 0;
        let mut n = 0;

        loop {
            if !primes.contains(&(n * n + a * n + b)) {
                break;
            }

            count += 1;
            n += 1;
        }

        count
    }

    #[test]
    pub fn test_euler() {
        let primes = prime_number_sieve(100000);
        assert_eq!(find_consectuive_amount(1, 41, &primes), 40);
    }

    pub fn solve() -> i64 {
        let (mut best_prod, mut best_count) = (0, 0);

        let primes = prime_number_sieve(100_000_000);

        for a in -999..1000 {
            for b in -999..1000 {
                let count = find_consectuive_amount(a, b, &primes);
                if count > best_count {
                    best_count = count;
                    best_prod = a * b;
                }
            }
        }

        best_prod
    }
}

mod prob_28 {
    pub fn solve(limit: u64) -> u64 {
        let limit = (limit as i64 + 1) / 2;

        // TR diagonal: (2n-1)^2
        // BR diagonal: 4n^2-10n+7
        // BL diagonal: 4(n-1)^2+1
        // TL diagonal: 4n^2 - 6n + 3
        // TOTAL per n: 16n^2-28n+16

        // sum_{i=1}^{(limit - 1) / 2} 16n^2-20n+12=16/6 N (N+1)(2N+1) - 28/2 N(N+1) + 16 N=
        // 8/3 (2N^3+3N^2+N)-14N^2-14N+16N = 16/3N^3 - 6N^2 + 14/3N

        // Amount = 16/3N^3 - 6N^2 + 14/3N - 3

        (16 * limit * limit * limit - 18 * limit * limit + 14 * limit - 9) as u64 / 3
    }
}

mod prob_29 {
    use std::collections::HashSet;

    fn get_primes(number: u64) -> Vec<(u64, u64)> {
        let mut primes: Vec<(u64, u64)> = Vec::new();

        let mut number = number;
        let mut cur_div = 2;

        while number > 1 {
            let mut count = 0;
            while number % cur_div == 0 {
                number /= cur_div;
                count += 1;
            }

            if count > 0 {
                primes.push((cur_div, count));
            }

            cur_div += 1;
        }

        primes
    }

    fn mult_array(powers: &mut Vec<(u64, u64)>, count: u64) {
        for number in powers {
            number.1 *= count;
        }
    }

    pub fn solve() -> usize {
        let mut power_array = HashSet::new();

        for a in 2..=100 {
            for b in 2..=100 {
                let mut powers = get_primes(a);
                mult_array(&mut powers, b);

                power_array.insert(powers);
            }
        }

        power_array.len()
    }
}

mod prob_30 {
    // if n (number of digits) >= 6
    // sum of powers of digits <= n * 9 ^ 5 < 10^5 * n < 10^n (for n >= 6)

    pub fn solve() -> u64 {
        let mut result = 0;

        for i in 10..1_000_000 {
            let sum_digits: u64 = i
                .to_string()
                .chars()
                .map(|a| a.to_digit(10).unwrap().pow(5) as u64)
                .sum();

            if sum_digits != i as u64 {
                continue;
            }

            result += i as u64;
        }

        result
    }
}

mod prob_31 {
    pub fn solve(amount: u64) -> u64 {
        let mut count = 0;

        for c200 in 0..=amount / 200 {
            for c100 in 0..=(amount - 200 * c200) / 100 {
                for c50 in 0..=(amount - 200 * c200 - 100 * c100) / 50 {
                    for c20 in 0..=(amount - 200 * c200 - 100 * c100 - 50 * c50) / 20 {
                        for c10 in 0..=(amount - 200 * c200 - 100 * c100 - 50 * c50 - 20 * c20) / 10
                        {
                            for c5 in 0..=(amount
                                - 200 * c200
                                - 100 * c100
                                - 50 * c50
                                - 20 * c20
                                - 10 * c10)
                                / 5
                            {
                                let cur_amount = amount
                                    - 200 * c200
                                    - 100 * c100
                                    - 50 * c50
                                    - 20 * c20
                                    - 10 * c10
                                    - 5 * c5;

                                count += cur_amount / 2 + 1;
                            }
                        }
                    }
                }
            }
        }

        count
    }
}

mod prob_32 {
    fn check_pandigital(mult1: u64, mult2: u64, prod: u64) -> bool {
        let mut digits = [0; 10];
        digits[0] = 1;

        mult1
            .to_string()
            .chars()
            .for_each(|f| digits[f.to_digit(10).unwrap() as usize] += 1);

        mult2
            .to_string()
            .chars()
            .for_each(|f| digits[f.to_digit(10).unwrap() as usize] += 1);

        prod.to_string()
            .chars()
            .for_each(|f| digits[f.to_digit(10).unwrap() as usize] += 1);

        digits == [1; 10]
    }

    pub fn solve() -> u64 {
        let mut result = 0;
        for product in 1000..10000 {
            for mult1 in 1..100 {
                if product % mult1 != 0 {
                    continue;
                }

                if check_pandigital(mult1, product / mult1, product) {
                    result += product;
                    break;
                }
            }
        }

        result
    }
}

mod prob_33 {
    fn check_quasi_cancellation(num: u64, denom: u64) -> bool {
        if num % 10 == 0 && denom % 10 == 0 {
            return false;
        }

        if num * (denom / 10) == denom * (num / 10) && (denom / 10) == (num / 10) {
            return true;
        }
        if num * (denom % 10) == denom * (num / 10) && (denom / 10) == (num % 10) {
            return true;
        }
        if num * (denom / 10) == denom * (num % 10) && (denom % 10) == (num / 10) {
            return true;
        }
        if num * (denom % 10) == denom * (num % 10) && (denom % 10) == (num % 10) {
            return true;
        }

        false
    }

    pub fn solve() -> u64 {
        let (mut numerator, mut denominator) = (1, 1);

        for num in 10..100 {
            for denom in num + 1..100 {
                if !check_quasi_cancellation(num, denom) {
                    continue;
                }

                numerator *= num;
                denominator *= denom;
            }
        }
        let binary_u64 = gcd::binary_u64(numerator, denominator);

        denominator / binary_u64
    }
}

mod prob_34 {
    pub fn solve() -> u64 {
        let mut result = 0;

        for number in 3..10_000_000 {
            let factorial_sum: u64 = number
                .to_string()
                .chars()
                .map(|a| (1..=a.to_digit(10).unwrap() as u64).product::<u64>())
                .sum();

            if factorial_sum != number {
                continue;
            }

            result += number;
        }

        result
    }
}

mod prob_35 {
    use std::collections::HashSet;

    use super::prob_27::prime_number_sieve;

    fn get_all_rotations(number: u64) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();

        let number_string = number.to_string();

        for i in 0..number_string.len() {
            result.push(
                (number_string[i..].to_owned() + &number_string[..i])
                    .parse::<u64>()
                    .unwrap(),
            );
        }

        result
    }

    pub fn solve(limit: u64) -> usize {
        let primes = prime_number_sieve(limit as usize);
        let mut answer: HashSet<u64> = HashSet::new();

        for i in 2..limit {
            if answer.contains(&i) || !primes.contains(&(i as i64)) {
                continue;
            }

            if get_all_rotations(i)
                .iter()
                .any(|f| !primes.contains(&(*f as i64)))
            {
                continue;
            }

            get_all_rotations(i).iter().for_each(|f| {
                answer.insert(*f);
            });
        }

        answer.len()
    }
}

mod prob_36 {
    fn is_palindromic(number: u64) -> bool {
        number
            .to_string()
            .chars()
            .eq(number.to_string().chars().rev())
    }

    fn is_palindromic_binary(number: u64) -> bool {
        format!("{number:b}").eq(&format!("{number:b}").chars().rev().collect::<String>())
    }

    pub fn solve() -> u64 {
        let mut result = 0;

        for number in 1..1_000_000 {
            if !is_palindromic(number) || !is_palindromic_binary(number) {
                continue;
            }

            if number % 2 == 0 && number % 10 == 0 {
                continue;
            }

            result += number;
        }

        result
    }
}

mod prob_37 {
    use std::collections::HashSet;

    use super::prob_27::prime_number_sieve;

    fn check_left_truncatable(number: i64, primes: &HashSet<i64>) -> bool {
        let number_string = number.to_string();

        for i in 0..number_string.len() {
            if !primes.contains(&(number_string[i..]).parse::<i64>().unwrap()) {
                return false;
            }
        }

        true
    }

    fn check_right_truncatable(number: i64, primes: &HashSet<i64>) -> bool {
        let number_string = number.to_string();

        for i in 1..=number_string.len() {
            if !primes.contains(&(number_string[..i]).parse::<i64>().unwrap()) {
                return false;
            }
        }

        true
    }

    pub fn solve() -> i64 {
        let mut result = 0;
        let primes = prime_number_sieve(1_000_000);

        for i in 10..1_000_000 {
            if !primes.contains(&i) {
                continue;
            }

            if !check_left_truncatable(i, &primes) || !check_right_truncatable(i, &primes) {
                continue;
            }

            result += i;
        }

        result
    }
}

mod prob_38 {
    fn check_pandigital(number: u64) -> bool {
        let mut digits = [0; 10];
        digits[0] = 1;

        number
            .to_string()
            .chars()
            .for_each(|symbol| digits[symbol.to_digit(10).unwrap() as usize] += 1);

        digits == [1; 10]
    }

    pub fn solve() -> u64 {
        let mut result = 0;

        for number in 9..1_000_000 {
            let mut cur_string = number.to_string();
            for n in 2..=9 {
                cur_string += &(number * n).to_string();

                if cur_string.len() > 9 {
                    break;
                }

                if !check_pandigital(cur_string.parse::<u64>().unwrap()) {
                    continue;
                }

                result = u64::max(result, cur_string.parse::<u64>().unwrap());
                break;
            }
        }

        result
    }
}

mod prob_39 {
    fn count_solutions(perimeter: u64) -> u64 {
        let mut count = 0;

        for i in 1..perimeter {
            for j in 1..perimeter - i {
                let k = perimeter - i - j;

                if i * i + j * j != k * k {
                    continue;
                }

                count += 1;
            }
        }

        count
    }

    pub fn solve() -> u64 {
        let (mut best_perimeter, mut best_count) = (0, 0);
        for i in 1..=1000 {
            let tmp = count_solutions(i);

            if tmp > best_count {
                best_count = tmp;
                best_perimeter = i;
            }
        }

        best_perimeter
    }
}

mod prob_40 {
    pub fn solve() -> u128 {
        let mut cur = 1;
        let mut cur_string = String::new();

        while cur_string.len() < 1_000_000 {
            cur_string += &cur.to_string();
            cur += 1;
        }

        let mut result = 1;

        assert_eq!(cur_string.chars().nth(11).unwrap(), '1');

        for i in [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000] {
            result *= cur_string
                .chars()
                .nth(i as usize - 1)
                .unwrap()
                .to_digit(10)
                .unwrap() as u128;
        }

        result
    }
}

mod prob_41 {
    use super::prob_27::prime_number_sieve;

    fn check_pandigital(number: i64, n: usize) -> bool {
        let mut digits = vec![0; n + 1];
        digits[0] = 1;

        if number
            .to_string()
            .chars()
            .any(|f| f.to_digit(10).unwrap() as usize > n)
        {
            return false;
        }

        number
            .to_string()
            .chars()
            .for_each(|a| digits[a.to_digit(10).unwrap() as usize] += 1);

        digits == vec![1; n + 1]
    }

    pub fn solve() -> i64 {
        assert!(check_pandigital(2143, 4));

        let mut result = 0;

        // sum of 1 + .. + 9 = 45, which is divisible by 9 => not a prime
        // sum of 1 + .. + 8 = 36 which is divisible by 9 => not a prime
        // the search space is now up to 10 ^ 7 - 1
        let primes = prime_number_sieve(10_000_000);

        for i in 2..10_000_000 {
            if !primes.contains(&i) {
                continue;
            }

            let n = i.to_string().len();

            if !check_pandigital(i, n) {
                continue;
            }

            result = i;
        }

        result
    }
}

mod prob_42 {
    use std::fs;

    fn read_from_file() -> Vec<String> {
        fs::read_to_string("src/txts/prob_42.txt")
            .unwrap()
            .split(',')
            .map(|a| a.trim_end_matches('\"').trim_start_matches('\"').to_owned())
            .collect::<Vec<String>>()
    }

    fn is_triangular(number: u64) -> bool {
        // n(n + 1) / 2 = k
        // n ^ 2 + n - 2k = 0
        // n = (-1 + sqrt(1 + 8k)) / 2

        let n = (f64::sqrt(1.0_f64 + 8.0_f64 * number as f64) - 1.0_f64) / 2.0_f64;
        f64::abs(n - n.floor()) < 1e-5
    }

    fn calculate_score(word: &str) -> u64 {
        word.chars().map(|a| (a as u8 - b'A' + 1) as u64).sum()
    }

    pub fn solve() -> u64 {
        assert_eq!(calculate_score("SKY"), 55);
        assert!(is_triangular(calculate_score("SKY")));

        let mut count = 0;

        let words = read_from_file();

        for word in &words {
            let score = calculate_score(word);

            if is_triangular(score) {
                count += 1;
            }
        }

        count
    }
}

mod prob_43 {
    fn next_permutation(nums: &mut [i32]) -> bool {
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

    pub fn solve() -> u64 {
        let mut count = 0;

        let mut digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let divisors = [1, 2, 3, 5, 7, 11, 13, 17];

        while next_permutation(&mut digits) {
            if digits[0] == 0 {
                continue;
            }

            if digits.iter().enumerate().all(|(index, _value)| {
                index >= 8
                    || (digits[index] * 100 + digits[index + 1] * 10 + digits[index + 2])
                        % divisors[index]
                        == 0
            }) {
                count += digits
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join("")
                    .parse::<u64>()
                    .unwrap();
            }
        }

        count
    }
}

mod prob_44 {
    fn is_pentagonal(number: u64) -> bool {
        // k = n(3n - 1) / 2 => 3n^2 - n - 2k = 0
        // n = (1 +- sqrt(1 + 24k)) / 2

        let n = (f64::sqrt(1.0_f64 + 24.0_f64 * number as f64) + 1.0_f64) / 6.0_f64;
        f64::abs(n - n.round()) < 1e-5
    }

    pub fn solve() -> u64 {
        assert!([1, 5, 12, 22, 35, 51, 70, 92, 117, 145]
            .iter()
            .all(|a| is_pentagonal(*a)));

        let mut result = u64::MAX;

        for n in 1..10_000 {
            for m in n + 1..10_000 {
                let first = n * (3 * n - 1) / 2;
                let second = m * (3 * m - 1) / 2;

                if is_pentagonal(first + second) && is_pentagonal(second - first) {
                    result = u64::min(result, second - first);
                }
            }
        }

        result
    }
}

mod prob_45 {
    fn is_triangular(number: u64) -> bool {
        // k = n(n + 1) / 2 => n^2 + n - 2k = 0
        // n = (-1 +- sqrt(1 + 8k)) / 2

        let n = (f64::sqrt(1.0_f64 + 8.0_f64 * number as f64) - 1.0_f64) / 2.0_f64;
        f64::abs(n - n.round()) < 1e-5
    }

    fn is_pentagonal(number: u64) -> bool {
        // k = n(3n - 1) / 2 => 3n^2 - n - 2k = 0
        // n = (1 +- sqrt(1 + 24k)) / 6

        let n = (f64::sqrt(1.0_f64 + 24.0_f64 * number as f64) + 1.0_f64) / 6.0_f64;
        f64::abs(n - n.round()) < 1e-5
    }

    pub fn solve() -> u64 {
        let mut cur_hex = 144;

        loop {
            let cur_num = cur_hex * (2 * cur_hex - 1);

            if is_triangular(cur_num) && is_pentagonal(cur_num) {
                return cur_num;
            }
            cur_hex += 1;
        }
    }
}

pub fn main() {
    println!("Problem 26: {}", prob_26::solve(1000));
    println!("Problem 27: {}", prob_27::solve());
    println!("Problem 28: {}", prob_28::solve(1001));
    println!("Problem 29: {}", prob_29::solve());
    println!("Problem 30: {}", prob_30::solve());
    println!("Problem 31: {}", prob_31::solve(200));
    println!("Problem 32: {}", prob_32::solve());
    println!("Problem 33: {}", prob_33::solve());
    println!("Problem 34: {}", prob_34::solve());
    println!("Problem 35: {}", prob_35::solve(1_000_000));
    println!("Problem 36: {}", prob_36::solve());
    println!("Problem 37: {}", prob_37::solve());
    println!("Problem 38: {}", prob_38::solve());
    println!("Problem 39: {}", prob_39::solve());
    println!("Problem 40: {}", prob_40::solve());
    println!("Problem 41: {}", prob_41::solve());
    println!("Problem 42: {}", prob_42::solve());
    println!("Problem 43: {}", prob_43::solve());
    println!("Problem 44: {}", prob_44::solve());
    println!("Problem 45: {}", prob_45::solve());
}
