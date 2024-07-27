fn generate_counts(n: i64, dice: i64) -> Vec<i64> {
    let mut dp: Vec<i64> = vec![0; n as usize * dice as usize + 1];

    _generate_counts(n, dice, &[], &mut dp);

    dp
}

fn _generate_counts(n: i64, dice: i64, current: &[i64], dp: &mut [i64]) {
    if current.len() == n as usize {
        dp[current.iter().sum::<i64>() as usize] += 1;
        return;
    }

    for i in 1..=dice {
        let mut current = current.to_vec();
        current.push(i);
        _generate_counts(n, dice, &current, dp);
    }
}

#[allow(clippy::needless_range_loop)]
pub fn solve() -> i128 {
    let peter_dp = generate_counts(9, 4);
    let collin_dp = generate_counts(6, 6);

    let mut answer = 0_i128;

    for i in 0..peter_dp.len() {
        for j in 0..collin_dp.len() {
            if i <= j {
                continue;
            }

            answer += peter_dp[i] as i128 * collin_dp[j] as i128;
        }
    }

    let answer = answer as f64
        / (peter_dp.iter().sum::<i64>() as i128 * collin_dp.iter().sum::<i64>() as i128) as f64;

    assert!((0.0..=1.0).contains(&answer));
    assert_eq!(collin_dp[6], 1);
    assert_eq!(collin_dp[7], 6);
    assert_eq!(collin_dp[36], 1);
    assert_eq!(collin_dp[35], 6);
    assert_eq!(peter_dp[9], 1);
    assert_eq!(peter_dp[10], 9);
    assert_eq!(peter_dp[36], 1);
    assert_eq!(peter_dp[35], 9);

    let answer = format!("{:.7}", answer)
        .split('.')
        .skip(1)
        .collect::<String>();

    answer.parse::<i128>().unwrap()
}
