use std::{collections::HashMap, ops::RangeBounds};

const FILE: &str = "src/day4/input.txt";

pub struct Board {
    marked: Vec<u8>,
    data: Vec<Vec<u8>>,
    bingo: bool
}

impl Board {
    pub fn new() -> Self {
        Self {
            marked: Vec::new(),
            data: Vec::new(),
            bingo: false
        }
    }

    fn get_col(&self, col: usize) -> Vec<u8> {
        self.data.iter().map(|row| row[col]).collect()
    }

    fn get_row(&self, row: usize) -> Vec<u8> {
        self.data[row].clone()
    }

    pub fn mark(&mut self, value: u8) -> bool {
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if self.data[y][x] != value {
                    continue;
                }

                self.marked.push(value);
                if self.get_col(x).iter().all(|n| self.marked.contains(n)) {
                    self.bingo = true;
                    return true;
                }
                if self.get_row(y).iter().all(|n| self.marked.contains(n)) {
                    self.bingo = true;
                    return true;
                }
                break;
            }
        }
        false
    }

    pub fn score(&self, last_chosen_number: u8) -> u32 {
        let unmarked_sum: u32 = self.data
            .iter()
            .flat_map(|row| row.iter().filter(|n| !self.marked.contains(n)).map(|n| *n as u32).collect::<Vec<u32>>())
            .sum();

        unmarked_sum * last_chosen_number as u32
    }
}

fn parse_boards() -> (Vec<u8>, Vec<Board>) {
    let lines = crate::shared::read_input(FILE);

    let mut lines_iter = lines.iter();

    let chosen_numbers: Vec<u8> = lines_iter.next()
        .unwrap()
        .split(',')
        .map(|n| u8::from_str_radix(n, 10).unwrap())
        .collect();

    let mut boards = Vec::new();

    // skip the first empty line
    lines_iter.next();

    let mut current_board = Board::new();

    while let Some(line) = lines_iter.next() {
        if line.trim().is_empty() {
            boards.push(current_board);
            current_board = Board::new();
            continue;
        }

        let row = line.split_whitespace().map(|n| u8::from_str_radix(n, 10).unwrap()).collect();
        current_board.data.push(row);
    }
    boards.push(current_board);
    (chosen_numbers, boards)
}

pub fn puzzle1() {
    let (chosen_numbers, mut boards) = parse_boards();

    for chosen_number in chosen_numbers {
        for board in &mut boards {
            if board.mark(chosen_number) {
                let score = board.score(chosen_number);
                println!("Puzzle 1: {}", score);
                return
            }
        }
    }
}

pub fn puzzle2() {
    let (chosen_numbers, mut boards) = parse_boards();

    let mut board_count = 0;
    let max_boards = boards.len();

    for chosen_number in chosen_numbers {
        for board in &mut boards {
            let board_bingo = board.bingo;
            if board.mark(chosen_number) {
                if board_bingo != board.bingo {
                    board_count += 1;
                }
            }

            if board_count == max_boards {
                let score = board.score(chosen_number);
                println!("Puzzle 2: {}", score);
                return
            }
        }
    }
}

