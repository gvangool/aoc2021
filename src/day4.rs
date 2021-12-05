use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Board {
    board: Vec<Vec<usize>>,
    found: Vec<Vec<bool>>,
    won: bool,
}

impl Board {
    fn new(board: Vec<Vec<usize>>) -> Board {
        let num_rows = board.len();
        let num_cols = board[0].len();
        Board {
            board: board,
            found: vec![vec![false; num_cols]; num_rows],
            won: false,
        }
    }

    fn mark(&mut self, number: usize) -> bool {
        for (x, row) in self.board.iter().enumerate() {
            match row.iter().position(|&value| value == number) {
                Some(y) => {
                    //println!("{} == {} -> [{}][{}]", number, self.board[x][y], x, y);
                    self.found[x][y] = true;
                    return true;
                }
                _ => continue,
            }
        }
        false
    }

    fn unmarked_sum(&self) -> usize {
        let values: Vec<&usize> = self.board.iter().flatten().collect();
        let found: Vec<&bool> = self.found.iter().flatten().collect();
        found
            .iter()
            .zip(values)
            .filter(|(&found, _)| !found)
            .map(|(_, &value)| value)
            .sum()
    }

    fn compute_win(&mut self) -> Option<usize> {
        for row_found in self.found.iter() {
            if row_found.iter().all(|&b| b) {
                self.won = true;
                return Some(self.unmarked_sum());
            }
        }
        let row_len = self.found.len();
        let col_len = self.found[0].len();

        for y in 0..col_len {
            let mut column_found: Vec<bool> = Vec::new();
            for x in 0..row_len {
                column_found.push(self.found[x][y]);
            }
            if column_found.iter().all(|&b| b) {
                self.won = true;
                return Some(self.unmarked_sum());
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Bingo {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Bingo {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    // empty line between numbers and boards
    lines.next();

    let mut boards: Vec<Board> = Vec::new();
    let mut buffer: Vec<Vec<usize>> = Vec::new();

    for line in lines.into_iter() {
        // an empty line separates the boards
        if line.is_empty() {
            boards.push(Board::new(buffer));
            buffer = Vec::new();
        } else {
            let numbers: Vec<usize> = line
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            buffer.push(numbers)
        }
    }
    boards.push(Board::new(buffer));

    Bingo {
        numbers: numbers,
        boards: boards,
    }
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Bingo) -> usize {
    let mut boards = input.boards.clone();
    for &number in input.numbers.iter() {
        for board in boards.iter_mut() {
            board.mark(number);

            match board.compute_win() {
                Some(score) => {
                    println!("{} * {}", score, number);
                    return score * number;
                }
                _ => continue,
            }
        }
    }

    unreachable!();
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Bingo) -> usize {
    let mut boards = input.boards.clone();
    let number_of_boards = input.boards.len();
    for &number in input.numbers.iter() {
        let mut won_boards = boards.iter().filter(|b| b.won).count();
        for board in boards.iter_mut() {
            // if board already completed, we can skip
            if board.won {
                continue;
            }
            board.mark(number);

            match board.compute_win() {
                Some(score) => {
                    // only post the last score
                    won_boards += 1;
                    if won_boards == number_of_boards {
                        return score * number;
                    }
                }
                _ => continue,
            }
        }
    }

    unreachable!();
}
