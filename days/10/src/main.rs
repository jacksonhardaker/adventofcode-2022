use std::{collections::HashMap, fs, str::Split};

fn get_sprite_positions_per_cycle(input: Split<&str>, cycles: i32) -> HashMap<i32, i32> {
    let mut register: HashMap<i32, i32> = HashMap::new();
    let mut x = 1;
    let mut index = 0;
    let instructions = input.collect::<Vec<&str>>();

    let mut buffer: Vec<i32> = vec![];

    for cycle in 1..=cycles {
        // Record x
        register.insert(cycle, x);

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

    register
}

fn part1(input: Split<&str>) -> i32 {
    let register = get_sprite_positions_per_cycle(input, 220);

    vec![20, 60, 100, 140, 180, 220]
        .iter()
        .map(|c| c * register.get(c).unwrap())
        .sum()
}

fn part2(input: Split<&str>) -> Result<(), ()> {
    let cycle_register = get_sprite_positions_per_cycle(input, 240);
    let mut sprite_pos = 0..=2;

    let mut cycle = 1;
    for _ in 1..=6 {
        println!("");
        for col in 0..40 {
            let x = cycle_register.get(&cycle).unwrap();
            sprite_pos = x - 1..=x + 1;

            if sprite_pos.contains(&col) {
                print!("#");
            } else {
                print!(".")
            }

            // fin
            cycle += 1;
        }
    }

    Ok(())
}

fn main() {
    let input = fs::read_to_string("./days/10/input.txt").expect("Error!");

    println!("Part 1: {}", part1(input.trim().split("\n")));
    println!("Part 2:");
    part2(input.trim().split("\n")).expect("Oops");
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
    assert_eq!(part2(input), Ok(()));
}
