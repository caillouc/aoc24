use std::collections::HashSet;

use crate::parser_helper;
use crate::utils::Position;

pub fn solve(data: String) {
    let pots = parser_helper::parse_double_vec(data);
    println!("Part one : {}", solve1(&pots));
    println!("Part two : {}", solve2(&pots));
}

#[derive(Debug)]
struct Region {
    positions: HashSet<Position>,
    plant: char,
    connection: i32,
}

impl Region {
    fn area(&self) -> i32 {
        self.positions.len() as i32
    }
    fn perimeter(&self) -> i32 {
        4 * self.area() - self.connection
    }
}

fn update_region(current_pos: Position, region: &mut Region, pots: &Vec<Vec<char>>) {
    let to_check: Vec<Position> = vec![
        current_pos.left(),
        current_pos.rigth(pots[0].len()),
        current_pos.top(),
        current_pos.bottom(pots.len()),
    ]
    .iter()
    .filter(|e| e.is_some())
    .map(|e| e.unwrap())
    .collect();

    for elem in to_check {
        if pots[elem.y_usize()][elem.x_usize()] == region.plant {
            if !region.positions.contains(&elem) {
                region.positions.insert(elem);
                update_region(elem, region, pots);
            }
            region.connection += 1;
        }
    }
}

fn build_regions(pots: &Vec<Vec<char>>) -> Vec<Region> {
    let mut all_positions = HashSet::new();
    let mut regions = vec![];
    for y in 0..pots.len() {
        for x in 0..pots[0].len() {
            all_positions.insert(Position::from_usize(x, y));
        }
    }
    while !all_positions.is_empty() {
        let current_position = all_positions.iter().last().unwrap();
        let mut current_region = Region {
            positions: HashSet::new(),
            plant: pots[current_position.y_usize()][current_position.x_usize()],
            connection: 0,
        };
        current_region.positions.insert(*current_position);
        update_region(*current_position, &mut current_region, pots);
        current_region.positions.iter().for_each(|p| {
            all_positions.remove(p);
        });
        regions.push(current_region);
    }
    regions
}

fn solve1(pots: &Vec<Vec<char>>) -> i32 {
    let regions = build_regions(pots);
    regions
        .iter()
        .fold(0, |acc, r| acc + r.area() * r.perimeter())
}

fn contains(region: &Region, pos: Option<Position>) -> bool {
    match pos {
        Some(p) => region.positions.contains(&p),
        None => false,
    }
}

fn nb_edges(current_pos: &Position, region: &Region, pots: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let max_x = pots[0].len();
    let max_y = pots.len();

    if !contains(region, current_pos.top()) && !contains(region, current_pos.left()) {
        count += 1;
    }
    if !contains(region, current_pos.top_left())
        && contains(region, current_pos.top())
        && contains(region, current_pos.left())
    {
        count += 1;
    }
    if !contains(region, current_pos.bottom(max_y)) && !contains(region, current_pos.left()) {
        count += 1;
    }
    if !contains(region, current_pos.bottom_left(max_y))
        && contains(region, current_pos.bottom(max_y))
        && contains(region, current_pos.left())
    {
        count += 1;
    }
    if !contains(region, current_pos.top()) && !contains(region, current_pos.rigth(max_x)) {
        count += 1;
    }
    if !contains(region, current_pos.top_rigth(max_x))
        && contains(region, current_pos.top())
        && contains(region, current_pos.rigth(max_x))
    {
        count += 1;
    }
    if !contains(region, current_pos.bottom(max_y)) && !contains(region, current_pos.rigth(max_x)) {
        count += 1;
    }
    if !contains(region, current_pos.bottom_rigth(max_x, max_y))
        && contains(region, current_pos.bottom(max_y))
        && contains(region, current_pos.rigth(max_x))
    {
        count += 1;
    }
    count
}

fn solve2(pots: &Vec<Vec<char>>) -> i32 {
    let regions = build_regions(pots);
    regions.iter().fold(0, |acc, r| {
        acc + r.area()
            * r.positions
                .iter()
                .fold(0, |total_corners, p| total_corners + nb_edges(p, r, pots))
    })
}

#[test]
fn part_one_simple() {
    let pots = parser_helper::parse_double_vec(String::from(
        "\
AAAA
BBCD
BBCC
EEEC
",
    ));
    assert_eq!(140, solve1(&pots));
}

#[test]
fn part_one_multiple() {
    let pots = parser_helper::parse_double_vec(String::from(
        "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
    ));
    assert_eq!(772, solve1(&pots));
}

#[test]
fn part_one() {
    let pots = parser_helper::parse_double_vec(String::from(
        "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
    ));
    assert_eq!(1930, solve1(&pots));
}

#[test]
fn part_two_e() {
    let pots = parser_helper::parse_double_vec(String::from(
        "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
",
    ));
    assert_eq!(236, solve2(&pots));
}

#[test]
fn part_two() {
    let pots = parser_helper::parse_double_vec(String::from(
        "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
",
    ));
    assert_eq!(368, solve2(&pots));
}

#[test]
fn part_two_big() {
    let pots = parser_helper::parse_double_vec(String::from(
        "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
    ));
    assert_eq!(1206, solve2(&pots));
}