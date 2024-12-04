pub fn solve(data: String) {
    println!("Part one : {}", solve1(&data));
    println!("Part two : {}", solve2(&data));
}

fn parse(data: &String) -> Vec<Vec<char>> {
    data.lines().map(|l| l.chars().collect()).collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RelativePosition {
    Before,
    After,
    Up,
    Down,
    BeforeUp,
    BeforeDown,
    AfterUp,
    AfterDown,
}

impl RelativePosition {
    fn index(&self) -> (i32, i32) {
        match self {
            RelativePosition::Before => (-1, 0),
            RelativePosition::BeforeDown => (-1, 1),
            RelativePosition::BeforeUp => (-1, -1),
            RelativePosition::After => (1, 0),
            RelativePosition::AfterDown => (1, 1),
            RelativePosition::AfterUp => (1, -1),
            RelativePosition::Up => (0, -1),
            RelativePosition::Down => (0, 1),
        }
    }
}

fn check_char(
    chars: &Vec<Vec<char>>,
    position_to_check: &[RelativePosition],
    x_start_index: i32,
    y_start_index: i32,
    offset: i32,
    c: char,
) -> Vec<RelativePosition> {
    let mut ret: Vec<RelativePosition> = Vec::new();
    for pos in position_to_check {
        let (x, y) = pos.index();
        match chars.get((y_start_index + y * offset) as usize) {
            Some(v) => match v.get((x_start_index + x * offset) as usize) {
                Some(value) => {
                    if *value == c {
                        ret.push(pos.clone());
                    }
                }
                _ => continue,
            },
            _ => continue,
        }
    }
    ret
}

fn solve1(data: &String) -> usize {
    let chars = parse(data);

    let mut counter = 0;
    for y in 0..chars.len() {
        for x in 0..chars[0].len() {
            let to_check = [
                RelativePosition::Before,
                RelativePosition::BeforeDown,
                RelativePosition::BeforeUp,
                RelativePosition::After,
                RelativePosition::AfterDown,
                RelativePosition::AfterUp,
                RelativePosition::Up,
                RelativePosition::Down,
            ];
            if chars[y][x] == 'X' {
                let to_check = check_char(&chars, &to_check, x as i32, y as i32, 1, 'M');
                let to_check = check_char(&chars, &to_check, x as i32, y as i32, 2, 'A');
                let to_check = check_char(&chars, &to_check, x as i32, y as i32, 3, 'S');
                counter += to_check.len();
            }
        }
    }
    counter
}

fn solve2(data: &String) -> i32 {
    let chars = parse(data);
    let mut counter = 0;
    for y in 0..chars.len() {
        for x in 0..chars[0].len() {
            let to_check = [
                RelativePosition::BeforeUp,
                RelativePosition::BeforeDown,
                RelativePosition::AfterDown,
                RelativePosition::AfterUp,
            ];
            if chars[y][x] == 'A' {
                let ms = check_char(&chars, &to_check, x as i32, y as i32, 1, 'M');
                let ss = check_char(&chars, &to_check, x as i32, y as i32, 1, 'S');
                if ms.len() == 2 && ss.len() == 2 {
                    if !((ms.contains(&RelativePosition::BeforeDown)
                        && ms.contains(&RelativePosition::AfterUp))
                        || (ms.contains(&RelativePosition::BeforeUp)
                            && ms.contains(&RelativePosition::AfterDown)))
                    {
                        counter += 1;
                    }
                }
            }
        }
    }
    counter
}

#[test]
fn part_one() {
    let res = solve1(&String::from(
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
    ));
    assert_eq!(18, res);
}

#[test]
fn part_two() {
    let res = solve2(&String::from(
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
    ));
    assert_eq!(9, res);
}
