use std::collections::BTreeSet;

use crate::utils::file_ops::read_grid;

fn _solve(pathname: &str) -> i128 {
    let grid = read_grid::<i128>(pathname, ",");

    let mut queue: BTreeSet<(i128, i128, i128)> = BTreeSet::new();
    let mut dist = vec![vec![i128::MAX; grid[0].len()]; grid.len()];

    (0..grid.len()).for_each(|i| {
        dist[i][0] = grid[i][0];
        queue.insert((dist[i][0], i as i128, 0));
    });

    let directions = [(1, 0), (-1, 0), (0, 1)];

    while !queue.is_empty() {
        let (top_dist, top_x, top_y) = queue.pop_first().unwrap();

        for direction in directions {
            let (new_x, new_y) = (top_x + direction.0, top_y + direction.1);

            if new_x < 0
                || new_y < 0
                || new_x as usize >= grid.len()
                || new_y as usize >= grid[0].len()
            {
                continue;
            }

            if dist[new_x as usize][new_y as usize]
                < top_dist + grid[new_x as usize][new_y as usize]
            {
                continue;
            }

            queue.remove(&(dist[new_x as usize][new_y as usize], new_x, new_y));
            dist[new_x as usize][new_y as usize] = top_dist + grid[new_x as usize][new_y as usize];
            queue.insert((dist[new_x as usize][new_y as usize], new_x, new_y));
        }
    }

    (0..grid.len()).map(|i| dist[i][grid[0].len() - 1]).min().unwrap()
}

pub fn solve() -> i128 {
    _solve("src/txts/prob_82.txt")
}

#[test]
fn test_example() {
    assert_eq!(_solve("src/txts/prob_82_example.txt"), 994);
}
