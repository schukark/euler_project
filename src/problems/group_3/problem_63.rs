fn check_n(n: u32) -> i128 {
    (1..=9)
        .filter(|i| (*i as i128).pow(n).to_string().len() == n as usize)
        .count() as i128
}

pub fn solve() -> i128 {
    let mut cur_n = 1;
    let mut answer = 0;

    loop {
        let tmp = check_n(cur_n);

        if tmp == 0 {
            break;
        }

        answer += tmp;
        cur_n += 1;
    }

    answer
}
