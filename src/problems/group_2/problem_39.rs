fn count_solutions(perimeter: i128) -> i128 {
    let mut count = 0;

    for i in 1..perimeter {
        for j in 1..perimeter - i {
            let k = perimeter - i - j;

            if i * i + j * j != k * k {
                continue;
            }

            count += 1;
        }
    }

    count
}

pub fn solve() -> i128 {
    let (mut best_perimeter, mut best_count) = (0, 0);
    for i in 1..=1000 {
        let tmp = count_solutions(i);

        if tmp > best_count {
            best_count = tmp;
            best_perimeter = i;
        }
    }

    best_perimeter
}
