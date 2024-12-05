use std::collections::{HashMap, HashSet};

pub fn solve(data: String) {
    println!("Part one : {}", solve1(&data));
    println!("Part two : {}", solve2(&data));
}

#[derive(Debug)]
struct Rule {
    before: i32,
    after: i32,
}

fn parse(data: &String) -> (Vec<Rule>, Vec<Vec<i32>>) {
    let mut rules = vec![];
    let mut updates = vec![];
    let mut rules_done = false;
    for l in data.lines() {
        if l.is_empty() && !rules_done {
            rules_done = true;
            continue;
        }
        if !rules_done {
            let mut split = l.split('|');
            rules.push(Rule {
                before: split.next().unwrap().parse().unwrap(),
                after: split.next().unwrap().parse().unwrap(),
            });
        } else {
            updates.push(l.split(',').map(|i| i.parse().unwrap()).collect());
        }
    }
    (rules, updates)
}

fn get_not_allowed_map(rules: &Vec<Rule>) -> HashMap<i32, Vec<i32>> {
    let mut not_allowed: HashMap<i32, Vec<i32>> = HashMap::new();
    rules.iter().for_each(|Rule { before, after }| {
        match not_allowed.get_mut(after) {
            Some(v) => v.push(*before),
            _ => {
                not_allowed.insert(*after, vec![*before]);
            }
        };
    });
    not_allowed
}

fn solve1(data: &String) -> i32 {
    let (rules, updates) = parse(data);
    let not_allowed = get_not_allowed_map(&rules);
    let valid = updates.iter().filter(|u| {
        let mut invalid: HashSet<i32> = HashSet::new();
        !u.iter().any(|page| {
            invalid.extend(not_allowed.get(page).unwrap_or(&Vec::new()));
            invalid.contains(page)
        })
    });
    valid.fold(0, |acc, v| v.get(v.len() / 2).unwrap() + acc)
}

fn solve2(data: &String) -> i32 {
    let (rules, updates) = parse(data);
    let not_allowed = get_not_allowed_map(&rules);
    let invalid_update = updates.iter().filter(|u| {
        let mut invalid: HashSet<i32> = HashSet::new();
        u.iter().any(|page| {
            invalid.extend(not_allowed.get(page).unwrap_or(&Vec::new()));
            invalid.contains(page)
        })
    });
    let mut ret = 0;
    for u in invalid_update {
        let mut counter_map = HashMap::new();
        for r in rules
            .iter()
            .filter(|r| u.contains(&r.before) && u.contains(&r.after))
        {
            match counter_map.get(&r.before) {
                Some(value) => {counter_map.insert(r.before, value + 1);}
                _ => {counter_map.insert(r.before, 1);}
            }
            match counter_map.get(&r.after) {
                Some(value) => {counter_map.insert(r.after, value - 1);}
                _ => {counter_map.insert(r.after, -1);}
            }
        }
        let mut zipped: Vec<(&i32, &i32)> = counter_map.values().zip(counter_map.keys()).collect();
        zipped.sort();
        zipped.reverse();
        let res: Vec<i32> = zipped.iter().map(|(_, value)| **value).collect();
        ret += res.get(res.len()/2).unwrap();
    }
    ret
}

#[test]
fn part_one() {
    let res = solve1(&String::from(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    ));
    assert_eq!(143, res);
}

#[test]
fn part_two() {
    let res = solve2(&String::from(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    ));
    assert_eq!(123, res);
}
