use crate::utils::file_ops::read_grid;

pub fn solve() -> i128 {
    let grid = read_grid::<i128>("src/txts/prob_81.txt", ",");
    let mut dp = vec![vec![i128::MAX; grid[0].len()]; grid.len()];

    dp[0][0] = grid[0][0];

    assert!(grid.len() == 80);
    assert!(grid[0].len() == 80);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut cur_min = i64::MAX as i128;

            if i >= 1 {
                cur_min = i128::min(cur_min, dp[i - 1][j]);
            }
            if j >= 1 {
                cur_min = i128::min(cur_min, dp[i][j - 1]);
            }

            dp[i][j] = i128::min(dp[i][j], cur_min + grid[i][j]);
        }
    }

    dp[grid.len() - 1][grid[0].len() - 1]
}
