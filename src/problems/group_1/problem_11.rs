use crate::utils::file_ops::read_grid;

pub fn solve() -> u64 {
    let grid = read_grid("src/txts/prob_11.txt").into_iter().flatten().collect::<Vec<i32>>();

    assert_eq!(grid.len(), 400);

    let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (1, 1), (1, -1)];
    let mut max_prod = 1;

    for direction in directions {
        for start_x in 0..20 {
            for start_y in 0..20 {
                if start_x + 4 * direction.0 < 0
                    || start_x + 4 * direction.0 >= 20
                    || start_y + 4 * direction.1 < 0
                    || start_y + 4 * direction.1 >= 20
                {
                    continue;
                }

                let mut product: i32 = 1;

                for i in 0..4 {
                    product *= grid[(20 * (start_x + i * direction.0)
                        + start_y
                        + i * direction.1) as usize];
                }

                if max_prod >= product as u64 {
                    continue;
                }
                max_prod = product as u64;
            }
        }
    }

    max_prod
}