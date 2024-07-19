pub fn solve(sum: u64) -> u64 {
    let mut result: Vec<u64> = Vec::new();

    for i in 1..sum {
        for j in i..sum - i {
            let k = sum - i - j;

            if i * i + j * j == k * k {
                result.push(i * j * k);
            }
        }
    }

    result.iter().product()
}
