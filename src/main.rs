use std::fs;
use std::env;

mod day1;
mod day2;


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
        day => println!("Invalid day {}", day)
    }
}