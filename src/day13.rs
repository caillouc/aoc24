use std::vec;

use regex::Regex;

use crate::utils::Position;

pub fn solve(data: String) {
    let machines = parse(data);
    println!("Part one : {}", solve1(&machines));
    println!("Part two : {}", solve2(&machines));
}

#[derive(Debug)]
struct Machine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

fn parse(data: String) -> Vec<Machine> {
    let mut lines = data.lines();
    let mut ret = vec![];

    while let Some(first_machine_line) = lines.next() {
        let button_a_regex = Regex::new(r"Button A: X\+([0-9]*), Y\+([0-9]*)").unwrap();
        let button_b_regex = Regex::new(r"Button B: X\+([0-9]*), Y\+([0-9]*)").unwrap();
        let prize_regex = Regex::new(r"Prize: X=([0-9]*), Y=([0-9]*)").unwrap();

        let cap = button_a_regex.captures(first_machine_line).unwrap();
        let button_a = Position::from(cap[1].parse().unwrap(), cap[2].parse().unwrap());

        let cap = button_b_regex.captures(lines.next().unwrap()).unwrap();
        let button_b = Position::from(cap[1].parse().unwrap(), cap[2].parse().unwrap());

        let cap = prize_regex.captures(lines.next().unwrap()).unwrap();
        let prize = Position::from(cap[1].parse().unwrap(), cap[2].parse().unwrap());

        // skip blank line
        lines.next();

        ret.push(Machine {
            button_a,
            button_b,
            prize,
        });
    }
    ret
}

// solve
// ax + by = c
// dx + ey = f
fn equation_solve(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> Option<(i32, i32)> {
    if a * e - b * d == 0 {
        return None;
    }
    if b * (a * e - b * d) == 0 {
        return None;
    }
    let x = (e * c - b * f) / (a * e - b * d);
    let y = (c * (a * e - b * d) - a * (e * c - b * f)) / (b * (a * e - b * d));
    Some((x, y))
}

fn solve1(machines: &Vec<Machine>) -> i32 {
    let mut solutions = vec![];
    println!("{:?}", machines.iter().last());
    for m in machines {
        if let Some(res) = equation_solve(
            m.button_a.x(),
            m.button_b.x(),
            m.prize.x(),
            m.button_a.y(),
            m.button_b.y(),
            m.prize.y(),
        ) {
            solutions.push(res);
        }
    }
    let solutions: Vec<(i32, i32)> = solutions
        .iter()
        .filter(|(p1, p2)| *p1 <= 100 && *p2 <= 100 && *p1 >= 0 && *p2 >= 0)
        .map(|v| *v)
        .collect();
    println!("{:?}", solutions);
    solutions.iter().fold(0, |acc, (a, b)| acc + a * 3 + b)
}

fn solve2(machines: &Vec<Machine>) -> i32 {
    0
}

#[test]
fn part_one() {
    let machines = parse(String::from(
        "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279

",
    ));
    assert_eq!(480, solve1(&machines));
}
