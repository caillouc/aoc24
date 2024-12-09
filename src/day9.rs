pub fn solve(data: String) {
    println!("Part one : {}", solve1(&data));
    println!("Part two : {}", solve2(&data));
}

fn parse(data: &String) -> Vec<u32> {
    data.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn solve1(data: &String) -> usize {
    let parsed = parse(data);
    let nb_elem = parsed
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .fold(0, |acc, (_, value)| acc + value);
    let max_value = parsed.len() / 2;
    let mut is_free = false;
    let mut current_pos = 0;
    let mut count = 0;
    let mut left_value = 0;
    let mut rigth_value = max_value;
    let mut rigth_index = parsed.len() - 1;
    let mut rigth_counter = parsed[rigth_index];
    for c in &parsed {
        let mut temp = match is_free {
            false => left_value,
            true => rigth_value,
        };
        for _ in 0..*c {
            count += current_pos * temp;
            current_pos += 1;
            if current_pos >= nb_elem as usize {
                break;
            }

            if is_free {
                rigth_counter -= 1;
                if rigth_counter == 0 {
                    rigth_value -= 1;
                    temp = rigth_value;
                    rigth_index -= 2;
                    rigth_counter = parsed[rigth_index];
                }
            }
        }
        if current_pos >= nb_elem as usize {
            break;
        }
        if !is_free {
            left_value += 1;
        }
        is_free = !is_free;
    }
    count
}

fn get_next_available(available: &Vec<usize>, c: u32, parsed: &Vec<u32>) -> Option<usize> {
    for option in available.iter().rev() {
        if parsed[option * 2] <= c {
            return Some(*option);
        }
    }
    None
}
fn solve2(data: &String) -> usize {
    let parsed = parse(data);
    let mut left_value: usize = 0;
    let mut is_free = false;
    let mut count = 0;
    let mut current_pos = 0;
    let mut rigth_counter: i32 = 0;
    let max_value = parsed.len() / 2;
    let mut available: Vec<usize> = (0..max_value + 1).collect();
    for c in &parsed {
        let mut temp = match is_free {
            false => {
                if available.contains(&left_value) {
                    left_value
                } else {
                    0
                }
            }
            true => 0,
        };
        for i in 0..*c {
            if is_free && rigth_counter == 0 {
                temp = match get_next_available(&available, *c - i, &parsed) {
                    Some(val) => {
                        rigth_counter = parsed[val * 2] as i32;
                        let index = available.iter().position(|e| *e == val).unwrap();
                        available.remove(index);
                        val
                    }
                    None => 0,
                }
            }
            count += current_pos * temp;
            current_pos += 1;
            if is_free && rigth_counter > 0 {
                rigth_counter -= 1;
            }
        }
        if !is_free {
            if left_value == temp {
                let index = available.iter().position(|e| *e == temp).unwrap();
                available.remove(index);
            }
            left_value += 1;
        }
        is_free = !is_free;
    }
    count
}

#[test]
fn part_one() {
    let res = solve1(&String::from("2333133121414131402"));
    assert_eq!(1928, res);
}

#[test]
fn part_two() {
    let res = solve2(&String::from("2333133121414131402"));
    assert_eq!(2858, res);
}

// 0...1...2......33333
// 0...1...233333......
// 0...1...233333......
// 02..1....33333......
// 021......33333......
// 021......33333......
#[test]
fn additional_check() {
    let res = solve2(&String::from("1313165"));
    assert_eq!(169, res);
} 
