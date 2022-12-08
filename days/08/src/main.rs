use std::{fs, str::Split};

fn build_tree_grid(input: Split<&str>) -> Vec<Vec<usize>> {
    input
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: Split<&str>) -> usize {
    let mut visible_trees = 0;

    let grid = build_tree_grid(input);
    let lines_len = grid.len();

    for (i, line) in grid.iter().enumerate() {
        let chars_len = line.len();

        for (j, height) in line.iter().enumerate() {
            let is_edge = i == 0 || i == lines_len - 1 || j == 0 || j == chars_len - 1;

            if is_edge {
                // All edges are visible
                visible_trees += 1;
            } else {
                let visible_from_top = (0..=i - 1).map(|ii| grid[ii][j]).max().unwrap() < *height;
                let visible_from_bottom =
                    (i + 1..lines_len).map(|ii| grid[ii][j]).max().unwrap() < *height;
                let visible_from_left = (0..=j - 1).map(|jj| grid[i][jj]).max().unwrap() < *height;
                let visible_from_right =
                    (j + 1..chars_len).map(|jj| grid[i][jj]).max().unwrap() < *height;

                if visible_from_top
                    || visible_from_bottom
                    || visible_from_left
                    || visible_from_right
                {
                    visible_trees += 1;
                }
            }
        }
    }

    visible_trees
}

fn part2(input: Split<&str>) -> usize {
    let mut max_view = 0;
    let grid = build_tree_grid(input);
    let y_limit = grid.len();

    for (y, line) in grid.iter().enumerate() {
        let x_limit = line.len();

        for (x, target_height) in line.iter().enumerate() {
            let mut up_blocked = false;
            let up_view = if y == 0 {
                0
            } else {
                (0..=y - 1)
                    .rev()
                    .map(|yy| {
                        if up_blocked {
                            return 0;
                        }

                        if grid[yy][x] >= *target_height {
                            up_blocked = true;
                        }

                        1
                    })
                    .sum()
            };
            let mut down_blocked = false;
            let down_view = if y == y_limit - 1 {
                0
            } else {
                (y + 1..y_limit)
                    .map(|yy| {
                        if down_blocked {
                            return 0;
                        }

                        if grid[yy][x] >= *target_height {
                            down_blocked = true;
                        }

                        1
                    })
                    .sum()
            };
            let mut left_blocked = false;
            let left_view = if x == 0 {
                0
            } else {
                (0..=x - 1)
                    .rev()
                    .map(|xx| {
                        if left_blocked {
                            return 0;
                        }

                        if grid[y][xx] >= *target_height {
                            left_blocked = true;
                        }

                        1
                    })
                    .sum()
            };
            let mut right_blocked = false;
            let right_view = if x == x_limit - 1 {
                0
            } else {
                (x + 1..x_limit)
                    .map(|xx| {
                        if right_blocked {
                            return 0;
                        }

                        if grid[y][xx] >= *target_height {
                            right_blocked = true;
                        }

                        1
                    })
                    .sum()
            };

            let current_view = up_view * down_view * left_view * right_view;
            // println!("pos: {}/{}, val: {}, up_view: {}, down_view: {}, left_view: {}, right_view: {}, view: {}", y, x, target_height, up_view, down_view, left_view, right_view, current_view);
            max_view = if current_view > max_view {
                current_view
            } else {
                max_view
            };
        }
    }

    max_view
}

fn main() {
    let input = fs::read_to_string("./days/08/input.txt").expect("Error!");

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
    assert_eq!(part2(input), 8);
}
