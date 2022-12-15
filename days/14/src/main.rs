use std::{collections::HashSet, fs, str::Split};

const START: (i32, i32) = (500, 0);

fn part1(input: Split<&str>) -> usize {
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();
    let mut max_depth: i32 = 0;
    input.for_each(|line| {
        let mut points = line.split(" -> ").map(|point| {
            let mut parts = point.split(",");
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        });

        let mut prev = points.next().unwrap();
        while let Some(current) = points.next() {
            occupied.insert(prev);

            if prev.0 == current.0 {
                let modifier: i32 = if prev.1 < current.1 { 1 } else { -1 };
                let mut pos = prev.1;
                while pos != current.1 {
                    pos += modifier;
                    if pos > max_depth {
                        max_depth = pos;
                    }
                    occupied.insert((prev.0, pos));
                }
            } else if prev.1 == current.1 {
                let modifier: i32 = if prev.0 < current.0 { 1 } else { -1 };
                let mut pos = prev.0;
                while pos != current.0 {
                    pos += modifier;
                    occupied.insert((pos, prev.1));
                }
            }
            prev = current;
        }
    });

    let mut sand_at_rest = 0;
    let mut sand = START.clone();
    loop {
        if !occupied.contains(&(sand.0, sand.1 + 1)) {
            sand.1 += 1;
        } else if !occupied.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand.1 += 1;
            sand.0 -= 1;
        } else if !occupied.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand.1 += 1;
            sand.0 += 1;
        } else {
            occupied.insert(sand);
            sand_at_rest += 1;
        }

        if occupied.contains(&sand) {
            sand = START.clone();
        }

        if sand.1 > max_depth {
            break;
        }
    }

    sand_at_rest
}

fn part2(input: Split<&str>) -> usize {
    0
}

fn main() {
    let input = fs::read_to_string("./days/14/input.txt").expect("Error!");

    println!("Part 1: {}", part1(input.trim().split("\n")));
    println!("Part 2: {}", part2(input.trim().split("\n")));
}

#[test]
fn test_part1() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part1(input), 24);
}
#[test]
fn test_part2() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part2(input), 0);
}
