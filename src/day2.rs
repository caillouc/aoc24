pub fn solve(data: String) {
    println!("Part one : {}", solve1(&data));
    println!("Part two : {}", solve2(&data));
}

pub fn parse(data: & String) -> Vec<Vec<i32>> {
    data.lines()
        .map(|l| 
            l.split(' ')
             .map(|i| 
                i.parse::<i32>()
                 .unwrap())
             .collect())
        .collect()
} 



fn is_safe(level: &[i32], resilient: bool) -> bool {
    let increase = level.get(0) < level.get(1);
    let mut previous = level.get(0).unwrap();
    let mut first_fault = false;
    let mut nb_check_done = 0;
    for i in &level[1..] {
        if !increase && i > previous || 
            increase && i < previous ||
            (i - previous).abs() < 1 ||
            (i - previous).abs() > 3 {
            if !resilient {
                return false
            } else if first_fault {
                // If more than one check have been done, we can be confident that
                // `increase` var is correct
                // With this check we avoid the recursive call
                if nb_check_done > 1 {
                    return false
                } else {
                    // We can skip the first or the second element
                    return is_safe(&level[1..], false) || 
                        is_safe(&[&[level[0]], &level[2..]].concat(), false);
                }
            } else {
                first_fault = true;
                continue;
            }
        }     
        previous = i;
        nb_check_done += 1;
    }
    true
}


fn solve1(data: &String) -> usize {
    let levels = parse(data);
    levels.iter().map(|l| is_safe(l, false)).filter(|b| *b).count()
}

fn solve2(data: &String) -> usize {
    let levels = parse(data);
    levels.iter().map(|l| is_safe(l, true)).filter(|b| *b).count()
}

#[test]
fn part_one() {
    let res = solve1(&String::from("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"));
    assert_eq!(2, res);
}

#[test]
fn part_two() {
    let res = solve2(&String::from("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"));
    assert_eq!(4, res);
}


#[test]
fn custom_test() {
    let res = solve2(&String::from("5 8 4 3 2 1
10 5 4 3 2 1
5 10 4 3 2 1
"));
    assert_eq!(3, res)
}