use num_traits::{AsPrimitive, Num};

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
    T: std::cmp::PartialOrd<i64>
        + std::ops::Rem<i64>
        + std::ops::DivAssign<i64>
        + std::convert::From<i64>,
    <T as std::ops::Rem<i64>>::Output: std::cmp::PartialEq<i64>,
    T: Copy,
{
    let mut primes: Vec<(T, T)> = Vec::new();

    let mut number = number;
    let mut cur_div = 2;

    while number > cur_div * cur_div {
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

    if number > 1 {
        primes.push((number, 1.into()));
    }

    primes
}

pub fn get_factorization_fast<T>(mut number: T, primes: &[T]) -> Vec<(T, i32)>
where
    T: std::cmp::PartialOrd<T>
        + std::ops::Rem<Output = T>
        + std::ops::DivAssign<T>
        + std::ops::Mul<Output = T>
        + From<i32>
        + Copy,
{
    let mut factorization = Vec::new();

    for &prime in primes {
        if prime * prime > number {
            break;
        }

        let mut count = 0;

        while number % prime == 0.into() {
            number /= prime;
            count += 1;
        }

        if count > 0 {
            factorization.push((prime, count));
        }
    }

    if number > 1.into() {
        factorization.push((number, 1));
    }

    factorization
}

pub fn totient_sieve<T>(limit: usize) -> Vec<T>
where
    T: Default + Clone + Copy,
    T: Num + AsPrimitive<usize>,
    usize: AsPrimitive<T>,
{
    let mut phi: Vec<T> = vec![T::zero(); limit + 1];
    let mut is_composite: Vec<bool> = vec![false; limit + 1];
    let mut primes: Vec<T> = Vec::new();

    phi[1] = T::one();

    for i in 2..=limit {
        if !is_composite[i] {
            primes.push(i.as_());
            phi[i] = (i - 1).as_();
        }

        let mut j = 0;

        while j < primes.len() && i * primes[j].as_() <= limit {
            is_composite[i * primes[j].as_()] = true;

            if i % primes[j].as_() == 0 {
                phi[i * primes[j].as_()] = phi[i] * primes[j];
            } else {
                phi[i * primes[j].as_()] = phi[i] * phi[primes[j].as_()];
            }

            j += 1;
        }
    }

    phi
}
