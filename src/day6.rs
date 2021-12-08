use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(lines: &Vec<usize>) -> usize {
    let mut current_state: Vec<usize> = lines.clone();
    let days = 80;

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

#[aoc(day6, part1, fast)]
pub fn solve_part1_same_as_part2(lines: &Vec<usize>) -> u64 {
    play_fish_lives(lines, 80)
}

#[aoc(day6, part2)]
pub fn solve_part2(lines: &Vec<usize>) -> u64 {
    play_fish_lives(lines, 256)
}

pub fn play_fish_lives(fishes: &Vec<usize>, days: usize) -> u64 {
    let mut new_fishes_per_day = vec![0; days];

    let number_of_weeks: usize = days / 7;
    for fish in fishes {
        for i in 0..=number_of_weeks {
            let next_gen = fish + i * 7;
            if next_gen < days {
                new_fishes_per_day[next_gen] += 1;
            }
        }
    }

    for i in 0..days {
        let mut next_gen = i + 9;
        while next_gen < days {
            new_fishes_per_day[next_gen] += new_fishes_per_day[i];
            next_gen += 7;
        }
    }

    let total_new_fishes: u64 = new_fishes_per_day.iter().sum();
    total_new_fishes + fishes.len() as u64
}
