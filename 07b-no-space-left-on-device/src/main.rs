use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut dirs = Vec::<usize>::new();
    let mut stack = Vec::<usize>::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                if line.is_empty() || line == "$ ls" || line.starts_with("dir") {
                    continue
                } else if line == "$ cd .." {
                    stack.pop();
                } else if line.starts_with("$ cd") {
                    dirs.push(0);
                    stack.push(dirs.len() - 1);
                } else if line.chars().next().unwrap().is_numeric() {
                    let size: usize = line.split_whitespace().next().unwrap().parse().unwrap();
                    for index in &stack {
                        let old_size = dirs[*index];
                        dirs[*index] = old_size + size;
                    }
                } else {
                    panic!("Unrecognized command: {}", line);
                }
            }
        }
    }

    let total_used_space = dirs[0];
    let free_space = 70000000 - total_used_space;
    let needed_space = 30000000 - free_space;
    let mut smallest_sufficient_folder = total_used_space;
    for dir_size in dirs {
        if dir_size >= needed_space && dir_size < smallest_sufficient_folder {
            smallest_sufficient_folder = dir_size
        }
    }
    println!("{}", smallest_sufficient_folder);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

