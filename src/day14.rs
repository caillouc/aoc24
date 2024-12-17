use std::{collections::HashSet, thread::sleep, time::Duration};

use regex::Regex;

use crate::utils::Position;

pub fn solve(data: String) {
    let robots = parse(data);
    println!("Part one : {}", solve1(&robots));
    println!("Part two : {}", solve2(&robots));
}

#[derive(Debug)]
struct Robot {
    start_position: Position,
    velocity: Position,
}

impl Robot {
    fn evolve(&self, second: i32) -> Position {
        let max_x = 101;
        let max_y = 103;
        Position::from(
            (self.start_position.x() + second * self.velocity.x()).rem_euclid(max_x),
            (self.start_position.y() + second * self.velocity.y()).rem_euclid(max_y),
        )
    }
}

fn parse(data: String) -> Vec<Robot> {
    let mut robots = vec![];
    for l in data.lines() {
        let re = Regex::new(r"p=([0-9]*),([0-9]*) v=(-?[0-9]*),(-?[0-9]*)").unwrap();
        let cap = re.captures(l).unwrap();
        robots.push(Robot {
            start_position: Position::from(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
            velocity: Position::from(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
        });
    }
    robots
}

fn solve1(robots: &Vec<Robot>) -> usize {
    let mut res_positions = vec![];
    for r in robots {
        res_positions.push(r.evolve(100));
    }
    let mut count = 1;
    count *= res_positions
        .iter()
        .filter(|p| p.y() < 51 && p.x() < 50)
        .count();
    count *= res_positions
        .iter()
        .filter(|p| p.y() > 51 && p.x() < 50)
        .count();
    count *= res_positions
        .iter()
        .filter(|p| p.y() > 51 && p.x() > 50)
        .count();
    count *= res_positions
        .iter()
        .filter(|p| p.y() < 51 && p.x() > 50)
        .count();
    count
}

fn solve2(robots: &Vec<Robot>) -> i32 {
    let mut second = 1;
    loop {
        let mut res_positions = HashSet::new();
        for r in robots {
            res_positions.insert(r.evolve(second));
        }
        if res_positions.len() == robots.len() {
            println!("Second {second}");
            for y in 0..103 {
                for x in 0..101 {
                    if res_positions.contains(&Position::from(x, y)) {
                        print!("X");
                    } else {
                        print!(" ");
                    }   
                }
                println!();
            }
            sleep(Duration::from_millis(100));
        }
        second += 1;
        // These two frame have a special pattern
        // 28 mod 101 
        // 84 mod 103
        // Chinese remainder theorem could lead to the answer
    }
}

#[test]
fn part_one() {
    let robots = parse(String::from(
        "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
",
    ));
    assert_eq!(12, solve1(&robots));
}
