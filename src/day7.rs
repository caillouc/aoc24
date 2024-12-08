pub fn solve(data: String) {
    println!("Part one : {}", solve1(&data));
    println!("Part two : {}", solve2(&data));
}

struct Entry {
    result: i64,
    values: Vec<i64>,
}

fn parse(data: &String) -> Vec<Entry> {
    let mut ret = vec![];
    data.lines().for_each(|l| {
        let mut split = l.split(": ");
        let key: i64 = split.next().unwrap().parse().unwrap();
        let values: Vec<i64> = split
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        ret.push(Entry {
            result: key,
            values,
        });
    });
    ret
}

fn is_valid(result: i64, values: &[i64], concatenation: bool) -> bool {
    let last = *values.last().unwrap();
    if values.len() == 1 {
        return result == last;
    } else {
        if result % last == 0 {
            if is_valid(result / last, &values[..values.len() - 1], concatenation) {
                return true;
            }
        }
        if result - last >= 0 {
            if is_valid(result - last, &values[..values.len() - 1], concatenation) {
                return true;
            }
        }
        if concatenation {
            if result.to_string().ends_with(last.to_string().as_str()) {
                if is_valid(
                    result.to_string().as_str()[..result.to_string().len() - last.to_string().len()]
                        .parse()
                        .unwrap_or(0),
                    &values[..values.len() - 1],
                    concatenation,
                ) {
                    return true;
                }
            }
        }
        false
    }
}

fn solve1(data: &String) -> i64 {
    let entries = parse(data);
    entries
        .iter()
        .filter(|Entry { result, values }| is_valid(*result, values, false))
        .fold(0, |acc, v| acc + v.result)
}

fn solve2(data: &String) -> i64 {
    let entries = parse(data);
    entries
        .iter()
        .filter(|Entry { result, values }| is_valid(*result, values, true))
        .fold(0, |acc, v| acc + v.result)
}

#[test]
fn part_one() {
    let res = solve1(&String::from(
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
",
    ));
    assert_eq!(3749, res);
}

#[test]
fn part_two() {
    let res = solve2(&String::from(
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
",
    ));
    assert_eq!(11387, res);
}
