use std::collections::HashMap;

pub fn solve(data: String) {
    println!("The solution for the first problem of day 1 is : {}", solve1(&data));
    println!("The solution for the second problem of day 1 is : {}", solve2(&data));
}

fn parse(data: &String) -> (Vec<i32>, Vec<i32>) {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    data.lines().for_each(|l| {
        let mut split = l.split("   ");
        first_list.push(split.next().unwrap().parse().unwrap());
        second_list.push(split.next().unwrap().parse().unwrap());
    });

    (first_list, second_list)
}

fn solve1(data: &String) -> i32 {
    let (mut first_list, mut second_list) = parse(&data);

    first_list.sort();
    second_list.sort();

    first_list.iter().zip(&second_list).map(|(a, b)| (a - b).abs()).sum()
}

fn solve2(data: &String) -> i32 {
    let (first_list, second_list) = parse(&data);

    let mut second_list_count: HashMap<i32, i32>= HashMap::new();
    second_list.iter().for_each(|k| {
        second_list_count.insert(*k, *second_list_count.get(k).unwrap_or(&0) + 1);
    });

    first_list.iter().map(|e| e * second_list_count.get(e).unwrap_or(&0)).sum()
}

#[test]
fn part_one() {
    let res = solve1(&String::from("3   4
4   3
2   5
1   3
3   9
3   3"));
    assert_eq!(11, res);
}

#[test]
fn part_two() {
    let res = solve2(&String::from("3   4
4   3
2   5
1   3
3   9
3   3"));
    assert_eq!(31, res);
}