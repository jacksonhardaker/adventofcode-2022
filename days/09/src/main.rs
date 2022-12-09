use std::{fs, str::Split};

fn part1(input: Split<&str>) -> usize {
    0
}

fn part2(input: Split<&str>) -> usize {
    0
}

fn main() {
    let input = fs::read_to_string("./days/XX/input.txt").expect("Error!");

    println!("Part 1: {}", part1(input.trim().split("\n")));
    println!("Part 2: {}", part2(input.trim().split("\n")));
}

#[test]
fn test_part1() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part1(input), 21);
}
#[test]
fn test_part2() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part2(input), 0);
}
