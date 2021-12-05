use std::cmp;

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Point {
    x: usize,
    y: usize,
}

pub struct Line {
    from: Point,
    to: Point,
}

impl Line {
    pub fn points(&self) -> Vec<Point> {
        if self.from.x == self.to.x {
            let min_y = cmp::min(self.from.y, self.to.y);
            let max_y = cmp::max(self.from.y, self.to.y);
            (min_y..(max_y + 1))
                .map(|y| Point {
                    x: self.from.x,
                    y: y,
                })
                .collect()
        } else if self.from.y == self.to.y {
            let min_x = cmp::min(self.from.x, self.to.x);
            let max_x = cmp::max(self.from.x, self.to.x);
            (min_x..(max_x + 1))
                .map(|x| Point {
                    x: x,
                    y: self.from.y,
                })
                .collect()
        } else {
            let range_x: Vec<usize> = if self.from.x < self.to.x {
                (self.from.x..(self.to.x + 1)).collect()
            } else {
                (self.to.x..(self.from.x + 1)).collect()
            };
            let range_y: Vec<usize> = if self.from.x < self.to.x {
                if self.from.y < self.to.y {
                    (self.from.y..(self.to.y + 1)).collect()
                } else {
                    (self.to.y..(self.from.y + 1)).rev().collect()
                }
            } else {
                if self.to.y < self.from.y {
                    (self.to.y..(self.from.y + 1)).collect()
                } else {
                    (self.from.y..(self.to.y + 1)).rev().collect()
                }
            };
            range_x
                .iter()
                .zip(range_y)
                .map(|(&x, y)| Point { x: x, y: y })
                .collect()
        }
    }
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            // 0,9 -> 5,9
            let mut values = line.split(' ');
            let p1: Vec<usize> = values
                .next()
                .unwrap()
                .split(',')
                .map(|i| i.parse::<usize>().unwrap())
                .collect();
            values.next(); // ->
            let p2: Vec<usize> = values
                .next()
                .unwrap()
                .split(',')
                .map(|i| i.parse::<usize>().unwrap())
                .collect();
            Line {
                from: Point { x: p1[0], y: p1[1] },
                to: Point { x: p2[0], y: p2[1] },
            }
        })
        .collect()
}

pub fn calculate_intersection(lines: &Vec<&Line>) -> usize {
    let max_x: usize = lines
        .iter()
        .map(|l| cmp::max(l.from.x, l.to.x))
        .max()
        .unwrap();
    let max_y: usize = lines
        .iter()
        .map(|l| cmp::max(l.from.y, l.to.y))
        .max()
        .unwrap();
    let row: Vec<usize> = vec![0; max_y + 1];

    let mut grid: Vec<Vec<usize>> = Vec::new();
    for _i in 0..(max_x + 1) {
        grid.push(row.clone());
    }

    let mut dangerous_point: usize = 0;
    for line in lines {
        for p in line.points() {
            grid[p.x][p.y] += 1;
            if grid[p.x][p.y] == 2 {
                dangerous_point += 1;
            }
        }
    }
    dangerous_point
}

#[aoc(day5, part1)]
pub fn solve_part1(lines: &Vec<Line>) -> usize {
    let simple_lines: Vec<&Line> = lines
        .iter()
        .filter(|&l| l.from.x == l.to.x || l.from.y == l.to.y)
        .collect();
    calculate_intersection(&simple_lines)
}

#[aoc(day5, part2)]
pub fn solve_part2(lines: &Vec<Line>) -> usize {
    calculate_intersection(&(lines.iter().collect()))
}
