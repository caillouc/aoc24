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
fn equation_solve(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Option<(i64, i64)> {
    let det = a * e - b * d;
    if a / d == b / e && a / d == c / f && det == 0.0 {
        // There are an infinite number of solution, return the cheapest one
        let ret_y = c / b;
        if ret_y.fract() == 0.0 {
            return Some((0, ret_y as i64));
        } else {
            return None;
        }
    }
    if det == 0.0 {
        return None;
    }
    let x = (c * e - b * f) / det;
    let y = (a * f - c * d) / det;
    if x.fract() == 0.0 && y.fract() == 0.0 {
        return Some((x as i64, y as i64));
    }
    None
}

fn solve1(machines: &Vec<Machine>) -> i64 {
    let mut solutions = vec![];
    for m in machines {
        if let Some(res) = equation_solve(
            m.button_a.x() as f64,
            m.button_b.x() as f64,
            m.prize.x() as f64,
            m.button_a.y() as f64,
            m.button_b.y() as f64,
            m.prize.y() as f64,
        ) {
            solutions.push(res);
        }
    }
    let solutions: Vec<(i64, i64)> = solutions
        .iter()
        .filter(|(p1, p2)| *p1 <= 100 && *p2 <= 100 && *p1 >= 0 && *p2 >= 0)
        .map(|v| *v)
        .collect();
    solutions.iter().fold(0, |acc, (a, b)| acc + a * 3 + b)
}

fn solve2(machines: &Vec<Machine>) -> i64 {
    let mut solutions = vec![];
    for m in machines {
        if let Some(res) = equation_solve(
            m.button_a.x() as f64,
            m.button_b.x() as f64,
            (m.prize.x() as i64 + 10000000000000)as f64,
            m.button_a.y() as f64,
            m.button_b.y() as f64,
            (m.prize.y() as i64 + 10000000000000) as f64,
        ) {
            solutions.push(res);
        }
    }
    let solutions: Vec<(i64, i64)> = solutions
        .iter()
        .filter(|(p1, p2)| *p1 >= 0 && *p2 >= 0)
        .map(|v| *v)
        .collect();
    solutions.iter().fold(0, |acc, (a, b)| acc + a * 3 + b)
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
