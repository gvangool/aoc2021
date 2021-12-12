use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use std::collections::HashMap;

enum SyntaxError {
    Corrupted(char),
    Incomplete,
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
