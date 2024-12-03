use regex::Regex;

pub fn solve(data: String) {
    println!("Part one : {}", solve1(&data));
    println!("Part two : {}", solve2(&data));
}

fn solve1(data: &String) -> i32 {
    generic_solve(data)
}

fn solve2(data: &String) -> i32 {
    let re = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();
    let clean_data = re.replace_all(format!("{data}do()").as_str(), "").to_string();
    generic_solve(&clean_data)
}

fn generic_solve(data: &String) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let values: Vec<(i32, i32)> = re.captures_iter(data.as_str()).map(|caps| {
        let (_, [val1, val2]) = caps.extract();
        (val1.parse().unwrap(), val2.parse().unwrap())
    }).collect();
    values.into_iter().fold(0, |acc, (val1, val2)| acc + val1 * val2)
}

#[test]
fn part_one() {
    let res = solve1(&String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"));
    assert_eq!(161, res);
}

#[test]
fn part_two() {
    let res = solve2(&String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"));
    assert_eq!(48, res);
}