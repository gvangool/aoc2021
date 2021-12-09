use aoc_runner_derive::{aoc, aoc_generator};

/*
#[aoc_generator(day8)]
pub fn parse(input: &str) -> Vec<usize> {
    // INPUT | OUTPUT
    // fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    input.lines().split('|').map(|x| x.split(' ').collect).collect()
}
*/

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut easy_numbers = 0;
    for line in input.lines() {
        let mut input_output = line.split('|');
        let _input = input_output.next().unwrap();
        let output = input_output.next().unwrap();
        println!("{}",output);
        for number_of_segments in output.split(' ').map(|x| x.len()) {
            match number_of_segments {
                2 | 3 | 4 | 7 => easy_numbers += 1,
                _ => continue,
            };
        }
    }
    easy_numbers
}
