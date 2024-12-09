use std::fs;
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        panic!("You must provide one argument (the day number");
    }

    let day: i32 = args[1].parse().expect(format!("Cannot parse day {}", args[1]).as_str());
    let file_path = format!("data/day{day}.txt");
    let data = fs::read_to_string(&file_path).expect(format!("Cannot read file {}", file_path).as_str());

    match day {
        1 => day1::solve(data),    
        2 => day2::solve(data),    
        3 => day3::solve(data),
        4 => day4::solve(data),
        5 => day5::solve(data),
        6 => day6::solve(data),
        7 => day7::solve(data),
        8 => day8::solve(data),
        9 => day9::solve(data),
        day => println!("Invalid day {}", day)
    }
}