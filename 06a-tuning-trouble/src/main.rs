use std::fs;

fn main() {
    let mut index = 0;
    let mut w : [char; 4] = [' '; 4]; // w is for window
    let signal = fs::read_to_string("./input.txt").unwrap();
    for c in signal.chars() {
        w[index%4] = c;
        if index >= 3 && w[0] != w[1] && w[0] != w[2] && w[0] != w[3] && w[1] != w[2] && w[1] != w[3] && w[2] != w[3] {
            break;
        }
        index += 1;
    }
    println!("{}", index + 1); // answer is expectect with 1 starting index
}
