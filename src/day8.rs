use std::collections::{HashMap, HashSet};
use crate::utils::Position;

pub fn solve(data: String) {
    println!("Part one : {}", solve1(&data));
    println!("Part two : {}", solve2(&data));
}


fn parse(data: &String) -> (HashMap<char, Vec<Position>>, usize, usize) {
    let mut map: HashMap<char, Vec<Position>> = HashMap::new();
    let mut width = 0;
    for (y, l) in data.lines().enumerate(){
        let mut temp = 0;
        for (x, c) in l.chars().enumerate() {
            temp += 1;
            if c != '.' {
                match map.get_mut(&c) {
                    Some(v) => v.push(Position::from_usize(x, y)),
                    _ => {map.insert(c, vec![Position::from_usize(x, y)]); ()}
                }
            }
        }
        if width == 0 {
            width = temp;
        }
    }
    (map, data.lines().count(), width)
}

fn solve1(data: &String) -> usize {
    let (map, y_max, x_max) = parse(data);
    let x_max = x_max;
    let y_max = y_max;
    let mut antinodes = HashSet::new();
    for (_, pos) in map {
        for i in 0..pos.len() {
            for j in i+1..pos.len() {
                let diff = pos[i] - pos[j];
                let ant1 = pos[i] + diff;
                let ant2 = pos[j] - diff;
                if ant1.is_valid(x_max, y_max) {
                    antinodes.insert(ant1);
                } 
                if ant2.is_valid(x_max, y_max) {
                    antinodes.insert(ant2);
                }

            }
        }
    }
    antinodes.len()
}

fn solve2(data: &String) -> usize {
    let (map, y_max, x_max) = parse(data);
    let x_max = x_max;
    let y_max = y_max;
    let mut antinodes = HashSet::new();
    for (_, pos) in map {
        for i in 0..pos.len() {
            for j in i+1..pos.len() {
                let diff = pos[i] - pos[j];

                let mut ant1 = pos[i];
                while ant1.is_valid(x_max, y_max) {
                    antinodes.insert(ant1);
                    ant1 = ant1 + diff;
                }
                ant1 = pos[i];
                while ant1.is_valid(x_max, y_max) {
                    antinodes.insert(ant1);
                    ant1 = ant1 - diff;
                }

                let mut ant2 = pos[j];
                while ant2.is_valid(x_max, y_max) {
                    antinodes.insert(ant2);
                    ant2 = ant2 + diff;
                }
                ant2 = pos[j];
                while ant2.is_valid(x_max, y_max) {
                    antinodes.insert(ant2);
                    ant2 = ant2 - diff;
                }
            }
        }
    }
    antinodes.len()
}

#[test]
fn part_one() {
    let res = solve1(&String::from(
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    ));
    assert_eq!(14, res);
}

#[test]
fn part_two() {
    let res = solve2(&String::from(
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    ));
    assert_eq!(34, res);
}