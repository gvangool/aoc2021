use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(lines: &Vec<usize>) -> usize {
    play_fish_lives(lines, 80)
}

pub fn play_fish_lives(fishes: &Vec<usize>, days: usize) -> usize {
    let mut current_state: Vec<usize> = fishes.clone();

    for _ in 0..days {
        let mut new_state = Vec::new();

        for fish in current_state {
            match fish {
                0 => {
                    new_state.push(6);
                    new_state.push(8);
                }
                i => new_state.push(i - 1),
            }
        }

        current_state = new_state;
    }

    current_state.iter().len()
}
