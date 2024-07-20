use crate::utils::file_ops::read_grid;

pub fn solve() -> i128 {
    let grid = read_grid::<i128>("src/txts/prob_67.txt", " ");

    let mut dp: Vec<Vec<i128>> = vec![Vec::new(); grid.len()];
    for i in 0..dp.len() {
        dp[i] = vec![0; grid[i].len()];
    }

    for i in 0..dp.len() {
        for j in 0..dp[i].len() {
            let mut result = 0;

            if i > 0 && j > 0 {
                result = dp[i - 1][j - 1];
            }
            if i > 0 && j < dp[i - 1].len() {
                result = i128::max(result, dp[i - 1][j]);
            }

            dp[i][j] = i128::max(dp[i][j], result + grid[i][j]);
        }
    }

    *dp[dp.len() - 1].iter().max().unwrap()
}
