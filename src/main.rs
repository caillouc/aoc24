use std::env;
use std::fs;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod parser_helper;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("You must provide one argument (the day number");
    }

    let day = &args[1];
    let file_path = format!("data/{day}.txt");
    let data =
        fs::read_to_string(&file_path).expect(format!("Cannot read file {}", file_path).as_str());

    match day.as_str() {
        "day1" => day1::solve(data),
        "day2" => day2::solve(data),
        "day3" => day3::solve(data),
        "day4" => day4::solve(data),
        "day5" => day5::solve(data),
        "day6" => day6::solve(data),
        "day7" => day7::solve(data),
        "day8" => day8::solve(data),
        "day9" => day9::solve(data),
        "day10" => day10::solve(data),
        "day11" => day11::solve(data),
        "day12" => day12::solve(data),
        "day13" => day13::solve(data),
        "day14" => day14::solve(data),
        "day15" => day15::solve(data),
        "day16" => day16::solve(data),
        day => println!("Invalid day {}", day),
    }
}
