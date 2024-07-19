pub fn solve(number: u64) -> u64 {
    let mut current_div: u64 = 2;
    let mut result: u64 = 0;
    let mut number = number;

    while number > 1 {
        while number % current_div == 0 {
            number /= current_div;
            result = current_div;
        }

        current_div += 1;
    }

    result
}