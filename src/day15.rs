use crate::{parser_helper, utils::Position};

pub fn solve(data: String) {
    let (map, moves) = parse(data);
    println!("Part one : {}", solve1(&map, &moves));
    println!("Part two : {}", solve2(&map, &moves));
}

fn parse(data: String) -> (Vec<Vec<char>>, Vec<char>) {
    let mut split = data.split("\n\n");
    let map = parser_helper::parse_double_vec(split.next().unwrap());
    let moves = parser_helper::parse_vec_char(split.next().unwrap());
    (map, moves)
}

fn update_box(
    box_pos: Position,
    direction: Position,
    boxes: &mut Vec<Position>,
    walls: &Vec<Position>,
) -> bool {
    let new_box_pos = box_pos + direction;
    if walls.contains(&new_box_pos) {
        return false;
    }
    if boxes.contains(&new_box_pos) {
        if update_box(new_box_pos, direction, boxes, walls) {
            let index = boxes.iter().position(|b| *b == box_pos).unwrap();
            boxes.remove(index);
            boxes.push(new_box_pos);
            return true;
        } else {
            return false;
        }
    }
    let index = boxes.iter().position(|b| *b == box_pos).unwrap();
    boxes.remove(index);
    boxes.push(new_box_pos);
    true
}

fn solve1(map: &Vec<Vec<char>>, moves: &Vec<char>) -> i32 {
    let mut box_positions = vec![];
    let mut wall_positions = vec![];
    let mut robot_position = Position::from(-1, -1);
    let max_y = map.len();
    let max_x = map[0].len();
    for y in 0..max_y {
        for x in 0..max_x {
            match map[y][x] {
                '#' => wall_positions.push(Position::from_usize(x, y)),
                'O' => box_positions.push(Position::from_usize(x, y)),
                '@' => robot_position = Position::from_usize(x, y),
                _ => continue,
            }
        }
    }

    if robot_position == Position::from(-1, -1) {
        panic!("The robot position has not bee found in the map");
    }

    for m in moves {
        let next_pos;
        match m {
            '>' => next_pos = robot_position.rigth(max_x),
            '<' => next_pos = robot_position.left(),
            '^' => next_pos = robot_position.up(),
            'v' => next_pos = robot_position.down(max_y),
            _ => panic!("Movement not supported"),
        }
        if let Some(p) = next_pos {
            if box_positions.contains(&p) {
                let can_move =
                    update_box(p, p - robot_position, &mut box_positions, &wall_positions);
                if can_move {
                    robot_position = p;
                }
            } else if !wall_positions.contains(&p) {
                robot_position = p;
            }
        }
    }
    box_positions
        .iter()
        .fold(0, |acc, v| acc + v.y() * 100 + v.x())
}

fn push_double_box(
    box_pos: (Position, Position),
    direction: Position,
    boxes: &mut Vec<(Position, Position)>,
) {
    let (left_new_box, rigth_new_box) = (box_pos.0 + direction, box_pos.1 + direction);
    let filtered: Vec<(Position, Position)> = boxes
        .iter()
        .filter(|b| {
            (rigth_new_box == b.0
                || rigth_new_box == b.1
                || left_new_box == b.0
                || left_new_box == b.1)
                && **b != box_pos
        })
        .map(|b| *b)
        .collect();
    for box_on_the_way in filtered {
        push_double_box(box_on_the_way, direction, boxes);
    }
    let index = boxes.iter().position(|b| *b == box_pos).unwrap();
    boxes.remove(index);
    boxes.push((left_new_box, rigth_new_box));
}

fn can_push_double_box(
    box_pos: (Position, Position),
    direction: Position,
    boxes: &mut Vec<(Position, Position)>,
    walls: &Vec<Position>,
    initial_box: bool
) -> bool {
    let (left_new_box, rigth_new_box) = (box_pos.0 + direction, box_pos.1 + direction);
    if walls.contains(&rigth_new_box) || walls.contains(&left_new_box) {
        return false;
    }
    let filtered: Vec<(Position, Position)> = boxes
        .iter()
        .filter(|b| {
            (rigth_new_box == b.0
                || rigth_new_box == b.1
                || left_new_box == b.0
                || left_new_box == b.1)
                && **b != box_pos
        })
        .map(|b| *b)
        .collect();
    let mut can_push = true;
    for box_on_the_way in filtered {
        can_push &= can_push_double_box(box_on_the_way, direction, boxes, walls, false);
    }
    if !can_push {
        return false;
    }
    if initial_box {
        push_double_box(box_pos, direction, boxes);
    }
    true
}

fn solve2(map: &Vec<Vec<char>>, moves: &Vec<char>) -> i32 {
    let mut box_positions = vec![];
    let mut wall_positions = vec![];
    let mut robot_position = Position::from(-1, -1);
    let max_y = map.len();
    let max_x = map[0].len() * 2;
    for y in 0..max_y {
        for x in 0..map[0].len() {
            match map[y][x] {
                '#' => {
                    wall_positions.push(Position::from_usize(x * 2, y));
                    wall_positions.push(Position::from_usize(x * 2, y).rigth(max_x).unwrap());
                }
                'O' => {
                    box_positions.push((
                        Position::from_usize(x * 2, y),
                        Position::from_usize(x * 2, y).rigth(max_x).unwrap(),
                    ));
                }
                '@' => robot_position = Position::from_usize(x * 2, y),
                _ => continue,
            }
        }
    }

    if robot_position == Position::from(-1, -1) {
        panic!("The robot position has not bee found in the map");
    }

    for m in moves {
        let next_pos;
        match m {
            '>' => next_pos = robot_position.rigth(max_x),
            '<' => next_pos = robot_position.left(),
            '^' => next_pos = robot_position.up(),
            'v' => next_pos = robot_position.down(max_y),
            _ => panic!("Movement not supported"),
        }
        if let Some(p) = next_pos {
            if let Some(b_pos) = box_positions
                .iter()
                .find(|(left, rigth)| *left == p || *rigth == p)
            {
                let can_move = can_push_double_box(
                    *b_pos,
                    p - robot_position,
                    &mut box_positions,
                    &wall_positions,
                    true
                );
                if can_move {
                    robot_position = p;
                }
            } else if !wall_positions.contains(&p) {
                robot_position = p;
            }
        }
    }
    box_positions
        .iter()
        .fold(0, |acc, v| acc + v.0.y() * 100 + v.0.x())
}

#[test]
fn part_one_small() {
    let (map, moves) = parse(String::from(
        "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
    ));
    let res = solve1(&map, &moves);
    assert_eq!(2028, res);
}

#[test]
fn part_one_big() {
    let (map, moves) = parse(String::from(
        "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    ));
    let res = solve1(&map, &moves);
    assert_eq!(10092, res);
}

// ####################
// ##....[]....[]..[]##
// ##............[]..##
// ##..[][]....[]..[]##
// ##....[]@.....[]..##
// ##[]##....[]......##
// ##[]....[]....[]..##
// ##..[][]..[]..[][]##
// ##........[]......##
// ####################
#[test]
fn part_two() {
    let (map, moves) = parse(String::from(
        "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    ));
    let res = solve2(&map, &moves);
    assert_eq!(9021, res);
}

// ##############
// ##......##..##
// ##..........##
// ##....[][]@.##
// ##....[]....##
// ##..........##
// ##############
#[test]
fn part_two_simple() {
    let (map, moves) = parse(String::from(
        "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
    ));
    let res = solve2(&map, &moves);
    assert_eq!(618, res);
}
