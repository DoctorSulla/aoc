use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./input_one.txt").unwrap();
    let mut total:u32 = 0;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        total = total + get_number(line.unwrap());
    }
    println!("The answer for day 1 is {}",total);
}

fn get_number(line: String) -> u32 {
    let regex = Regex::new("[0-9]").unwrap();
    let matches: Vec<_> = regex.find_iter(&line).collect();
    let first = matches[0].as_str();
    let last = matches[matches.len()-1].as_str();
    let number: u32 = format!("{}{}",first,last).parse().unwrap();
    number
}
