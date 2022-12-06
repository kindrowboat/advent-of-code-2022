use std::fs;
use std::collections::HashSet;

const WINDOW_LENGTH: usize = 14;
type StartOfMessageWindow = [char; WINDOW_LENGTH];

fn main() {
    let mut index = 0;
    let mut window : StartOfMessageWindow = [' '; WINDOW_LENGTH];
    let signal = fs::read_to_string("./input.txt").unwrap();
    for c in signal.chars() {
        window[index%14] = c;
        if index >= 13 {
            if unique(&window) {
                break;
            }
        }
        index += 1;
    }
    println!("{}", index + 1); // answer is expectect with 1 starting index
}

fn unique(window: &StartOfMessageWindow) -> bool {
    let mut hash = HashSet::<&char>::new();
    for item in window {
        if !hash.insert(item) {
            return false
        }
    }
    return true;
}
