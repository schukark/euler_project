use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use num_bigint::BigUint;

pub fn prob_1_compute(limit: u32) -> u64 {
    let mut result: u64 = 0;
    for i in 1..limit {
        if i % 5 != 0 && i % 3 != 0 {
            continue;
        }

        result += i as u64;
    }

    result
}

pub fn prob_2_compute() -> u64 {
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

pub fn prob_3_compute(number: u64) -> u64 {
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

pub fn prob_4_compute(digits: u32) -> u64 {
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

pub fn prob_5_compute(limit: u32) -> u64 {
    let mut result: u64 = 0;

    for i in 1..=limit {
        result = lcm(result, i as u64);
    }

    result
}

pub fn prob_6_compute(limit: u64) -> u64 {
    limit * limit * (limit + 1) * (limit + 1) / 4 - limit * (limit + 1) * (2 * limit + 1) / 6
}

fn check_primes(number: u64, primes: &mut Vec<u64>) -> bool {
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

pub fn prob_7_compute(limit: u64) -> u64 {
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

pub fn prob_8_compute() -> u64 {
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

pub fn prob_9_compute(sum: u64) -> Vec<u64> {
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

pub fn prob_10_compute(limit: u64) -> u64 {
    let mut primes: Vec<u64> = Vec::new();
    let mut sum: u64 = 0;

    for i in 2..limit {
        if check_primes(i, &mut primes) {
            sum += i;
        }
    }

    sum
}

pub fn prob_11_compute() -> u64 {
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
                    product *= grid
                        [(20 * (start_x + i * direction.0) + start_y + i * direction.1) as usize];
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

fn count_div(number: u64) -> u64 {
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

pub fn prob_12_compute(limit: u64) -> u64 {
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

fn add_strings(s1: &str, s2: &str) -> String {
    let mut i = s1.len() as isize - 1;
    let mut j = s2.len() as isize - 1;

    let mut result = String::new();

    let mut carry = 0;

    while i >= 0 || j >= 0 || carry > 0 {
        let mut tmp = carry;

        if i >= 0 {
            tmp += s1.chars().nth(i as usize).unwrap().to_digit(10).unwrap();
            i -= 1;
        }

        if j >= 0 {
            tmp += s2.chars().nth(j as usize).unwrap().to_digit(10).unwrap();
            j -= 1;
        }

        result = (tmp % 10).to_string() + &result;
        carry = tmp / 10;
    }

    result
}

pub fn prob_13_compute() -> String {
    let read_to_string = fs::read_to_string("src/txts/prob_13.txt")
        .unwrap()
        .split('\n')
        .map(|f| f.to_owned())
        .collect::<Vec<String>>();

    assert_eq!(add_strings("1", "2"), "3");
    assert_eq!(add_strings("9", "9"), "18");

    let mut result = read_to_string[0].clone();

    (1..read_to_string.len()).for_each(|i| {
        result = add_strings(&result, &read_to_string[i]);
    });

    result[..10].to_string()
}

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

pub fn prob_14_compute(limit: u64) -> u64 {
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

pub fn prob_15_compute(width: u64, height: u64) -> u64 {
    let mut map = BTreeMap::new();
    choose(width + height, height, &mut map)
}

fn digit_sum(s: &str) -> u64 {
    s.chars().map(|a| a.to_digit(10).unwrap() as u64).sum()
}

pub fn prob_16_compute(limit: u64) -> u64 {
    assert_eq!(digit_sum("1024"), 7);

    let mut result = "1".to_owned();

    for _i in 0..limit {
        result = add_strings(&result, &result);
    }

    digit_sum(&result)
}

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

pub fn prob_17_compute(limit: u64) -> usize {
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

pub fn prob_18_compute() -> u64 {
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

pub fn prob_19_compute() -> u64 {
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

pub fn prob_20_compute() -> u64 {
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

pub fn main() {
    println!("Problem 1: {}", prob_1_compute(1000));
    println!("Problem 2: {}", prob_2_compute());
    println!("Problem 3: {}", prob_3_compute(600851475143));
    println!("Problem 4: {}", prob_4_compute(3));
    println!("Problem 5: {}", prob_5_compute(20));
    println!("Problem 6: {}", prob_6_compute(100));
    println!("Problem 7: {}", prob_7_compute(10001));
    println!("Problem 8: {}", prob_8_compute());
    println!("Problem 9: {:?}", prob_9_compute(1000));
    println!("Problem 10: {}", prob_10_compute(2_000_000));
    println!("Problem 11: {}", prob_11_compute());
    println!("Problem 12: {}", prob_12_compute(500));
    println!("Problem 13: {}", prob_13_compute());
    println!("Problem 14: {}", prob_14_compute(1_000_000));
    println!("Problem 15: {}", prob_15_compute(20, 20));
    println!("Problem 16: {}", prob_16_compute(1000));
    println!("Problem 17: {}", prob_17_compute(1000));
    println!("Problem 18: {}", prob_18_compute());
    println!("Problem 19: {}", prob_19_compute());
    println!("Problem 20: {}", prob_20_compute());
}
