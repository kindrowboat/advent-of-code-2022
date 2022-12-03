use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use bitmaps::Bitmap;

fn main() {
    let mut sum : usize = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                let midpoint = line.len() / 2;
                let first_comp_substr = &line[..midpoint];
                let first_comp_bitmap = bitmap_from_string(first_comp_substr);
                let second_comp_substr = &line[midpoint..];
                let second_comp_bitmap = bitmap_from_string(second_comp_substr);
                let common = first_comp_bitmap & second_comp_bitmap;
                assert_eq!(common.len(), 1);
                sum += common.first_index().unwrap();
            }
        }
    }
    println!("{}", sum);
}

fn bitmap_from_string(items: &str) -> Bitmap<128> {
    let mut bitmap = Bitmap::<128>::new();
    for c in items.chars() {
        let priority = to_priority(&c);
        bitmap.set(priority, true);
    }
    return bitmap;
}

fn to_priority(thing: &char) -> usize {
    if thing.is_ascii_lowercase() {
        return *thing as usize - 96;
    } else if thing.is_ascii_uppercase() {
        return *thing as usize - 38;
    } else {
        panic!("Not an ascii letter.");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

