use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let mut sum : u64 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                let midpoint = line.len() / 2;
                let first_comp_substr = &line[..midpoint];
                let first_comp_set : HashSet<char> = first_comp_substr.chars().collect();
                let second_comp_substr = &line[midpoint..];
                let second_comp_set: HashSet<char> = second_comp_substr.chars().collect();
                let common : Vec<&char> = first_comp_set.intersection(&second_comp_set).collect();
                assert_eq!(common.len(), 1);
                sum += to_priority(common[0]);
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

