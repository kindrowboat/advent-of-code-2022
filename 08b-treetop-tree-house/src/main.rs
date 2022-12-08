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
    println!("{}", forest.best_score());
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
    pub fn best_score(&self) -> usize {
        let mut max: usize = 0;
        for i in 0..self.trees.len() {
            for j in 0..self.trees[i].len() {
                let score = self.calc_score(&i, &j);
                if score > max {
                    max = score;
                }
            }
        }
        return max;
    }
    pub fn calc_score(&self, row_num: &usize, col_num: &usize) -> usize {
       return self.calc_north_score(row_num, col_num) *
              self.calc_south_score(row_num, col_num) *
              self.calc_east_score(row_num, col_num) *
              self.calc_west_score(row_num, col_num); 
    }
    pub fn calc_north_score(&self, row_num: &usize, col_num: &usize) -> usize {
        if *row_num == 0 {
            return 0;
        }
        let mut score = 0;
        let height = &self.trees[*row_num][*col_num];
        for i in (0..*row_num).rev() {
            score += 1;
            if self.trees[i][*col_num] >= *height {
                break;
            }
        }
        return score;
    }
    pub fn calc_south_score(&self, row_num: &usize, col_num: &usize) -> usize {
        if *row_num == self.trees.len() - 1 {
            return 0;
        }
        let mut score = 0;
        let height = &self.trees[*row_num][*col_num];
        for i in (*row_num+1)..self.trees.len() {
            score += 1;
            if self.trees[i][*col_num] >= *height {
                break;
            }
        }
        return score;
    }
    pub fn calc_east_score(&self, row_num: &usize, col_num: &usize) -> usize {
        if *col_num == 0 {
            return 0;
        }
        let mut score = 0;
        let height = &self.trees[*row_num][*col_num];
        for i in (0..*col_num).rev() {
            score += 1;
            if self.trees[*row_num][i] >= *height {
                break;
            }
        }
        return score;
    }
    pub fn calc_west_score(&self, row_num: &usize, col_num: &usize) -> usize {
        if *col_num == self.trees[*row_num].len() - 1 {
            return 0;
        }
        let mut score = 0;
        let height = &self.trees[*row_num][*col_num];
        for i in (*col_num+1)..self.trees.len() {
            score += 1;
            if self.trees[*row_num][i] >= *height {
                break;
            }
        }
        return score;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

