use std::{
    collections::{HashMap, HashSet},
    fmt, vec,
};

use crate::{
    parser_helper,
    utils::{Direction, Position},
};

pub fn solve(data: String) {
    let maze = parser_helper::parse_double_vec(data.as_str());
    println!("Part one : {}", solve1(&maze));
    println!("Part two : {}", solve2(&maze));
}

fn wall_positions(maze: &Vec<Vec<char>>) -> Vec<Position> {
    let mut walls = vec![];
    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            if maze[y][x] == '#' {
                walls.push(Position::from_usize(x, y));
            }
        }
    }
    walls
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct SavePos {
    pos: Position,
    dir: Direction,
    history: Vec<Position>,
    price: usize,
}

impl fmt::Debug for SavePos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Custom formatting logic
        f.debug_struct("SavePos")
            .field("pos", &self.pos)
            .field("dir", &self.dir)
            .field("price", &self.price)
            .finish() // Don't include `history`
    }
}

#[allow(dead_code)]
fn print_path(walls: &Vec<Position>, path: &Vec<Position>, max_x: usize, max_y: usize) {
    for y in 0..max_y {
        for x in 0..max_x {
            let p = Position::from_usize(x, y);
            if walls.contains(&p) {
                print!("#");
            } else if path.contains(&p) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_shortest_paths(
    walls: &Vec<Position>,
    max_x: usize,
    max_y: usize,
) -> (Vec<Vec<Position>>, usize) {
    let mut paths = vec![];
    let final_pos = Position::from_usize(max_x - 2, 1);
    let mut current_pos = Position::from_usize(1, max_y - 2);
    let mut current_price = 0;
    let mut current_dir = Direction::Rigth;
    let mut visited: HashMap<(Position, Direction), usize> = HashMap::new();
    let mut options: HashSet<SavePos> = HashSet::new();
    let mut history = vec![];
    let mut min_price = 0;
    let mut found = false;
    loop {
        while current_pos != final_pos {
            if found && current_price > min_price {
                break;
            }
            history.push(current_pos);
            let possible_dir = vec![
                current_dir.clockwise(),
                current_dir.counter_clockwise(),
                current_dir,
            ];
            let mut can_go_forward = false;
            for d in possible_dir {
                let next_pos = current_pos.to(d, max_x, max_y).unwrap();
                if !walls.contains(&next_pos) {
                    let temp = visited.get(&(next_pos, d));
                    if temp.is_none() || *temp.unwrap() > current_price {
                        if d == current_dir {
                            can_go_forward = true;
                        }
                        let next_price = if d == current_dir {
                            current_price + 1
                        } else {
                            current_price + 1001
                        };
                        let sp = options.iter().find(|o| o.pos == next_pos && o.dir == d);
                        if sp.is_none() || sp.unwrap().price >= next_price {
                            if !sp.is_none() && sp.unwrap().price > next_price{
                                options.remove(&sp.unwrap().clone());
                            }
                            options.insert(SavePos {
                                pos: next_pos,
                                dir: d,
                                history: history.clone(),
                                price: next_price,
                            });
                        }
                    }
                }
            }
            if can_go_forward {
                visited.insert((current_pos, current_dir), current_price);
            } else {
                visited.insert((current_pos, current_dir), current_price + 1000);
            }
            let min_opt = options
                .iter()
                .reduce(|min, opt| {
                    if opt.price < min.price {
                        return opt;
                    } else {
                        min
                    }
                })
                .unwrap()
                .clone();
            options.remove(&min_opt);
            current_pos = min_opt.pos;
            current_dir = min_opt.dir;
            current_price = min_opt.price;
            history = min_opt.history;
        }
        min_price = current_price;
        found = true;
        history.push(current_pos);
        paths.push(history);
        options = options
            .into_iter()
            .filter(|o| o.price <= min_price)
            .collect();
        if options.is_empty() {
            break;
        }
        let opt = options.iter().last().unwrap().clone();
        options.remove(&opt);
        current_pos = opt.pos;
        current_dir = opt.dir;
        current_price = opt.price;
        history = opt.history.clone();
    }
    (paths, min_price)
}

fn solve1(maze: &Vec<Vec<char>>) -> usize {
    let walls = wall_positions(maze);
    let max_y = maze.len();
    let max_x = maze[0].len();
    let (_, current_price) = get_shortest_paths(&walls, max_x, max_y);
    current_price
}

fn solve2(maze: &Vec<Vec<char>>) -> usize {
    let walls = wall_positions(maze);
    let max_y = maze.len();
    let max_x = maze[0].len();
    let mut all_pos: HashSet<Position> = HashSet::new();
    let (paths, _) = get_shortest_paths(&walls, max_x, max_y);
    paths.iter().for_each(|p| all_pos.extend(p));
    all_pos.len()
}

#[test]
fn part_one_simple() {
    let maze = parser_helper::parse_double_vec(
        "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
",
    );
    assert_eq!(7036, solve1(&maze));
}

#[test]
fn part_one_big() {
    let maze = parser_helper::parse_double_vec(
        "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
",
    );
    assert_eq!(11048, solve1(&maze));
}

#[test]
fn part_one_reddit() {
    let maze = parser_helper::parse_double_vec(
        "\
##########
#.......E#
#.##.#####
#..#.....#
##.#####.#
#S.......#
##########
",
    );
    assert_eq!(4013, solve1(&maze));
}

#[test]
fn part_two_simple() {
    let maze = parser_helper::parse_double_vec(
        "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
",
    );
    assert_eq!(45, solve2(&maze));
}

#[test]
fn part_two_big() {
    let maze = parser_helper::parse_double_vec(
        "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
",
    );
    assert_eq!(64, solve2(&maze));
}

#[test]
fn part_two_perso() {
    let maze = parser_helper::parse_double_vec(
        "\
#################
#.........#....E#
#.........#.....#
#....#....#.....#
#....#....#.....#
#....#....#.....#
#....#....#.....#
#....#..........#
#S...#..........#
#################
",
    );
    assert_eq!(31, solve2(&maze));
}
