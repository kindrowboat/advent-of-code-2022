use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut rps = RPS::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                if line.len() == 3 {
                    let mine = parse_my_play(line.chars().nth(2).unwrap());
                    let theirs = parse_their_play(line.chars().nth(0).unwrap());
                    rps.play_round(mine, theirs);
                } else if line.len() != 0 {
                    panic!("Invalid line");
                }
            }
        }
    }
    println!("{}", rps.score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub enum Play {
    Rock,
    Paper,
    Scissors
}

fn parse_their_play(play: char) -> Play {
    match play {
        'A' => return Play::Rock,
        'B' => return Play::Paper,
        'C' => return Play::Scissors,
        _ => panic!("Invalid move"),
    }
}

fn parse_my_play(play: char) -> Play {
    match play {
        'X' => return Play::Rock,
        'Y' => return Play::Paper,
        'Z' => return Play::Scissors,
        _ => panic!("Invalid move"),
    }
}

pub struct RPS {
    score: u32,
}

impl RPS {
    pub fn new() -> RPS {
        RPS {
            score: 0,
        }
    }
    pub fn play_round(&mut self, mine: Play, theirs: Play) {
        self.score += get_score(mine, theirs);
    }
}

fn get_score(mine: Play, theirs: Play) -> u32 {
    let mut score = 0;
    match mine {
        Play::Rock => score += 1,
        Play::Paper => score += 2,
        Play::Scissors => score += 3,
    }
    let result = get_result(mine, theirs);
    match result {
        Result::Win => score += 6,
        Result::Draw => score += 3,
        Result::Lose => score += 0,
    }
    score
}

enum Result {
    Lose,
    Draw,
    Win,
}

fn get_result(mine: Play, theirs: Play) -> Result {
    match mine {
        Play::Rock => match theirs {
            Play::Rock => Result::Draw,
            Play::Paper => Result::Lose,
            Play::Scissors => Result::Win,
        },
        Play::Paper => match theirs {
            Play::Rock => Result::Win,
            Play::Paper => Result::Draw,
            Play::Scissors => Result::Lose,
        },
        Play::Scissors => match theirs {
            Play::Rock => Result::Lose,
            Play::Paper => Result::Win,
            Play::Scissors => Result::Draw,
        },
    }
}

