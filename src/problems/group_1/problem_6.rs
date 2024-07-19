pub fn solve(limit: u64) -> u64 {
    limit * limit * (limit + 1) * (limit + 1) / 4 - limit * (limit + 1) * (2 * limit + 1) / 6
}
