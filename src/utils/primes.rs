pub fn prime_number_sieve<T>(limit: usize) -> Vec<T>
where
    T: std::str::FromStr,
    T: std::convert::TryFrom<usize>,
    T::Error: std::fmt::Debug,
{
    let mut primes: Vec<T> = Vec::new();

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
        if !*value || index == 0 {
            return;
        }

        primes.push(index.try_into().unwrap());
    });

    primes
}

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

pub fn get_prime_exponents<T>(number: T) -> Vec<(T, T)>
where
    T: std::cmp::PartialOrd<i128>
        + std::ops::Rem<i128>
        + std::ops::DivAssign<i128>
        + std::convert::From<i128>,
    <T as std::ops::Rem<i128>>::Output: std::cmp::PartialEq<i128>,
    T: Copy,
{
    let mut primes: Vec<(T, T)> = Vec::new();

    let mut number = number;
    let mut cur_div = 2;

    while number > 1 {
        let mut count = 0;
        while number % cur_div == 0 {
            number /= cur_div;
            count += 1;
        }

        if count > 0 {
            primes.push((cur_div.into(), count.into()));
        }

        cur_div += 1;
    }

    primes
}
