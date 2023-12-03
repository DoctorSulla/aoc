use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

static DEBUG: bool = false;

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
        total += get_number_two(line.unwrap());
    }
    println!("The answer for puzzle 2 is {}", total);
}

fn get_number_two(line: String) -> u32 {
    let values: Vec<&str> = Vec::from([
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]);
    let subs: Vec<&str> = Vec::from(["1", "2", "3", "4", "5", "6", "7", "8", "9"]);
    let mut i = 1;
    let mut found = false;
    let mut first_num = 0;
    let mut last_num = 0;
    // From front
    while i <= line.len() && !found {
        let slice = &line[0..i];
        let char = &line[i - 1..i];
        if DEBUG {
            println!("The char is {} and the slice is {}", char, slice);
        }
        if char.parse::<u8>().is_ok() {
            first_num = char.parse().unwrap();
            break;
        }
        let mut j = 0;
        while j < values.len() {
            let check = slice.replace(values[j], subs[j]);
            if check != slice {
                first_num = subs[j].parse().unwrap();
                found = true;
                break;
            }
            j += 1;
        }

        i += 1;
    }
    // From back
    i = 1;
    found = false;
    while i <= line.len() && !found {
        let slice = &line[line.len() - i..line.len()];
        let char = &line[line.len() - i..cmp::min(line.len() - i + 1, line.len())];
        if DEBUG {
            println!("The char is {} and the slice is {}", char, slice);
        }
        if char.parse::<u8>().is_ok() {
            last_num = char.parse().unwrap();
            break;
        }
        let mut j = 0;
        while j < values.len() {
            let check = slice.replace(values[j], subs[j]);
            if check != slice {
                last_num = subs[j].parse().unwrap();
                found = true;
                break;
            }
            j += 1;
        }

        i += 1;
    }
    if DEBUG {
        println!("The total is {}", first_num * 10 + last_num);
    }
    first_num * 10 + last_num
}

fn get_number(line: String) -> u32 {
    let regex = Regex::new("[0-9]").unwrap();
    let matches: Vec<_> = regex.find_iter(&line).collect();
    let first = matches[0].as_str();
    let last = matches[matches.len() - 1].as_str();
    let number: u32 = format!("{}{}", first, last).parse().unwrap();
    number
}
