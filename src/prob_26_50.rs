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

    fn prime_number_sieve(limit: usize) -> HashSet<i64> {
        let mut primes: HashSet<i64> = HashSet::new();

        let mut sieve: Vec<bool> = vec![true; limit + 1];

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

pub fn main() {
    println!("Problem 26: {}", prob_26::solve(1000));
    println!("Problem 27: {}", prob_27::solve());
    println!("Problem 28: {}", prob_28::solve(1001));
    println!("Problem 29: {}", prob_29::solve());
    println!("Problem 30: {}", prob_30::solve());
}
