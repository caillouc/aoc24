use std::fs;

mod day1;


fn main() {
    let data = fs::read_to_string("data/day1.txt").unwrap();
    day1::solve(data);
}