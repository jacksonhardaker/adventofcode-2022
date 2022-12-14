use std::{fs, str::Split};

fn part1(input: Split<&str>) -> usize {
    let results: usize = input
        .map(|pair| {
            let mut split_pair = pair.trim().split("\n");
            let left = string_to_list(split_pair.next().unwrap());
            let right = string_to_list(split_pair.next().unwrap());

            compare_lists(left, right)
        })
        .enumerate()
        .map(|(index, result)| {
            if result == 1 {
                return index + 1;
            }

            return 0;
        })
        .sum();

    results
}

fn part2(input: &str) -> usize {
    let divider_packets = ("[[2]]", "[[6]]");
    let mut new_input = String::new();
    new_input.push_str(&input.replace("\n\n", "\n"));
    new_input.push_str("\n");
    new_input.push_str(divider_packets.0);
    new_input.push_str("\n");
    new_input.push_str(divider_packets.1);

    let mut vec: Vec<&str> = new_input.split("\n").collect();

    vec.sort_by(|a, b| {
        let left = string_to_list(a);
        let right = string_to_list(b);
        let result = compare_lists(left, right);

        if result == 1 {
            std::cmp::Ordering::Less
        } else if result == -1 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    let first = vec.iter().position(|packet| packet.eq(&divider_packets.0)).unwrap() + 1;
    let second = vec.iter().position(|packet| packet.eq(&divider_packets.1)).unwrap() + 1;

    first * second
}

fn main() {
    let input = fs::read_to_string("./days/13/input.txt").expect("Error!");

    println!("Part 1: {}", part1(input.trim().split("\n\n")));
    println!("Part 2: {}", part2(input.trim()));
}

#[derive(Debug, Clone)]
enum ListItem {
    N(u32),
    L(List),
}

type List = Vec<ListItem>;

fn compare_lists(left: ListItem, right: ListItem) -> i32 {
    // If both values are integers, the lower integer should come first.
    if matches!(left.clone(), ListItem::N(_)) && matches!(right.clone(), ListItem::N(_)) {
        if let ListItem::N(left_int) = left.clone() {
            if let ListItem::N(right_int) = right.clone() {
                if left_int < right_int {
                    return 1;
                } else if right_int < left_int {
                    return -1;
                } else {
                    return 0;
                }
            }
        }
    }

    // If both values are lists, compare the first value of each list, then the second value, and so on
    if matches!(left.clone(), ListItem::L(_)) && matches!(right.clone(), ListItem::L(_)) {
        if let ListItem::L(left_list) = left.clone() {
            if let ListItem::L(right_list) = right.clone() {
                let left_len = left_list.len();
                let right_len = right_list.len();

                let max = if left_len > right_len {
                    left_len
                } else {
                    right_len
                };

                for index in 0..max {
                    if index >= left_len {
                        return 1;
                    } else if index >= right_len {
                        return -1;
                    }
                    let result = compare_lists(left_list[index].clone(), right_list[index].clone());
                    if result == 0 {
                    } else {
                        return result;
                    }
                }
            }
        }
    }

    // If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison.
    if matches!(left.clone(), ListItem::N(_)) && matches!(right.clone(), ListItem::L(_)) {
        let left = ListItem::L(vec![left.clone()]);
        return compare_lists(left, right);
    }

    if matches!(left.clone(), ListItem::L(_)) && matches!(right.clone(), ListItem::N(_)) {
        let right = ListItem::L(vec![right.clone()]);
        return compare_lists(left, right);
    }

    0
}

fn string_to_list(s: &str) -> ListItem {
    let mut result = List::new();

    let mut chars = s.chars();
    // Skip leading '['
    chars.next();

    while let Some(c) = chars.next() {
        match c {
            '[' => {
                let mut inner_string = String::new();
                inner_string.push_str("[");
                while let Some(c) = chars.next() {
                    inner_string.push(c);
                    if c == ']' {
                        result.push(string_to_list(&inner_string));
                        break;
                    }
                }
            }
            ']' => {}
            ',' => {}
            c => {
                let mut num = String::new();
                num.push(c);
                while let Some(c) = chars.next() {
                    if c == ',' || c == ']' {
                        break;
                    }
                    num.push(c);
                }

                result.push(ListItem::N(num.parse::<u32>().unwrap()));
            }
        }
    }

    ListItem::L(result)
}

#[test]
fn test_part1() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n\n");
    assert_eq!(part1(input), 13);
}
#[test]
fn test_part2() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    assert_eq!(part2(raw_input.trim()), 140);
}
