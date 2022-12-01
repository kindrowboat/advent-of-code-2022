use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./calorie_list.txt") {
        let mut heap = BinaryHeap::new();
        let mut current_sum = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(count) = line {
                if count.is_empty() {
                    if heap.len() < 3 {
                        heap.push(Reverse(current_sum))
                    } else if current_sum > heap.peek().unwrap().0 {
                        heap.pop();
                        heap.push(Reverse(current_sum))
                    }
                    current_sum = 0;
                } else {
                    current_sum += count.parse::<i32>().unwrap();
                }
            }
        }
        let top_three_total = heap.pop().unwrap().0 + heap.pop().unwrap().0 + heap.pop().unwrap().0;
        println!("{}", top_three_total);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
