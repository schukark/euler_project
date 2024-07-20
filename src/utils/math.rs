use std::collections::BTreeMap;

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

pub fn partition_mod(left: i128, last: i128, modulus: i128) -> i128 {
    if left == 0 {
        return 1;
    }

    if left < last {
        return 0;
    }

    (last..=left).map(|n| partition(left - n, n)).sum::<i128>() % modulus
}
