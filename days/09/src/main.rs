use std::{collections::HashSet, fs, str::Split};

type Coords = (i32, i32);

fn part1(input: Split<&str>) -> usize {
    let mut visited_map: HashSet<Coords> = HashSet::new();

    let mut head: Coords = (0, 0);
    let mut tail: Coords = (0, 0);
    visited_map.insert(tail);

    input.for_each(|line| {
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap().parse::<i32>().unwrap();

        if direction == "U" {
            // move up by distance
            let mut moved = 0;
            while moved != distance {
                head.1 += 1;
                tail = calc_tail_pos(head, tail);
                visited_map.insert(tail);
                moved += 1;
            }
        } else if direction == "D" {
            // move down by distance
            let mut moved = 0;
            while moved != distance {
                head.1 -= 1;
                tail = calc_tail_pos(head, tail);
                visited_map.insert(tail);
                moved += 1;
            }
        } else if direction == "L" {
            // move left by distance
            let mut moved = 0;
            while moved != distance {
                head.0 += 1;
                tail = calc_tail_pos(head, tail);
                visited_map.insert(tail);
                moved += 1;
            }
        } else if direction == "R" {
            // move right by distance
            let mut moved = 0;
            while moved != distance {
                head.0 -= 1;
                tail = calc_tail_pos(head, tail);
                visited_map.insert(tail);
                moved += 1;
            }
        }
    });

    visited_map.len()
}

fn part2(input: Split<&str>) -> usize {
    0
}

fn calc_tail_pos(head: Coords, tail: Coords) -> Coords {
    let same_x = head.0.abs_diff(tail.0) == 0;
    let same_y = head.1.abs_diff(tail.1) == 0;

    // Tail is already adjacent
    if same_x && (head.1.abs_diff(tail.1) == 1 || same_y)
        || same_y && (head.0.abs_diff(tail.0) == 1 || same_x)
        || head.1.abs_diff(tail.1) == 1 && head.0.abs_diff(tail.0) == 1
    {
        return tail;
    }

    // H and T are on the same x-axis
    if same_x {
        let y = if head.1 - tail.1 > 0 {
            head.1 - 1
        } else {
            head.1 + 1
        };
        return (tail.0, y);
    }

    // H and T are on the same y-axis
    if same_y {
        let x = if head.0 - tail.0 > 0 {
            head.0 - 1
        } else {
            head.0 + 1
        };
        return (x, tail.1);
    }

    // H and T share neither x or y axis - tail always moves one step diagonally to keep up
    let x = if head.0 > tail.0 {
        tail.0 + 1
    } else {
        tail.0 - 1
    };
    let y = if head.1 > tail.1 {
        tail.1 + 1
    } else {
        tail.1 - 1
    };

    (x, y)
}

fn main() {
    let input = fs::read_to_string("./days/09/input.txt").expect("Error!");

    println!("Part 1: {}", part1(input.trim().split("\n")));
    println!("Part 2: {}", part2(input.trim().split("\n")));
}

#[test]
fn test_calc_tail_pos() {
    assert_eq!(calc_tail_pos((2, 6), (2, 6)), (2, 6));
    assert_eq!(calc_tail_pos((2, 0), (1, 0)), (1, 0));
    assert_eq!(calc_tail_pos((1, 1), (0, 0)), (0, 0));
    assert_eq!(calc_tail_pos((2, 0), (0, 0)), (1, 0));
    assert_eq!(calc_tail_pos((2, -1), (2, -3)), (2, -2));
    assert_eq!(calc_tail_pos((1, 2), (0, 0)), (1, 1));
    assert_eq!(calc_tail_pos((1, 0), (-1, -1)), (0, 0));
}

#[test]
fn test_part1() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part1(input), 13);
}
#[test]
fn test_part2() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part2(input), 0);
}
