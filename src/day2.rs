use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    units: usize,
}

pub struct Position {
    horizontal: usize,
    depth: usize,
}

impl Position {
    pub fn default() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }

    pub fn summary(self) -> usize {
        self.horizontal * self.depth
    }

    pub fn move_to(self, instruction: &Instruction) -> Self {
        match instruction.direction {
            Direction::Forward => Self {
                horizontal: self.horizontal + instruction.units,
                depth: self.depth,
            },
            Direction::Up => Self {
                horizontal: self.horizontal,
                depth: self.depth - instruction.units,
            },
            Direction::Down => Self {
                horizontal: self.horizontal,
                depth: self.depth + instruction.units,
            },
        }
    }
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .flat_map(|l| {
            let mut split = l.split(' ');
            split.next().and_then(|direction| {
                split
                    .next()
                    .map(|units| (direction, units))
                    .and_then(|(direction, units)| {
                        direction.parse().ok().and_then(|direction| {
                            units.parse().ok().map(|units| (direction, units))
                        })
                    })
            })
        })
        .map(|(direction, units)| Instruction { direction, units }) // put the value tuples into a struct
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> usize {
    let mut pos = Position::default();
    for i in instructions {
        pos = pos.move_to(i)
    }
    pos.summary()
}
