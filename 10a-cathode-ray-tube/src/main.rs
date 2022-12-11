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
    println!("{}", machine.sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Machine {
    cycle: isize,
    x: isize,
    sum: isize, // sum x at 20th, 60th, 100th, 140th, 180th
}

impl Machine {
    pub fn new() -> Machine {
        return Machine {
            cycle: 1,
            x: 1,
            sum: 0,
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
        if (self.cycle - 20) % 40 == 0 {
            self.sum += self.x * self.cycle;
            println!("------");
            println!("{}", self.x);
            println!("{}", self.cycle);
            println!("{}", self.x * self.cycle);
        }
        self.cycle += 1;
    }
}
