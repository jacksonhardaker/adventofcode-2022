use std::{fs, str::Split};

fn part1(input: Split<&str>) -> usize {
    let mut visible_trees = 0;

    let grid: Vec<Vec<usize>> = input
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let lines_len = grid.len();

    for (i, line) in grid.iter().enumerate() {
        let chars_len = line.len();

        for (j, height) in line.iter().enumerate() {
            let is_edge = i == 0 || i == lines_len - 1 || j == 0 || j == chars_len - 1;
            // println!("{}, {} - {} - {}", i, j, height, is_edge);

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
    0
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
#[ignore]
fn test_part2() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part2(input), 0);
}
