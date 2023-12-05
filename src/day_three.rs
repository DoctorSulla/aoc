use std::fs::File;
use std::io::{self, BufRead};

pub fn puzzle_one() {
    let file = File::open("./input_day_three.txt").unwrap();
    let mut total: u32 = 0;
    let lines = io::BufReader::new(file).lines();

    let mut lines_vec: Vec<Vec<char>> = Vec::new();

    // Build a 2D vector to store all the lines and their chars
    for line in lines {
        let chars = line.unwrap().chars().collect::<Vec<char>>();
        lines_vec.push(chars);
    }

    // Work our way through line until we find a number
    for (line_index, line) in lines_vec.iter().enumerate() {
        //let mut line_total = 0;
        let mut number_string = String::new();
        let mut number_starting_index: usize = 0;
        let mut number_finishing_index: usize;
        let mut capture = false;
        for (char_index, char) in line.iter().enumerate() {
            if char.is_digit(10) && !capture {
                number_starting_index = char_index;
                number_string.push(char.clone());
                capture = true;
            } else if (!char.is_digit(10) || char_index == line.len() - 1) && capture {
                if !char.is_digit(10) {
                    number_finishing_index = char_index - 1;
                } else {
                    number_finishing_index = char_index;
                    number_string.push(char.clone());
                }
                let mut valid_number = false;
                // Check same line
                if number_starting_index == 0 {
                    if line[number_finishing_index + 1] != '.' {
                        valid_number = true;
                    }
                } else if number_starting_index == line.len() - 1 {
                    if line[number_starting_index - 1] != '.' {
                        valid_number = true;
                    }
                } else {
                    if number_finishing_index == line.len() - 1 {
                        if line[number_starting_index - 1] != '.' {
                            valid_number = true;
                        }
                    } else {
                        if line[number_starting_index - 1] != '.' {
                            valid_number = true;
                        }
                        if line[number_finishing_index + 1] != '.' {
                            valid_number = true;
                        }
                    }
                }
                // Check line below
                if !valid_number && line_index == 0 {
                    valid_number = check_line(
                        lines_vec[line_index + 1].clone(),
                        number_starting_index,
                        number_finishing_index,
                    );
                // Check line above
                } else if !valid_number && line_index == lines_vec.len() - 1 {
                    valid_number = check_line(
                        lines_vec[line_index - 1].clone(),
                        number_starting_index,
                        number_finishing_index,
                    );
                }
                // Check both
                if !valid_number && line_index != 0 && line_index != lines_vec.len() - 1 {
                    valid_number = check_line(
                        lines_vec[line_index - 1].clone(),
                        number_starting_index,
                        number_finishing_index,
                    ) || check_line(
                        lines_vec[line_index + 1].clone(),
                        number_starting_index,
                        number_finishing_index,
                    );
                }

                if valid_number {
                    // println!(
                    //     "Adding {} to the total from line {}",
                    //     number_string.parse::<u32>().unwrap(),
                    //     line_index + 1
                    // );

                    total += number_string.parse::<u32>().unwrap();
                    //line_total += number_string.parse::<u32>().unwrap();
                } else {
                    // println!(
                    //     "Not adding {} to the total from line {}",
                    //     number_string,
                    //     line_index + 1
                    // );
                }
                number_string = String::from("");
                capture = false;
            } else if char.is_digit(10) && capture {
                number_string.push(char.clone());
            }
        }
        //println!("{}", line_total);
    }

    println!("The solution for puzzle 1 day three is {}", total);
}

pub fn puzzle_two() {
    let file = File::open("./input_day_three.txt").unwrap();
    let mut total: u32 = 0;
    let lines = io::BufReader::new(file).lines();

    let mut lines_vec: Vec<Vec<char>> = Vec::new();

    // Build a 2D vector to store all the lines and their chars
    for line in lines {
        let chars = line.unwrap().chars().collect::<Vec<char>>();
        lines_vec.push(chars);
    }

    for (line_index, line) in lines_vec.iter().enumerate() {
        for (char_index, char) in line.iter().enumerate() {
            if char.clone() == '*' {
                let mut first_number_string = String::new();
                let mut second_number_string = String::new();
                // Look left
                if char_index == 0 {
                } else {
                    if first_number_string.len() == 0 {
                        let mut i = 1;
                        while char_index - i >= 0 && line[char_index - i].is_digit(10) {
                            first_number_string.push(line[char_index - i]);
                            i += 1;
                        }
                        first_number_string = first_number_string.chars().rev().collect::<String>();
                    } else {
                        let mut i = 1;
                        while char_index - i >= 0 && line[char_index - i].is_digit(10) {
                            second_number_string.push(line[char_index - i]);
                            i += 1;
                        }
                        second_number_string =
                            second_number_string.chars().rev().collect::<String>();
                    }
                }
                // Look right
                if char_index == line.len() - 1 {
                } else {
                    if first_number_string.len() == 0 {
                        let mut i = 1;
                        while char_index + i < line.len() && line[char_index + i].is_digit(10) {
                            first_number_string.push(line[char_index + i]);
                            i += 1;
                        }
                    } else {
                        let mut i = 1;
                        while char_index + i < line.len() && line[char_index + i].is_digit(10) {
                            second_number_string.push(line[char_index + i]);
                            i += 1;
                        }
                    }
                }
                // Look up
                // Look down
                // Look top left
                // Look top right
                // Look bottom left
                // Look bottom right
                if first_number_string.len() > 0 && second_number_string.len() > 0 {
                    println!("{} * {}", first_number_string, second_number_string);
                    total += first_number_string.parse::<u32>().unwrap()
                        * second_number_string.parse::<u32>().unwrap();
                }
            }
        }
    }
    println!("The solution for puzzle 2 day three is {}", total);
}

fn check_line(line: Vec<char>, start_index: usize, finish_index: usize) -> bool {
    let mut start: usize;
    let finish: usize;
    if start_index == 0 {
        start = 0;
    } else {
        start = start_index - 1;
    }

    if finish_index == line.len() - 1 {
        finish = line.len() - 1;
    } else {
        finish = finish_index + 1;
    }

    //println!("Starting at {} and finishing at {}", start, finish);

    while start <= finish {
        //println!("{}", line[start]);
        if !line[start].is_digit(10) && line[start] != '.' {
            return true;
        }
        start += 1;
    }
    return false;
}
