use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut forest = Forest::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                forest.append_row(&line);
            }
        }
    }
    println!("{}", forest.num_visible());
}

pub struct Forest {
    trees: Vec<Vec<u32>>,
}

impl Forest {
    pub fn new() -> Forest {
        Forest {
            trees: Vec::<Vec<u32>>::new(),
        }
    }
    pub fn append_row(&mut self, line: &str) {
        let row : Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        self.trees.push(row);
    }
    pub fn num_visible(&self) -> usize {
        let mut count: usize = 0;
        for i in 0..self.trees.len() {
            for j in 0..self.trees[i].len() {
                if self.is_visible(&i, &j) {
                    count += 1;
                }
            }
        }
        return count;
    }
    pub fn is_visible(&self, row_num: &usize, col_num: &usize) -> bool {
       return self.is_visible_from_north(row_num, col_num) ||
              self.is_visible_from_south(row_num, col_num) ||
              self.is_visible_from_east(row_num, col_num) ||
              self.is_visible_from_west(row_num, col_num); 
    }
    pub fn is_visible_from_north(&self, row_num: &usize, col_num: &usize) -> bool {
        if *row_num == 0 {
            return true;
        }
        let height = &self.trees[*row_num][*col_num];
        for i in 0..*row_num {
            if self.trees[i][*col_num] >= *height {
                return false;
            }
        }
        return true;
    }
    pub fn is_visible_from_south(&self, row_num: &usize, col_num: &usize) -> bool {
        if *row_num == self.trees.len() - 1 {
            return true;
        }
        let height = &self.trees[*row_num][*col_num];
        for i in (*row_num+1)..self.trees.len() {
            if self.trees[i][*col_num] >= *height {
                return false;
            }
        }
        return true;
    }
    pub fn is_visible_from_east(&self, row_num: &usize, col_num: &usize) -> bool {
        if *col_num == 0 {
            return true;
        }
        let height = &self.trees[*row_num][*col_num];
        for i in 0..*col_num {
            if self.trees[*row_num][i] >= *height {
                return false;
            }
        }
        return true;
    }
    pub fn is_visible_from_west(&self, row_num: &usize, col_num: &usize) -> bool {
        if *col_num == self.trees[*row_num].len() - 1 {
            return true;
        }
        let height = &self.trees[*row_num][*col_num];
        for i in (*col_num+1)..self.trees.len() {
            if self.trees[*row_num][i] >= *height {
                return false;
            }
        }
        return true;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

