use std::collections::HashSet;

use crate::parser_helper;
use crate::utils::Position;

pub fn solve(data: String) {
    let map = parser_helper::parse_double_vec_int(data.as_str());
    println!("Part one : {}", solve1(&map));
    println!("Part two : {}", solve2(&map));
}

#[derive(Clone)]
struct Node {
    elevation: u32,
    pos: Position,
    children: Vec<Node>,
}

impl Node {
    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

fn check_one_node(map: &Vec<Vec<u32>>, current: &Node, pos: Position) -> Option<Node> {
    let y_max = map.len();
    let x_max = map[0].len();
    if pos.is_valid(x_max, y_max) {
        if map[pos.y_usize()][pos.x_usize()] == current.elevation + 1 {
            return Some(Node {
                elevation: current.elevation + 1,
                pos: pos,
                children: vec![],
            });
        }
    }
    None
}

fn build_path(map: &Vec<Vec<u32>>, current: &mut Node) {
    if current.elevation == 9 {
        return;
    }

    let left = Position::from(current.pos.x() - 1, current.pos.y());
    let rigth = Position::from(current.pos.x() + 1, current.pos.y());
    let up = Position::from(current.pos.x(), current.pos.y() - 1);
    let down = Position::from(current.pos.x(), current.pos.y() + 1);

    if let Some(n) = check_one_node(map, current, left) {
        let mut next = n.clone();
        build_path(map, &mut next);
        current.add_child(next);
    }
    if let Some(n) = check_one_node(map, current, rigth) {
        let mut next = n.clone();
        build_path(map, &mut next);
        current.add_child(next);
    }
    if let Some(n) = check_one_node(map, current, up) {
        let mut next = n.clone();
        build_path(map, &mut next);
        current.add_child(next);
    }
    if let Some(n) = check_one_node(map, current, down) {
        let mut next = n.clone();
        build_path(map, &mut next);
        current.add_child(next);
    }
}

fn can_reach_9(root: Node, nine_position: &mut HashSet<Position>) {
    if root.elevation == 9 {
        nine_position.insert(root.pos);
    }
    for c in root.children {
        can_reach_9(c, nine_position);
    }
}

fn nb_path_to_9(root: Node) -> i32 {
    if root.elevation == 9 {
        return 1;
    }
    let mut ret = 0;
    for c in root.children {
        ret += nb_path_to_9(c);
    }
    ret
}


fn build_graph(map: &Vec<Vec<u32>>) -> Vec<Node> {
    let mut graph = vec![];
    for (y, v) in map.iter().enumerate() {
        for (x, elem) in v.iter().enumerate() {
            if *elem == 0 {
                let mut current = Node {
                    elevation: 0,
                    pos: Position::from_usize(x, y),
                    children: vec![],
                };
                build_path(&map, &mut current);
                graph.push(current);
            }
        }
    }
    graph
}

fn solve1(map: &Vec<Vec<u32>>) -> usize {
    let graph = build_graph(map);
    let mut ret = 0;
    for zero in graph {
        let mut valid_nine = HashSet::new();
        can_reach_9(zero, &mut valid_nine);
        ret += valid_nine.len();
    }
    ret
}

fn solve2(map: &Vec<Vec<u32>>) -> i32 {
    let graph = build_graph(map);
    let mut ret = 0;
    for zero in graph {
        ret += nb_path_to_9(zero);
    }
    ret
}

#[test]
fn part_one() {
    let map = parser_helper::parse_double_vec_int("\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    );
    let res = solve1(&map);
    assert_eq!(36, res);
}

#[test]
fn part_two() {
    let map = parser_helper::parse_double_vec_int("\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    );
    let res = solve2(&map);
    assert_eq!(81, res);
}

