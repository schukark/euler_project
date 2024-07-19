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
