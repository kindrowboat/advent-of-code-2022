use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut machine = Machine::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                machine.process_instruction(&line);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Machine {
    cycle: isize,
    x: isize,
    line: String,
}

impl Machine {
    pub fn new() -> Machine {
        return Machine {
            cycle: 0,
            x: 1,
            line: "".to_string(),
        }
    }
    pub fn process_instruction(&mut self, instruction: &str) {
        if instruction == "noop" {
            self.tick();
        } else {
            let incr: isize = instruction.split_whitespace().last().unwrap().parse().unwrap();
            self.tick();
            self.tick();
            self.x += incr;
        }
    }
    pub fn tick(&mut self) {
        let mut pixel = ' ';
        if self.cycle >= self.x - 1 && self.cycle <= self.x + 1 {
            pixel = '#';
        } else {
            pixel = '.';
        }
        self.line.push(pixel);
        self.cycle += 1;
        if self.cycle == 40 {
            println!("{}", self.line);
            self.cycle = 0;
            self.line = "".to_string();
        }
    }
}
