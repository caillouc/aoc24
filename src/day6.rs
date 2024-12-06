use std::{collections::HashSet, vec};

pub fn solve(data: String) {
    println!("Part one : {}", solve1(&data));
    println!("Part two : {}", solve2(&data));
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Chair,
    Guard(Direction),
    Visited,
    NotVisited,
}

impl Cell {
    fn from(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::NotVisited),
            '#' => Some(Self::Chair),
            '^' => Some(Self::Guard(Direction::Up)),
            'v' => Some(Self::Guard(Direction::Down)),
            '>' => Some(Self::Guard(Direction::Right)),
            '<' => Some(Self::Guard(Direction::Left)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Area {
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Area {
    fn visited_cell_count(&self) -> usize {
        self.cells.iter().fold(0, |acc, v| {
            acc + v
                .iter()
                .filter(|c| matches!(c, Cell::Visited) || matches!(c, Cell::Guard(_)))
                .count()
        })
    }
    fn visit_cell(&mut self, pos: &Position) {
        if matches!(self.cells[pos.y][pos.x], Cell::NotVisited) {
            self.cells[pos.y][pos.x] = Cell::Visited;
        }
    }
    fn get_cell(&self, pos: &Position) -> Option<&Cell> {
        match self.cells.get(pos.y) {
            Some(v) => v.get(pos.x),
            _ => None,
        }
    }
    fn walk_in_direction(&mut self, from: &Position, direction: Direction) -> Option<Position> {
        let to = match direction {
            Direction::Up => Position {
                x: from.x,
                y: {
                    if from.y as i32 - 1 == -1 {return None;}
                    from.y - 1
                },
            },
            Direction::Down => Position {
                x: from.x,
                y: from.y + 1,
            },
            Direction::Left => Position {
                x: {
                    if from.x as i32 - 1 == -1 {return None;}
                    from.x - 1
                },
                y: from.y,
            },
            Direction::Right => Position {
                x: from.x + 1,
                y: from.y,
            },
        };
        if let Some(cell) = self.get_cell(&to) {
            if matches!(cell, Cell::Chair) {
                return Some(from.clone());
            }
            self.visit_cell(&to);
            return Some(to);
        }
        None
    }
    fn walk_to_chair(&mut self, from: Position, direction: Direction) -> Option<Position> {
        let mut start = from;
        while let Some(cell) = self.walk_in_direction(&start, direction) {
            if cell != start {
                start = cell;
            } else {
                return Some(cell);
            }
        }
        None
    }
    fn walk_to_chair_with_new(&mut self, from: Position, direction: Direction, new_chair: Position) -> Option<Position> {
        let mut start = from;
        while let Some(cell) = self.walk_in_direction(&start, direction) {
            if cell != start && cell != new_chair{
                start = cell;
            } else {
                if cell == new_chair {
                    return self.walk_in_direction(&cell, direction.rotate().rotate());
                }
                return Some(cell);
            }
        }
        None
    }
    fn get_guard_position(&self) -> Option<(Position, Direction)> {
        for (y, v) in self.cells.iter().enumerate() {
            for (x, c) in v.iter().enumerate() {
                if let Cell::Guard(d) = c {
                    return Some((Position { x, y }, *d));
                }
            }
        }
        None
    }
}

fn parse(data: &String) -> Area {
    Area {
        cells: data
            .lines()
            .map(|l| l.chars().map(|c| Cell::from(c).unwrap()).collect())
            .collect(),
    }
}

fn solve1(data: &String) -> usize {
    let mut area = parse(data);
    let (guard_pos, mut dir) = area.get_guard_position().unwrap();
    let mut current_pos = guard_pos.clone();
    while let Some(new_pos) = area.walk_to_chair(current_pos, dir) {
        current_pos = new_pos;
        dir = dir.rotate();
    }
    area.visited_cell_count()
}

fn solve2(data: &String) -> usize {
    let mut area = parse(data);
    let (guard_pos, guard_dir) = area.get_guard_position().unwrap();
    let mut dir = guard_dir;
    let mut current_pos = guard_pos.clone();
    let mut turning_point = vec![];
    let mut cell_visited = vec![];
    while let Some(next_pos) = area.walk_in_direction(&current_pos, dir) {
        if next_pos == current_pos {
            // I reached a chair
            turning_point.push(current_pos.clone());
            dir = dir.rotate();
        } else {
            cell_visited.push(next_pos.clone());
            current_pos = next_pos;
        }
    }


    let mut valid_chair = HashSet::new();
    for new_init_pos in cell_visited {
        let mut turning_point = vec![];
        let mut from = guard_pos.clone();
        let mut dir = guard_dir.clone();
        while let Some(tp) = area.walk_to_chair_with_new(from.clone(), dir, new_init_pos.clone()) {
            if turning_point.contains(&(tp.clone(), dir)) {
                if new_init_pos != guard_pos {
                    valid_chair.insert(new_init_pos);
                }
                break;
            } else {
                turning_point.push((tp.clone(), dir));
                from = tp;
                dir = dir.rotate()
            }
        }
    }
    valid_chair.len()

}

#[test]
fn part_one() {
    let res = solve1(&String::from(
        "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
    ));
    assert_eq!(41, res);
}

#[test]
fn part_one_2() {
    let res = solve1(&String::from(
        "\
.#................
..#...............
..................
.^................
..................
",
    ));
    assert_eq!(4, res);
}

#[test]
fn part_two() {
    let res = solve2(&String::from(
        "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
    ));
    assert_eq!(6, res);
}

#[test]
fn part_two_2() {
    let res = solve2(&String::from(
        "\
.#..
....
#..#
.^#.
",
    ));
    assert_eq!(2, res);
}

#[test]
fn part_two_3() {
    let res = solve2(&String::from(
        "\
.#....
.....#
..#...
#.....
....#.
#.....
..^#..
",
    ));
    assert_eq!(1, res);
}

#[test]
fn part_two_4() {
    let res = solve2(&String::from(
        "\
.#.
...
#.#
#.#
#.#
#^#
###
",
    ));
    assert_eq!(5, res);
}

#[test]
fn part_two_5() {
    let res = solve2(&String::from(
        "\
########
#......#
...^...#
########
",
    ));
    assert_eq!(6, res);
}