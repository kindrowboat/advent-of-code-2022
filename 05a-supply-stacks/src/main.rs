use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;
use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let mut stack_number = 1;
    let mut stacks = Stacks::new();
    if let Ok(lines) = read_lines("./initial_stacks.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                stacks.add_stack(stack_number, &line);
                stack_number += 1;
            }
        }
    }
    if let Ok(lines) = read_lines("./instructions.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                stacks.process_instruction(&line);
            }
        }
    }

    let s: String = stacks.top_items().iter().cloned().collect();
    println!("{}", s);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct Stacks {
    map : BTreeMap<usize, Stack>,
}

type Stack = Vec<char>;

impl Stacks {

    pub fn new() -> Stacks {
        Stacks {
            map: BTreeMap::<usize, Stack>::new(),
        }
    }
    pub fn top_items(&self) -> Vec<&char> {
        let mut tops = Vec::<&char>::new();
        for stack in self.map.values() {
            tops.push(stack.last().unwrap_or(&' '));
        }
        return tops;
    }
    pub fn add_stack(&mut self, stack_number : usize, stack_str : &str) {
        self.map.insert(stack_number, stack_str.chars().collect());
    }

    pub fn process_instruction(&mut self, instruction : &str) {
        lazy_static! {
            static ref RE : Regex = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
        }
        let caps = RE.captures(instruction).unwrap();
        let count : usize = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
        let from_stack : usize = caps.get(2).map_or("", |m| m.as_str()).parse().unwrap();
        let to_stack : usize = caps.get(3).map_or("", |m| m.as_str()).parse().unwrap();
        for _i in 1..=count {
            let item = self.map.get_mut(&from_stack).unwrap().pop().unwrap();
            self.map.get_mut(&to_stack).unwrap().push(item);
        }
    }
}

