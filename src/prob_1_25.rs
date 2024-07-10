mod prob_1 {
    pub fn solve(limit: u32) -> u64 {
        let mut result: u64 = 0;
        for i in 1..limit {
            if i % 5 != 0 && i % 3 != 0 {
                continue;
            }

            result += i as u64;
        }

        result
    }
}

mod prob_2 {
    pub fn solve() -> u64 {
        let (mut a, mut b): (u64, u64) = (1, 1);

        let mut result: u64 = 0;

        while b < 4_000_000 {
            if b % 2 == 0 {
                result += b;
            }

            (a, b) = (b, a + b);
        }

        result
    }
}

mod prob_3 {
    pub fn solve(number: u64) -> u64 {
        let mut current_div: u64 = 2;
        let mut result: u64 = 0;
        let mut number = number;

        while number > 1 {
            while number % current_div == 0 {
                number /= current_div;
                result = current_div;
            }

            current_div += 1;
        }

        result
    }
}

mod prob_4 {
    pub fn solve(digits: u32) -> u64 {
        let min_number = u64::pow(10, digits - 1);
        let max_number = min_number * 10 - 1;

        let mut result = 0;

        for i in min_number..=max_number {
            for j in min_number..=max_number {
                let product = i * j;

                let stringified = product.to_string();

                if !stringified.chars().eq(stringified.chars().rev()) {
                    continue;
                }

                result = u64::max(result, product);
            }
        }

        result
    }
}

mod prob_5 {
    fn gcd(u: u64, v: u64) -> u64 {
        if v == 0 {
            u
        } else if u == 0 {
            v
        } else if u % 2 == 0 && v % 2 == 0 {
            2 * gcd(u / 2, v / 2)
        } else if v % 2 == 0 {
            gcd(u, v / 2)
        } else if u % 2 == 0 {
            gcd(u / 2, v)
        } else if v >= u {
            gcd(u, v - u)
        } else {
            gcd(v, u)
        }
    }

    fn lcm(u: u64, v: u64) -> u64 {
        if u == 0 || v == 0 {
            return gcd(u, v);
        }
        u * v / gcd(u, v)
    }

    pub fn solve(limit: u32) -> u64 {
        let mut result: u64 = 0;

        for i in 1..=limit {
            result = lcm(result, i as u64);
        }

        result
    }
}

mod prob_6 {
    pub fn solve(limit: u64) -> u64 {
        limit * limit * (limit + 1) * (limit + 1) / 4 - limit * (limit + 1) * (2 * limit + 1) / 6
    }
}

mod prob_7 {
    pub fn check_primes(number: u64, primes: &mut Vec<u64>) -> bool {
        let mut cur_ind = 0;

        while cur_ind < primes.len() && primes[cur_ind] * primes[cur_ind] <= number {
            if number % primes[cur_ind] == 0 {
                return false;
            }
            cur_ind += 1;
        }

        primes.push(number);
        true
    }

    pub fn solve(limit: u64) -> u64 {
        let mut count = 0;
        let mut current_num = 2;

        let mut primes: Vec<u64> = Vec::new();

        while count < limit {
            if check_primes(current_num, &mut primes) {
                count += 1;
            }
            current_num += 1;
        }

        current_num - 1
    }
}

mod prob_8 {
    pub fn solve() -> u64 {
        let mut number = "73167176531330624919225119674426574742355349194934".to_owned();
        number += "96983520312774506326239578318016984801869478851843";
        number += "85861560789112949495459501737958331952853208805511";
        number += "12540698747158523863050715693290963295227443043557";
        number += "66896648950445244523161731856403098711121722383113";
        number += "62229893423380308135336276614282806444486645238749";
        number += "30358907296290491560440772390713810515859307960866";
        number += "70172427121883998797908792274921901699720888093776";
        number += "65727333001053367881220235421809751254540594752243";
        number += "52584907711670556013604839586446706324415722155397";
        number += "53697817977846174064955149290862569321978468622482";
        number += "83972241375657056057490261407972968652414535100474";
        number += "82166370484403199890008895243450658541227588666881";
        number += "16427171479924442928230863465674813919123162824586";
        number += "17866458359124566529476545682848912883142607690042";
        number += "24219022671055626321111109370544217506941658960408";
        number += "07198403850962455444362981230987879927244284909188";
        number += "84580156166097919133875499200524063689912560717606";
        number += "05886116467109405077541002256983155200055935729725";
        number += "71636269561882670428252483600823257530420752963450";

        let prod_len = 13;
        let mut max_prod = u64::MIN;

        for i in 0..number.len() - prod_len {
            let mut product: u64 = 1;

            for j in i..i + prod_len {
                product *= number.chars().nth(j).unwrap().to_digit(10).unwrap() as u64;
            }

            if product > max_prod {
                max_prod = product;
            }
        }

        max_prod
    }
}

mod prob_9 {
    pub fn solve(sum: u64) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();

        for i in 1..sum {
            for j in i..sum - i {
                let k = sum - i - j;

                if i * i + j * j == k * k {
                    result.push(i * j * k);
                }
            }
        }

        result
    }
}

mod prob_10 {
    use super::prob_7::check_primes;

    pub fn solve(limit: u64) -> u64 {
        let mut primes: Vec<u64> = Vec::new();
        let mut sum: u64 = 0;

        for i in 2..limit {
            if check_primes(i, &mut primes) {
                sum += i;
            }
        }

        sum
    }
}

mod prob_11 {
    pub fn solve() -> u64 {
        let grid: Vec<i32> = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48"
            .replace('\n', " ")
            .split(' ')
            .map(|a| -> i32 {
                a.chars().next().unwrap().to_digit(10).unwrap() as i32 * 10
                    + a.chars().nth(1).unwrap().to_digit(10).unwrap() as i32
            })
            .collect::<Vec<i32>>();

        assert_eq!(grid.len(), 400);

        let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (1, 1), (1, -1)];
        let mut max_prod = 1;

        for direction in directions {
            for start_x in 0..20 {
                for start_y in 0..20 {
                    if start_x + 4 * direction.0 < 0
                        || start_x + 4 * direction.0 >= 20
                        || start_y + 4 * direction.1 < 0
                        || start_y + 4 * direction.1 >= 20
                    {
                        continue;
                    }

                    let mut product: i32 = 1;

                    for i in 0..4 {
                        product *= grid[(20 * (start_x + i * direction.0)
                            + start_y
                            + i * direction.1) as usize];
                    }

                    if max_prod >= product as u64 {
                        continue;
                    }
                    max_prod = product as u64;
                }
            }
        }

        max_prod
    }
}

mod prob_12 {
    pub fn count_div(number: u64) -> u64 {
        let mut div_count = 1;
        let mut cur_div = 2;

        let mut number = number;

        while number > 1 {
            let mut cur_count = 0;
            while number % cur_div == 0 {
                cur_count += 1;
                number /= cur_div;
            }

            div_count *= 1 + cur_count;
            cur_div += 1;
        }

        div_count
    }

    pub fn solve(limit: u64) -> u64 {
        let mut sum = 1;
        let mut adder = 1;

        loop {
            if count_div(sum) > limit {
                return sum;
            }

            adder += 1;
            sum += adder;
        }
    }
}

mod prob_13 {
    use std::fs;

    use num_bigint::BigUint;

    pub fn solve() -> String {
        let read_to_string = fs::read_to_string("src/txts/prob_13.txt")
            .unwrap()
            .split('\n')
            .map(|f| f.to_owned().parse::<BigUint>().unwrap())
            .collect::<Vec<BigUint>>();

        let mut result = read_to_string[0].clone();

        (1..read_to_string.len()).for_each(|i| {
            result = &result + &read_to_string[i];
        });

        result.to_string()[..10].to_owned()
    }
}

mod prob_14 {
    use std::collections::HashMap;

    fn collatz(number: u64, lengths: &mut HashMap<u64, u64>) -> u64 {
        let mut cur_num = number;
        let mut count = 0;

        while cur_num != 1 {
            if lengths.get(&cur_num).is_some() {
                let map_entry = *lengths.get_mut(&cur_num).unwrap();
                lengths.insert(number, map_entry + count);
                return map_entry + count;
            }

            if cur_num % 2 == 0 {
                cur_num /= 2;
            } else {
                cur_num = 3 * cur_num + 1;
            }

            count += 1;
        }

        count
    }

    pub fn solve(limit: u64) -> u64 {
        let mut max_num = 0;
        let mut max_count = 0;

        let mut map: HashMap<u64, u64> = HashMap::new();

        for i in 1..=limit {
            let tmp = collatz(i, &mut map);

            if tmp > max_count {
                max_count = tmp;
                max_num = i;
            }
        }

        max_num
    }
}

mod prob_15 {
    use std::collections::BTreeMap;

    fn choose(n: u64, k: u64, map: &mut BTreeMap<(u64, u64), u64>) -> u64 {
        if let Some(get) = map.get(&(n, k)) {
            return *get;
        }

        if k == 0 || n == k {
            return 1;
        } else if n < k {
            return 0;
        }

        let result = choose(n - 1, k, map) + choose(n - 1, k - 1, map);
        map.insert((n, k), result);
        result
    }

    pub fn solve(width: u64, height: u64) -> u64 {
        let mut map = BTreeMap::new();
        choose(width + height, height, &mut map)
    }
}

mod prob_16 {
    use num_bigint::BigUint;

    fn digit_sum(s: &str) -> u64 {
        s.chars().map(|a| a.to_digit(10).unwrap() as u64).sum()
    }

    pub fn solve(limit: u64) -> u64 {
        assert_eq!(digit_sum("1024"), 7);

        let mut result = BigUint::new(vec![1]);

        for _i in 0..limit {
            result = &result + &result;
        }

        digit_sum(&result.to_string())
    }
}

mod prob_17 {
    fn count_letters(number: u64) -> usize {
        assert!(number <= 1000);

        if number == 1000 {
            return "onethousand".len();
        }

        let letters = [0, 3, 3, 5, 4, 4, 3, 5, 5, 4];
        let letters_tens = [0, 3, 6, 6, 5, 5, 5, 7, 6, 6];

        let mut result = 0;

        if number >= 100 {
            result += letters[(number / 100) as usize];
            result += "hundred".len();

            if number % 100 != 0 {
                result += "and".len();
            } else {
                return result;
            }
        }

        let number = number % 100;

        if !(11..20).contains(&number) {
            result += letters_tens[(number / 10) as usize] + letters[(number % 10) as usize];
        } else {
            result += match number {
                0..=10 => 0,
                11 => "eleven".len(),
                12 => "twelve".len(),
                13 => "thirteen".len(),
                14 => "fourteen".len(),
                15 => "fifteen".len(),
                16 => "sixteen".len(),
                17 => "seventeen".len(),
                18 => "eighteen".len(),
                19 => "nineteen".len(),
                20..=u64::MAX => 0,
            }
        }

        result
    }

    pub fn solve(limit: u64) -> usize {
        assert_eq!(count_letters(342), 23);
        assert_eq!(count_letters(115), 20);
        assert_eq!(count_letters(105), 17);
        assert_eq!(count_letters(555), "fivehundredandfiftyfive".len());
        assert_eq!(count_letters(25), "twentyfive".len());
        assert_eq!(count_letters(5), "five".len());
        assert_eq!(count_letters(110), "onehundredandten".len());
        assert_eq!(count_letters(900), "ninehundred".len());

        (1..=limit).map(count_letters).sum()
    }
}

mod prob_18 {
    use std::fs;

    pub fn solve() -> u64 {
        let grid: Vec<Vec<u64>> = fs::read_to_string("src/txts/prob_18.txt")
            .unwrap()
            .split('\n')
            .map(|a| {
                a.split(' ')
                    .map(|b| b.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect();

        let mut dp: Vec<Vec<u64>> = vec![Vec::new(); grid.len()];
        for i in 0..dp.len() {
            dp[i] = vec![0; grid[i].len()];
        }

        for i in 0..dp.len() {
            for j in 0..dp[i].len() {
                let mut result = 0;

                if i > 0 && j > 0 {
                    result = dp[i - 1][j - 1];
                }
                if i > 0 && j < dp[i - 1].len() {
                    result = u64::max(result, dp[i - 1][j]);
                }

                dp[i][j] = u64::max(dp[i][j], result + grid[i][j]);
            }
        }

        *dp[dp.len() - 1].iter().max().unwrap()
    }
}

mod prob_19 {
    pub fn solve() -> u64 {
        let mut day = 0;
        let mut weekday = 0;
        let mut month = 0;
        let mut year = 1900;

        let month_daycount = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let mut sunday_count = 0;

        while year != 2001 {
            if (1901..=2000).contains(&year) && weekday == 6 && day == 0 {
                // println!("{:2}.{:2}.{:4}", day + 1, month + 1, year);
                sunday_count += 1;
            }

            day += 1;
            weekday = (weekday + 1) % 7;

            let mut cur_month_days = month_daycount[month];

            let is_leap = |x| {
                if x % 400 == 0 {
                    return true;
                } else if x % 100 == 0 {
                    return false;
                }
                x % 4 == 0
            };

            if month == 1 && is_leap(year) {
                cur_month_days = 29;
            }

            if day >= cur_month_days {
                day = 0;
                month += 1;
            }

            if month >= 12 {
                year += 1;
                month = 0;
            }
        }

        sunday_count
    }
}

mod prob_20 {
    use num_bigint::BigUint;

    pub fn solve() -> u64 {
        let mut result = BigUint::new(vec![1]);

        for i in 2..=100 {
            result = BigUint::new(vec![i]) * &result;
        }

        result
            .to_string()
            .chars()
            .map(|a| a.to_digit(10).unwrap() as u64)
            .sum()
    }
}

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
    println!("Problem 1: {}", prob_1::solve(1000));
    println!("Problem 2: {}", prob_2::solve());
    println!("Problem 3: {}", prob_3::solve(600851475143));
    println!("Problem 4: {}", prob_4::solve(3));
    println!("Problem 5: {}", prob_5::solve(20));
    println!("Problem 6: {}", prob_6::solve(100));
    println!("Problem 7: {}", prob_7::solve(10001));
    println!("Problem 8: {}", prob_8::solve());
    println!("Problem 9: {:?}", prob_9::solve(1000));
    println!("Problem 10: {}", prob_10::solve(2_000_000));
    println!("Problem 11: {}", prob_11::solve());
    println!("Problem 12: {}", prob_12::solve(500));
    println!("Problem 13: {}", prob_13::solve());
    println!("Problem 14: {}", prob_14::solve(1_000_000));
    println!("Problem 15: {}", prob_15::solve(20, 20));
    println!("Problem 16: {}", prob_16::solve(1000));
    println!("Problem 17: {}", prob_17::solve(1000));
    println!("Problem 18: {}", prob_18::solve());
    println!("Problem 19: {}", prob_19::solve());
    println!("Problem 20: {}", prob_20::solve());
    println!("Problem 21: {}", prob_21::solve(10000));
    println!("Problem 22: {}", prob_22::solve());
    println!("Problem 23: {}", prob_23::solve());
    println!("Problem 24: {}", prob_24::solve());
    println!("Problem 25: {}", prob_25::solve());
}
