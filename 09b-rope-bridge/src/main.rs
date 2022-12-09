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
pub enum Direction { 
    NORTH, 
    NE,
    EAST,
    SE,
    SOUTH,
    SW,
    WEST,
    NW,
}

const NUM_KNOTS: usize = 10;

pub struct Rope {
    knots: [Location; NUM_KNOTS],
    tail_visited: HashSet<Location>
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            knots: [(0,0); NUM_KNOTS],
            tail_visited: HashSet::<Location>::new(),
        }
    }

    pub fn parse_direction(direction: &str) -> Direction {
        match direction {
            "U" => return Direction::NORTH,
            "D" => return Direction::SOUTH,
            "L" => return Direction::WEST,
            "R" => return Direction::EAST,
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    pub fn perform(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let direction_str = parts[0];
        let steps: usize = parts[1].parse().unwrap();
        let direction = Rope::parse_direction(direction_str);
        for _ in 0..steps {
            self.step(0, direction);
            self.tail_visited.insert(self.knots[NUM_KNOTS-1]);
        }
    }

    pub fn step(&mut self, i: usize, direction: Direction) {
        let is_not_tail = i != NUM_KNOTS - 1;
        let next = i+1;
        fn is_touching(me: Location, them: Location) -> bool {
            return (me.0 - them.0).abs() <= 1 && (me.1 - them.1).abs() <= 1;
        }
        match direction {
            Direction::NORTH => {
                self.knots[i].1 += 1;
                if is_not_tail && !is_touching(self.knots[i], self.knots[next]) {
                    if self.knots[i].0 > self.knots[next].0 {
                        self.step(next, Direction::NE)
                    } else if self.knots[i].0 < self.knots[next].0 {
                        self.step(next, Direction::NW)
                    } else {
                        self.step(next, Direction::NORTH)
                    }
                }
            },
            Direction::SOUTH => {
                self.knots[i].1 -= 1;
                if is_not_tail && !is_touching(self.knots[i], self.knots[next]) {
                    if self.knots[i].0 > self.knots[next].0 {
                        self.step(next, Direction::SE)
                    } else if self.knots[i].0 < self.knots[next].0 {
                        self.step(next, Direction::SW)
                    } else {
                        self.step(next, Direction::SOUTH)
                    }
                }
            },
            Direction::EAST => {
                self.knots[i].0 += 1;
                if is_not_tail && !is_touching(self.knots[i], self.knots[next]) {
                    if self.knots[i].1 > self.knots[next].1 {
                        self.step(next, Direction::NE)
                    } else if self.knots[i].1 < self.knots[next].1 {
                        self.step(next, Direction::SE)
                    } else {
                        self.step(next, Direction::EAST)
                    }
                }
            },
            Direction::WEST => {
                self.knots[i].0 -= 1;
                if is_not_tail && !is_touching(self.knots[i], self.knots[next]) {
                    if self.knots[i].1 > self.knots[next].1 {
                        self.step(next, Direction::NW)
                    } else if self.knots[i].1 < self.knots[next].1 {
                        self.step(next, Direction::SW)
                    } else {
                        self.step(next, Direction::WEST)
                    }
                }
            },
            Direction::NE => {
                self.knots[i].0 += 1;
                self.knots[i].1 += 1;
                if is_not_tail && !is_touching(self.knots[i], self.knots[next]) {
                    if self.knots[i].0 == self.knots[next].0 {
                        self.step(next, Direction::NORTH);
                    } else if self.knots[i].1 == self.knots[next].1 {
                        self.step(next, Direction::EAST);
                    } else {
                        self.step(next, Direction::NE);
                    }
                }
            },
            Direction::SE => {
                self.knots[i].0 += 1;
                self.knots[i].1 -= 1;
                if is_not_tail && !is_touching(self.knots[i], self.knots[next]) {
                    if self.knots[i].0 == self.knots[next].0 {
                        self.step(next, Direction::SOUTH);
                    } else if self.knots[i].1 == self.knots[next].1 {
                        self.step(next, Direction::EAST);
                    } else {
                        self.step(next, Direction::SE);
                    }
                }
            },
            Direction::SW => {
                self.knots[i].0 -= 1;
                self.knots[i].1 -= 1;
                if is_not_tail && !is_touching(self.knots[i], self.knots[next]) {
                    if self.knots[i].0 == self.knots[next].0 {
                        self.step(next, Direction::SOUTH);
                    } else if self.knots[i].1 == self.knots[next].1 {
                        self.step(next, Direction::WEST);
                    } else {
                        self.step(next, Direction::SW);
                    }
                }
            },
            Direction::NW => {
                self.knots[i].0 -= 1;
                self.knots[i].1 += 1;
                if is_not_tail && !is_touching(self.knots[i], self.knots[next]) {
                    if self.knots[i].0 == self.knots[next].0 {
                        self.step(next, Direction::NORTH);
                    } else if self.knots[i].1 == self.knots[next].1 {
                        self.step(next, Direction::WEST);
                    } else {
                        self.step(next, Direction::NW);
                    }
                }
            },
        }
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
