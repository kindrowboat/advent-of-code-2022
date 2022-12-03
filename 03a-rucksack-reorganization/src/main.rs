use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                println!("{}", line);
                for c in line.chars() {
                    println!("{}", to_priority(c));
                }
            }
        }
    }
    
}

fn to_priority(thing: char) -> u8 {
    if thing.is_ascii_lowercase() {
        return thing as u8 - 96;
    } else if thing.is_ascii_uppercase() {
        return thing as u8 - 38;
    } else {
        panic!("Not an ascii letter.");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

