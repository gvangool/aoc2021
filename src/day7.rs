use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .flat_map(|s| s.parse::<usize>())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<usize>) -> usize {
    let mut numbers: Vec<usize> = input.clone();
    numbers.sort();

    let min_step = numbers[0];
    let max_step = numbers[numbers.len() -1];

    //let mut lowest_step = 0;
    let mut lowest_fuel_cost = usize::MAX;

    for i in min_step..max_step {
        let fuel_cost = numbers.iter().map(|&num| {
            if num < i {
                i - num
            } else {
                num - i
            }
        }).sum();
        if lowest_fuel_cost >  fuel_cost {
            //lowest_step = i;
            lowest_fuel_cost = fuel_cost;
        }
    }
    lowest_fuel_cost
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<usize>) -> usize {
    let mut numbers: Vec<usize> = input.clone();
    numbers.sort();

    let min_step = numbers[0];
    let max_step = numbers[numbers.len() -1];

    let mut lowest_fuel_cost = usize::MAX;

    for i in min_step..max_step {
        let fuel_cost = numbers.iter().map(|&num| {
            let steps = if num < i {
                i - num
            } else {
                num - i
            };
            (1..=steps).fold(0, |acc, x| acc + x)
        }).sum();
        if lowest_fuel_cost >  fuel_cost {
            lowest_fuel_cost = fuel_cost;
        }
    }
    lowest_fuel_cost
}
