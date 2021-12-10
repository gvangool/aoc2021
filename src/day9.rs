use aoc_runner_derive::{aoc, aoc_generator};

pub struct Floor {
    grid: Vec<Vec<usize>>,
}

impl Floor {
    pub fn is_low_point(&self, i: usize, j: usize) -> bool {
        let mut to_check: Vec<(usize, usize)> = Vec::new();
        let last_row = self.grid.len() - 1;
        let last_col = self.grid[i].len() - 1;
        match (i, j) {
            (_, _) if (i == 0 && j == 0) => {
                to_check.push((i, j + 1));
                to_check.push((i + 1, j));
            }
            (_, _) if (i == last_row && j == 0) => {
                to_check.push((i - 1, j));
                to_check.push((i, j + 1));
            }
            (_, _) if (j == 0) => {
                to_check.push((i - 1, j));
                to_check.push((i, j + 1));
                to_check.push((i + 1, j));
            }
            (_, _) if (i == 0 && j == last_col) => {
                to_check.push((i, j - 1));
                to_check.push((i + 1, j));
            }
            (_, _) if (i == last_row && j == last_col) => {
                to_check.push((i - 1, j));
                to_check.push((i, j - 1));
            }
            (_, _) if (j == last_col) => {
                to_check.push((i - 1, j));
                to_check.push((i, j - 1));
                to_check.push((i + 1, j));
            }
            (_, _) if (i == 0) => {
                to_check.push((i, j - 1));
                to_check.push((i, j + 1));
                to_check.push((i + 1, j));
            }
            (_, _) if (i == last_row)=> {
                to_check.push((i - 1, j));
                to_check.push((i, j - 1));
                to_check.push((i, j + 1));
            }
            (_, _) => {
                to_check.push((i - 1, j));
                to_check.push((i, j - 1));
                to_check.push((i, j + 1));
                to_check.push((i + 1, j));
            }
        };
        to_check
            .iter()
            .all(|&(ii, jj)| self.grid[i][j] < self.grid[ii][jj])
    }
}

#[aoc_generator(day9)]
pub fn parse(input: &str) -> Floor {
    let mut grid: Vec<Vec<usize>> = Vec::new();

    for line in input.lines().into_iter() {
        let mut numbers: Vec<usize> = Vec::new();
        for c in line.chars() {
            numbers.push(c.to_string().parse::<usize>().unwrap());
        }
        grid.push(numbers);
    }

    Floor { grid: grid }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Floor) -> usize {
    let mut low_points: Vec<usize> = Vec::new();
    for i in 0..input.grid.len() {
        for j in 0..input.grid[i].len() {
            if input.is_low_point(i, j) {
                low_points.push(input.grid[i][j]);
            }
        }
    }
    low_points.iter().map(|n| n + 1).sum()
}
