use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

pub struct Floor {
    grid: Vec<Vec<usize>>,
}

impl Floor {
    pub fn neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
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
            (_, _) if (i == last_row) => {
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
    }
    pub fn is_low_point(&self, i: usize, j: usize) -> bool {
        let to_check: Vec<(usize, usize)> = self.neighbours(i, j);
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

#[aoc(day9, part2)]
pub fn solve_part2(ocean_floor: &Floor) -> usize {
    let mut basin_size: Vec<usize> = Vec::new();
    for i in 0..ocean_floor.grid.len() {
        for j in 0..ocean_floor.grid[i].len() {
            if ocean_floor.is_low_point(i, j) {
                let mut basin: HashSet<(usize, usize)> = HashSet::new();
                check_basin(&mut basin, &ocean_floor, i, j);
                basin_size.push(basin.len());
            }
        }
    }
    // We need the biggest ones
    basin_size.sort();
    basin_size.reverse();

    basin_size.iter().take(3).fold(1, |acc, x| acc * x)
}

pub fn check_basin(
    mut basin: &mut HashSet<(usize, usize)>,
    ocean_floor: &Floor,
    i: usize,
    j: usize,
) {
    for (ii, jj) in ocean_floor.neighbours(i, j) {
        // anything that's not a 9 is part of our basin
        if ocean_floor.grid[ii][jj] != 9 {
            if basin.insert((ii, jj)) {
                check_basin(&mut basin, &ocean_floor, ii, jj);
            }
        }
    }
}
