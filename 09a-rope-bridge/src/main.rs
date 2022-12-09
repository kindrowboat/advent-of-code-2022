use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let mut rope = Rope::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                rope.perform(&line);
            }
        }
    }
    println!("{}", rope.total_tail_visited());
}

type Location = (isize, isize);
#[derive(PartialEq, Copy, Clone)]
pub enum Direction { UP, DOWN, LEFT, RIGHT }

pub struct Rope {
    total_moves: usize,
    head: Location,
    tail: Location,
    tail_visited: HashSet<Location>
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            total_moves: 0,
            head: (0,0),
            tail: (0,0),
            tail_visited: HashSet::<Location>::new(),
        }
    }

    pub fn parse_direction(direction: &str) -> Direction {
        match direction {
            "U" => return Direction::UP,
            "D" => return Direction::DOWN,
            "L" => return Direction::LEFT,
            "R" => return Direction::RIGHT,
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    pub fn perform(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let direction_str = parts[0];
        let steps: usize = parts[1].parse().unwrap();
        let direction = Rope::parse_direction(direction_str);
        for _ in 0..steps {
            self.step(direction);
        }
    }

    pub fn step(&mut self, direction: Direction) {
        match direction {
            Direction::UP => {
                self.head.1 += 1;
                if self.head.1 - self.tail.1 == 2 {
                    self.tail.1 = self.head.1 - 1;
                    self.tail.0 = self.head.0;
                }
            },
            Direction::DOWN => {
                self.head.1 -= 1;
                if self.head.1 - self.tail.1 == -2 {
                    self.tail.1 = self.head.1 + 1;
                    self.tail.0 = self.head.0;
                }
            },
            Direction::RIGHT => {
                self.head.0 += 1;
                if self.head.0 - self.tail.0 == 2 {
                    self.tail.0 = self.head.0 - 1;
                    self.tail.1 = self.head.1;
                }
            },
            Direction::LEFT => {
                self.head.0 -= 1;
                if self.head.0 - self.tail.0 == -2 {
                    self.tail.0 = self.head.0 + 1;
                    self.tail.1 = self.head.1;
                }
            }
        }

        self.total_moves += 1;
        self.tail_visited.insert(self.tail);
    }

    pub fn total_tail_visited(&self) -> usize {
        return self.tail_visited.len();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
