use std::collections::HashMap;

use crate::parser_helper;

pub fn solve(data: String) {
    let stones: Vec<u64> = parser_helper::parse_vec_delimiter(data, " ");
    println!("Part one : {}", solve1(&stones));
    println!("Part two : {}", solve2(&stones));
}

fn evolve(stone: u64) -> Vec<u64> {
    let stone_str = stone.to_string();
    if stone == 0 {
        return vec![1];
    } else if stone_str.len() % 2 == 0{
        let mid = stone_str.len() / 2;
        return vec![stone_str[..mid].parse().unwrap(), stone_str[mid..].parse().unwrap()];
    } else {
        return  vec![stone * 2024];
    }
}

fn solve1(stones: &Vec<u64>) -> usize {
    let mut stones = stones.clone();
    for _ in 0..25 {
        stones = stones.iter().flat_map(|s| evolve(*s)).collect();
    }
    stones.len()
}

fn solve2(stones: &Vec<u64>) -> usize {
    // Assume no duplicate in data
    let mut stones_map: HashMap<u64, usize> = stones.iter().map(|s| (*s, 1)).collect();
    for _ in 0..75 {
        let mut new_map: HashMap<u64, usize> = HashMap::new();
        for (k, v) in stones_map {
            let temp = evolve(k);
            temp.iter().for_each(|s| {
                new_map.insert(*s, new_map.get(s).unwrap_or(&0) + v);
            });
        }
        stones_map = new_map;
    }
    stones_map.values().sum()
}

#[test]
fn part_one() {
    let stones: Vec<u64> = parser_helper::parse_vec_delimiter(String::from("125 17"), " ");
    let res = solve1(&stones);
    assert_eq!(55312, res);
}