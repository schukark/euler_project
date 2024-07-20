use std::collections::{BTreeMap, HashMap};

pub fn choose(n: u64, k: u64, map: &mut BTreeMap<(u64, u64), u64>) -> u64 {
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

pub fn partition(left: i128, last: i128) -> i128 {
    if left == 0 {
        return 1;
    }

    if left < last {
        return 0;
    }

    (last..=left).map(|n| partition(left - n, n)).sum()
}

pub fn partition_fast_mod(number: i64, lookup: &mut HashMap<i64, i64>, modulus: i64) -> i64 {
    if let Some(e) = lookup.get(&number) {
        return *e;
    }

    if number == 0 {
        return 1;
    }

    if number < 0 {
        return 0;
    }

    let mut k = 1;

    let mut result = 0;

    loop {
        if number < k * (3 * k - 1) / 2 {
            break;
        }

        let tmp = partition_fast_mod(number - k * (3 * k - 1) / 2, lookup, modulus)
            + partition_fast_mod(number - k * (3 * k + 1) / 2, lookup, modulus);

        if k % 2 == 1 {
            result += tmp;
        } else {
            result -= tmp;
        }

        result %= modulus;

        k += 1;
    }

    lookup.insert(number, result);

    result
}

#[test]
fn test_partition_fast() {
    let mut lookup = HashMap::new();
    let partition_fast = partition_fast_mod(5, &mut lookup, 10);
    assert_eq!(partition_fast, 7);
}
