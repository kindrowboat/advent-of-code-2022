use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                if is_any_overlap(&line) {
                    sum += 1;
                }
            }
        }
    }
    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

type Range = (usize, usize);
type RangePair = (Range, Range);

fn parse_ranges(line: &str) -> RangePair {
    let ranges : Vec<&str> = line.split(",").collect();
    assert_eq!(ranges.len(), 2);
    return (parse_range(ranges[0]), parse_range(ranges[1]));
}

fn parse_range(range_str: &str) -> Range {
    let points : Vec<&str> = range_str.split("-").collect();
    assert_eq!(points.len(), 2);
    let start : usize = points[0].parse().unwrap();
    let end : usize = points[1].parse().unwrap();
    return (start, end);
}

fn is_any_overlap(line: &str) -> bool {
    let range_pair = parse_ranges(line);
    (range_pair.0.0 <= range_pair.1.0 && range_pair.0.1 >= range_pair.1.0 ) ||
    (range_pair.1.0 <= range_pair.0.0 && range_pair.1.1 >= range_pair.0.0 )
}
