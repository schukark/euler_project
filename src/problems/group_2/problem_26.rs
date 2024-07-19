fn reduce_2_5(number: i128) -> i128 {
    let mut number = number;
    while number % 2 == 0 {
        number /= 2;
    }

    while number % 5 == 0 {
        number /= 5;
    }

    number
}

fn find_loop_len(denominator: i128) -> usize {
    let new_num = reduce_2_5(denominator);

    if new_num == 1 {
        return 0;
    }

    let mut numerator = 10;

    let mut cycle = Vec::new();

    let num_len = denominator.to_string().len();

    loop {
        cycle.push(numerator / new_num);
        numerator %= new_num;
        numerator *= 10;

        if cycle.len() > num_len && cycle[..num_len] == cycle[cycle.len() - num_len..] {
            return cycle.len() - num_len;
        }
    }
}

#[test]
pub fn test_6() {
    assert_eq!(find_loop_len(6), 1);
}

#[test]
pub fn test_7() {
    assert_eq!(find_loop_len(7), 6);
}

#[test]
pub fn test_11() {
    assert_eq!(find_loop_len(11), 2);
}

pub fn solve(limit: i128) -> i128 {
    let (mut best_cycle, mut best_denom) = (0, 1);
    for i in 1..limit {
        let tmp = find_loop_len(i);

        if tmp > best_cycle {
            best_cycle = tmp;
            best_denom = i;
        }
    }

    best_denom
}
