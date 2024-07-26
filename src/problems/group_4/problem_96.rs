use std::{collections::HashSet, fmt::Display, fs};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum GridCell {
    Empty,
    Occupied(u8),
    Permanent(u8),
}

impl GridCell {
    fn unwrap(&self) -> u8 {
        match self {
            GridCell::Empty => panic!(),
            GridCell::Occupied(e) | GridCell::Permanent(e) => *e,
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self, GridCell::Empty)
    }
}

impl Display for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridCell::Empty => "0".to_string(),
                GridCell::Occupied(e) | GridCell::Permanent(e) => e.to_string(),
            }
        )
    }
}

struct SudokuGrid {
    cells: [[GridCell; 9]; 9],
}

impl SudokuGrid {
    fn from_string(grid: Vec<String>) -> Self {
        let mut cells = [[GridCell::Empty; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                if grid[i].chars().nth(j).unwrap() == '0' {
                    continue;
                }

                cells[i][j] = GridCell::Permanent(grid[i].chars().nth(j).unwrap() as u8 - b'0');
            }
        }

        SudokuGrid { cells }
    }

    fn absent_row(&self, row: usize) -> HashSet<u8> {
        let mut numbers = HashSet::from_iter(1..=9);
        for i in 0..9 {
            match self.cells[row][i] {
                GridCell::Empty => continue,
                GridCell::Occupied(e) | GridCell::Permanent(e) => {
                    numbers.remove(&e);
                }
            }
        }

        numbers
    }

    fn absent_col(&self, col: usize) -> HashSet<u8> {
        let mut numbers = HashSet::from_iter(1..=9);
        for i in 0..9 {
            match self.cells[i][col] {
                GridCell::Empty => continue,
                GridCell::Occupied(e) | GridCell::Permanent(e) => {
                    numbers.remove(&e);
                }
            }
        }

        numbers
    }

    fn absent_square(&self, x_coord: usize, y_coord: usize) -> HashSet<u8> {
        let mut numbers = HashSet::from_iter(1..=9);

        for i in x_coord * 3..x_coord * 3 + 3 {
            for j in y_coord * 3..y_coord * 3 + 3 {
                match self.cells[i][j] {
                    GridCell::Empty => continue,
                    GridCell::Occupied(e) | GridCell::Permanent(e) => {
                        numbers.remove(&e);
                    }
                };
            }
        }

        numbers
    }

    fn cell_variants(&self, x: usize, y: usize) -> HashSet<u8> {
        if matches!(
            self.cells[x][y],
            GridCell::Occupied(_) | GridCell::Permanent(_)
        ) {
            return HashSet::new();
        }
        let digits = self
            .absent_col(0)
            .intersection(
                &self
                    .absent_row(0)
                    .intersection(&self.absent_square(0, 0))
                    .copied()
                    .collect(),
            )
            .copied()
            .collect();

        digits
    }

    fn check_solved(&self) -> bool {
        (0..9).all(|f| self.absent_row(f).iter().filter(|&&x| x == 0).count() == 9)
            && (0..9).all(|f| self.absent_col(f).iter().filter(|&&x| x == 0).count() == 9)
            && (0..9).all(|f| {
                self.absent_square(f / 3, f % 3)
                    .iter()
                    .filter(|&&x| x == 0)
                    .count()
                    == 9
            })
    }

    fn solve(&mut self) -> bool {
        if self.check_solved() {
            return true;
        }

        let mut best = (0, 0);
        let mut best_variants = 10;

        for i in 0..9 {
            for j in 0..9 {
                if !self.cells[i][j].is_empty() {
                    continue;
                }

                let tmp_count = self.cell_variants(i, j).len();

                if best_variants > tmp_count {
                    best_variants = tmp_count;
                    best = (i, j);
                }
            }
        }

        for variant in self.cell_variants(best.0, best.1) {
            self.cells[best.0][best.1] = GridCell::Occupied(variant);

            if self.solve() {
                return true;
            }
        }

        false
    }

    fn get_top_left_sum(&self) -> i128 {
        self.cells[0][0].unwrap() as i128 * 100
            + self.cells[0][1].unwrap() as i128 * 10
            + self.cells[0][2].unwrap() as i128
    }
}

impl Display for SudokuGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_string = String::new();

        for i in 0..9 {
            for j in 0..9 {
                display_string += &self.cells[i][j].to_string();
            }
            display_string += "\n";
        }

        write!(f, "{display_string}")
    }
}

fn read_sudoku(filename: &'static str) -> Vec<SudokuGrid> {
    fs::read_to_string(filename)
        .expect("File not found!")
        .split('\n')
        .array_chunks::<10>()
        .map(|lines| {
            lines
                .iter()
                .skip(1)
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        })
        .map(SudokuGrid::from_string)
        .collect::<Vec<_>>()
}

pub fn solve() -> i128 {
    let mut sudoku = read_sudoku("src/txts/prob_96.txt");

    sudoku
        .iter_mut()
        .map(|sudoku| {
            sudoku.solve();
            println!("{}", sudoku);
            sudoku.get_top_left_sum()
        })
        .sum::<i128>()
}
