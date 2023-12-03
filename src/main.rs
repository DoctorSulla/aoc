use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Day one
    puzzle_one();

    puzzle_two();
}

fn puzzle_one() {
    // Puzzle One
    let file = File::open("./input_one.txt").unwrap();
    let mut total: u32 = 0;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        total = total + get_number(line.unwrap());
    }
    println!("The answer for puzzle 1 is {}", total);
}

fn puzzle_two() {
    // Puzzle Two

    let file = File::open("./input_one.txt").unwrap();
    let mut total: u32 = 0;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        total = total + get_number_two(line.unwrap());
    }
    println!("The answer for puzzle 2 is {}", total);
}

fn get_number_two(line: String) -> u32 {
    let values: Vec<&str> = Vec::from([
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]);
    let subs: Vec<&str> = Vec::from(["1", "2", "3", "4", "5", "6", "7", "8", "9"]);
    // From front
    let mut i = 1;
    let mut first_num = 0;
    let mut last_num = 0;
    while i > line.len() {
        let slice = &line[0..i];
        let char = &line[i - 1..i];
        if char.parse::<u8>().is_ok() {
            first_num = char.parse().unwrap();
            break;
        }

        i += 1;
    }
    22
}

fn get_number(line: String) -> u32 {
    let regex = Regex::new("[0-9]").unwrap();
    let matches: Vec<_> = regex.find_iter(&line).collect();
    let first = matches[0].as_str();
    let last = matches[matches.len() - 1].as_str();
    let number: u32 = format!("{}{}", first, last).parse().unwrap();
    number
}
