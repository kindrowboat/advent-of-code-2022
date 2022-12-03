use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let mut sum : u64 = 0;
    let mut index = 0;
    let mut bags = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                let bag : HashSet<char> = line.chars().collect();
                bags.push(bag);
                index += 1;
                if index == 3 {
                    index = 0;
                    let mut common = bags.pop().unwrap();
                    while let Some(bag) = bags.pop() {
                        common = common.intersection(&bag).cloned().collect();
                    }
                    assert_eq!(common.len(), 1);
                    let item = common.iter().next().unwrap();
                    sum += to_priority(item);
                }
            }
        }
    }
    println!("{}", sum);
}

fn to_priority(thing: &char) -> u64 {
    if thing.is_ascii_lowercase() {
        return *thing as u64 - 96;
    } else if thing.is_ascii_uppercase() {
        return *thing as u64 - 38;
    } else {
        panic!("Not an ascii letter.");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

