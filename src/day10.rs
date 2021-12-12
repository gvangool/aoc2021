use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use std::collections::HashMap;

enum SyntaxError {
    Corrupted(char),
    Incomplete(usize),
}

lazy_static! {
    static ref CHAR_MAP: HashMap<char, char> = {
        let mut m: HashMap<char, char> = HashMap::new();
        m.insert('(', ')');
        m.insert('{', '}');
        m.insert('[', ']');
        m.insert('<', '>');
        m
    };
}

fn syntax_error_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn syntax_incomplete_score(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!(),
    }
}

fn check_syntax(line: &str) -> Result<(), SyntaxError> {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else {
            let last = stack.pop().unwrap();
            let endc = CHAR_MAP[&last];
            if endc != c {
                return Err(SyntaxError::Corrupted(c));
            }
        }
    }
    if stack.len() > 0 {
        let mut score: usize = 0;
        while stack.len() > 0 {
            let last = stack.pop().unwrap();
            let endc = CHAR_MAP[&last];
            score = score * 5 + syntax_incomplete_score(endc);
        }
        return Err(SyntaxError::Incomplete(score));
    }
    Ok(())
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut score = 0;
    for line in input.lines() {
        match check_syntax(line) {
            Err(SyntaxError::Corrupted(c)) => {
                score += syntax_error_score(c);
            }
            _ => continue,
        }
    }
    score
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut all_scores: Vec<usize> = Vec::new();
    for line in input.lines() {
        match check_syntax(line) {
            Err(SyntaxError::Incomplete(score)) => {
                all_scores.push(score);
            }
            _ => continue,
        }
    }
    all_scores.sort();
    all_scores[all_scores.len() / 2]
}
