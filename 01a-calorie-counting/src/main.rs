use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./calorie_list.txt") {
        let mut max = 0;
        let mut current_sum = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(count) = line {
                if count.is_empty() {
                    if current_sum > max {
                        max = current_sum;
                    }
                    current_sum = 0;
                } else {
                    current_sum += count.parse::<i32>().unwrap();
                }
            }
        }
        println!("{}", max);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
