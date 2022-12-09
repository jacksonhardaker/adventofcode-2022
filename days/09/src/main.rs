use std::{collections::HashMap, collections::HashSet, fs, str::Split};

type Coords = (i32, i32);

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
            tail.1 + 1
        } else {
            tail.1 - 1
        };
        return (tail.0, y);
    }

    // H and T are on the same y-axis
    if same_y {
        let x = if head.0 - tail.0 > 0 {
            tail.0 + 1
        } else {
            tail.0 - 1
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

fn flip_tail(rope: &mut Vec<Coords>, visited_map: &mut HashSet<(i32, i32)>) {
    for i in 1..rope.len() {
        let prev = *rope.get(i - 1).unwrap();
        rope.get_mut(i)
            .map(|knot| *knot = calc_tail_pos(prev, *knot));
        visited_map.insert(rope[rope.len() - 1]);
    }
}

fn simulate_rope(input: Split<&str>, rope_len: usize) -> usize {
    let dir_mod = HashMap::from([("U", 1), ("D", -1), ("L", -1), ("R", 1)]);
    let mut visited_map: HashSet<Coords> = HashSet::new();
    let mut rope: Vec<Coords> = (0..rope_len).map(|_| (0, 0)).collect();

    visited_map.insert(rope[rope.len() - 1]);

    input.for_each(|line| {
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..distance {
            rope.get_mut(0).map(|head| {
                if direction == "U" || direction == "D" {
                    head.1 += dir_mod[direction]
                } else if direction == "L" || direction == "R" {
                    head.0 += dir_mod[direction]
                }
            });
            flip_tail(&mut rope, &mut visited_map);
        }
    });

    visited_map.len()
}

fn part1(input: Split<&str>) -> usize {
    simulate_rope(input, 2)
}

fn part2(input: Split<&str>) -> usize {
    simulate_rope(input, 10)
}

/**
 * Start Helpers
 */
fn _print_rope(rope: Vec<Coords>) {
    for y in (-5..=15).rev() {
        for x in -11..=14 {
            let mut knotted = false;
            for (i, knot) in rope.iter().enumerate() {
                if knot.0 == x && knot.1 == y {
                    if i == 0 {
                        print!("H");
                    } else {
                        print!("{}", i)
                    }
                    knotted = true;
                    break;
                }
            }

            if !knotted {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn _print_visited_map(map: HashSet<Coords>) {
    for y in (-5..=15).rev() {
        for x in -11..=14 {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
}

/**
 * End Helpers
 */

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
fn test_part2_a() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part2(input), 1);
}

#[test]
fn test_part2_b() {
    let raw_input = fs::read_to_string("./test-input-2.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part2(input), 36);
}
