fn lcm(u: u64, v: u64) -> u64 {
    if u == 0 || v == 0 {
        return gcd::binary_u64(u, v);
    }
    u * v / gcd::binary_u64(u, v)
}

pub fn solve(limit: u32) -> u64 {
    let mut result: u64 = 0;

    for i in 1..=limit {
        result = lcm(result, i as u64);
    }

    result
}