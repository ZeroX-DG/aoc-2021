use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE: &str = "src/day2/input.txt";

fn read_input() -> Vec<String> {
    let file = File::open(FILE).expect("Unable to read input file");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Unable to read line"))
        .collect()
}

pub fn puzzle1() {
    let lines = read_input();

    let mut x: u32 = 0;
    let mut depth: u32 = 0;

    for line in lines {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let action = parts[0];
        let value = u32::from_str_radix(parts[1], 10).expect("Unable to parse value");

        match action {
            "up" => depth -= value,
            "down" => depth += value,
            "forward" => x += value,
            _ => {}
        }
    }
    println!("Puzzle 1: {}", x * depth);
}

pub fn puzzle2() {
    let lines = read_input();

    let mut x: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;

    for line in lines {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let action = parts[0];
        let value = usize::from_str_radix(parts[1], 10).expect("Unable to parse value");

        match action {
            "up" => {
                aim -= value;
            },
            "down" => {
                aim += value;
            }
            "forward" => {
                depth += aim * value;
                x += value;
            }
            _ => {}
        }
    }
    println!("Puzzle 2: {}", x * depth);
}

