use std::fs::File;
use std::io::{self, BufRead};

pub fn puzzle_one() {
    let file = File::open("./input_day_two.txt").unwrap();
    let mut total: u32 = 0;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        total += get_game_score(line.unwrap());
    }
    println!("The solution for puzzle 1 day two is {}", total);
}

pub fn puzzle_two() {
    let file = File::open("./input_day_two.txt").unwrap();
    let mut total: u32 = 0;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        total += get_game_power(line.unwrap());
    }
    println!("The solution for puzzle 2 day two is {}", total);
}

fn get_game_score(game: String) -> u32 {
    let tred = 12;
    let tgreen = 13;
    let tblue = 14;

    let mut blue = 0;
    let mut green = 0;
    let mut red = 0;

    let parts = game.split(":").collect::<Vec<&str>>();
    let game_id: u32 = parts[0].split(" ").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    let draws = parts[1].split(";").collect::<Vec<&str>>();
    for draw in draws {
        let balls = draw.split(",").collect::<Vec<&str>>();
        for ball in balls {
            if ball.contains("red") {
                red = ball.trim().split(" ").collect::<Vec<&str>>()[0]
                    .parse()
                    .unwrap();
            }
            if ball.contains("green") {
                green = ball.trim().split(" ").collect::<Vec<&str>>()[0]
                    .parse()
                    .unwrap();
            }
            if ball.contains("blue") {
                blue = ball.trim().split(" ").collect::<Vec<&str>>()[0]
                    .parse()
                    .unwrap();
            }
        }
        if red > tred || blue > tblue || green > tgreen {
            return 0;
        }
    }
    game_id
}

fn get_game_power(game: String) -> u32 {
    let mut blue;
    let mut green;
    let mut red;

    let mut blue_required = 0;
    let mut green_required = 0;
    let mut red_required = 0;

    let parts = game.split(":").collect::<Vec<&str>>();

    let draws = parts[1].split(";").collect::<Vec<&str>>();
    for draw in draws {
        let balls = draw.split(",").collect::<Vec<&str>>();
        for ball in balls {
            if ball.contains("red") {
                red = ball.trim().split(" ").collect::<Vec<&str>>()[0]
                    .parse()
                    .unwrap();
                if red > red_required {
                    red_required = red
                }
            }
            if ball.contains("green") {
                green = ball.trim().split(" ").collect::<Vec<&str>>()[0]
                    .parse()
                    .unwrap();
                if green > green_required {
                    green_required = green
                }
            }
            if ball.contains("blue") {
                blue = ball.trim().split(" ").collect::<Vec<&str>>()[0]
                    .parse()
                    .unwrap();
                if blue > blue_required {
                    blue_required = blue
                }
            }
        }
    }
    blue_required * red_required * green_required
}
