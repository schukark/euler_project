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
