use crate::utils::file_ops::read_grid;

pub fn solve() -> u64 {
    let grid: Vec<Vec<u64>> = read_grid("src/txts/prob_18.txt");

    let mut dp: Vec<Vec<u64>> = vec![Vec::new(); grid.len()];
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
                result = u64::max(result, dp[i - 1][j]);
            }

            dp[i][j] = u64::max(dp[i][j], result + grid[i][j]);
        }
    }

    *dp[dp.len() - 1].iter().max().unwrap()
}
