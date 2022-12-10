use std::{collections::HashMap, fs, str::Split};

fn part1(input: Split<&str>) -> i32 {
    let mut register: HashMap<i32, i32> = HashMap::new();
    let mut x = 1;
    let mut index = 0;
    let instructions = input.collect::<Vec<&str>>();

    let mut buffer: Vec<i32> = vec![];

    for cycle in 1..=220 {
        if cycle == 20
            || cycle == 60
            || cycle == 100
            || cycle == 140
            || cycle == 180
            || cycle == 220
        {
            // Record x
            register.insert(cycle, cycle * x);
        }

        // println!("Cycle: {}, X: {}", cycle, x);

        if buffer.len() != 0 {
            x += buffer.pop().unwrap();
        } else {
            if index < instructions.len() {
                let mut instruction = instructions[index].split(" ");
                let left = instruction.next().unwrap();

                if left == "noop" {
                    // Do nothing
                    index += 1;
                } else if left == "addx" {
                    buffer.push(instruction.next().unwrap().parse::<i32>().unwrap());
                    index += 1;
                }
            }
        }
    }

    register.values().sum::<i32>()
}

fn part2(input: Split<&str>) -> usize {
    0
}

fn main() {
    let input = fs::read_to_string("./days/10/input.txt").expect("Error!");

    println!("Part 1: {}", part1(input.trim().split("\n")));
    println!("Part 2: {}", part2(input.trim().split("\n")));
}

#[test]
fn test_part1() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part1(input), 13140);
}
#[test]
fn test_part2() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part2(input), 0);
}
