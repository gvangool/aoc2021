use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Clone)]
pub struct Octopuses {
    grid: Vec<Vec<usize>>,
}

impl Octopuses {
    pub fn all(&self, expected: usize) -> bool {
        self.grid.iter().all(|col| col.iter().all(|&i| i == expected))
    }

    pub fn neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut to_check: Vec<(usize, usize)> = Vec::new();
        let last_row = self.grid.len() - 1;
        let last_col = self.grid[i].len() - 1;
        match (i, j) {
            (_, _) if (i == 0 && j == 0) => {
                to_check.push((i, j + 1));
                to_check.push((i + 1, j));
                to_check.push((i + 1, j + 1));
            }
            (_, _) if (i == last_row && j == 0) => {
                to_check.push((i - 1, j));
                to_check.push((i - 1, j + 1));
                to_check.push((i, j + 1));
            }
            (_, _) if (j == 0) => {
                to_check.push((i - 1, j));
                to_check.push((i - 1, j + 1));
                to_check.push((i, j + 1));
                to_check.push((i + 1, j));
                to_check.push((i + 1, j + 1));
            }
            (_, _) if (i == 0 && j == last_col) => {
                to_check.push((i, j - 1));
                to_check.push((i + 1, j - 1));
                to_check.push((i + 1, j));
            }
            (_, _) if (i == last_row && j == last_col) => {
                to_check.push((i - 1, j - 1));
                to_check.push((i - 1, j));
                to_check.push((i, j - 1));
            }
            (_, _) if (j == last_col) => {
                to_check.push((i - 1, j - 1));
                to_check.push((i - 1, j));
                to_check.push((i, j - 1));
                to_check.push((i + 1, j - 1));
                to_check.push((i + 1, j));
            }
            (_, _) if (i == 0) => {
                to_check.push((i, j - 1));
                to_check.push((i, j + 1));
                to_check.push((i + 1, j - 1));
                to_check.push((i + 1, j));
                to_check.push((i + 1, j + 1));
            }
            (_, _) if (i == last_row) => {
                to_check.push((i - 1, j - 1));
                to_check.push((i - 1, j));
                to_check.push((i - 1, j + 1));
                to_check.push((i, j - 1));
                to_check.push((i, j + 1));
            }
            (_, _) => {
                to_check.push((i - 1, j - 1));
                to_check.push((i - 1, j));
                to_check.push((i - 1, j + 1));
                to_check.push((i, j - 1));
                to_check.push((i, j + 1));
                to_check.push((i + 1, j - 1));
                to_check.push((i + 1, j));
                to_check.push((i + 1, j + 1));
            }
        };
        to_check
    }
}

#[aoc_generator(day11)]
pub fn parse(input: &str) -> Octopuses {
    let mut grid: Vec<Vec<usize>> = Vec::new();

    for line in input.lines().into_iter() {
        let mut numbers: Vec<usize> = Vec::new();
        for c in line.chars() {
            numbers.push(c.to_string().parse::<usize>().unwrap());
        }
        grid.push(numbers);
    }

    Octopuses { grid: grid }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Octopuses) -> usize {
    let mut octopuses = input.clone();

    let mut flashes = 0;
    for _step in 0..100 {
        let step_flashes = do_step(&mut octopuses);
        flashes += step_flashes;
    }
    flashes
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Octopuses) -> usize {
    let mut octopuses = input.clone();
    let mut step = 0;

    while !octopuses.all(0) {
        do_step(&mut octopuses);
        step += 1
    }
    step
}

pub fn do_step(octopuses: &mut Octopuses) -> usize {
    let row_len = octopuses.grid.len();
    let col_len = octopuses.grid[0].len();
    let mut step_flashes = 0;
    let mut to_flash: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..row_len {
        for j in 0..col_len {
            octopuses.grid[i][j] += 1;
            if octopuses.grid[i][j] > 9 {
                to_flash.insert((i, j));
            }
        }
    }

    while to_flash.len() != step_flashes {
        let mut tmp_to_flash: HashSet<(usize, usize)> = HashSet::new();
        for &(i, j) in to_flash.iter() {
            if octopuses.grid[i][j] == 0 {
                // already handled this one
                continue;
            }
            octopuses.grid[i][j] = 0;
            step_flashes += 1;
            for (ii, jj) in octopuses.neighbours(i, j) {
                if octopuses.grid[ii][jj] != 0 {
                    octopuses.grid[ii][jj] += 1;
                    if octopuses.grid[ii][jj] > 9 {
                        tmp_to_flash.insert((ii, jj));
                    }
                }
            }
        }
        to_flash.extend(tmp_to_flash);
    }
    step_flashes
}
