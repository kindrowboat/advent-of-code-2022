use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut rps = RPS::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                if line.len() == 3 {
                    let theirs = parse_their_play(line.chars().nth(0).unwrap());
                    let needed_result = parse_needed_result(line.chars().nth(2).unwrap());
                    let needed_play = calc_needed_play(&theirs, needed_result);
                    rps.play_round(needed_play, theirs);
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

#[derive(Clone, Copy)]
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

fn parse_needed_result(result: char) -> Result {
    match result {
        'X' => return Result::Lose,
        'Y' => return Result::Draw,
        'Z' => return Result::Win,
        _ => panic!("Invalid move"),
    }
}

fn calc_needed_play(theirs: &Play, desired: Result) -> Play {
    match desired {
        Result::Win => match theirs {
            Play::Rock => return Play::Paper,
            Play::Paper => return Play::Scissors,
            Play::Scissors => return Play::Rock,
        },
        Result::Lose => match theirs {
            Play::Rock => return Play::Scissors,
            Play::Paper => return Play::Rock,
            Play::Scissors => return Play::Paper,
        },
        Result::Draw => return *theirs,
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

