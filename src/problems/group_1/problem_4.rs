pub fn solve(digits: u32) -> u64 {
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
