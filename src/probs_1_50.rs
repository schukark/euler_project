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
