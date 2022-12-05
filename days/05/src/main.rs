use regex::Regex;
use std::collections::VecDeque;
use std::fs;

fn _part1() {}

fn main() {
    let input = fs::read_to_string("./days/05/test-input.txt").expect("Error!");
    let mut parts = input.split("\n\n");

    // Split into stacks and instructions
    let initial_stacks = parts.next().unwrap();
    let instructions = parts.next().unwrap();

    // Get total number of stacks
    let re = Regex::new(r"\d\s*$").unwrap();
    let values = re.captures(initial_stacks).unwrap();
    let num_of_stacks = values
        .get(0)
        .unwrap()
        .as_str()
        .trim()
        .parse::<i32>()
        .unwrap();

    // Init stacks
    let mut stacks: Vec<VecDeque<String>> =
        vec![VecDeque::new(); num_of_stacks.try_into().unwrap()];

    // Populate stacks
    let item_re = Regex::new(r"\[([A-Z])\]").unwrap();
    for cap in item_re.captures_iter(initial_stacks) {
        // Stack index to push into
        let stack_pos = ((cap.get(0).unwrap().start() % (num_of_stacks * 4) as usize) as f64
            / num_of_stacks as f64)
            .floor() as usize;

        // Value to push
        let val = (&cap[1]).to_owned();

        stacks[stack_pos].push_front(val);
    }

    let instr_re = Regex::new(r"\d+").unwrap();
    instructions.trim().split("\n").for_each(|instruction| {
        let digits: Vec<_> = instr_re
            .captures_iter(instruction)
            .map(|digit| (&digit[0]).to_owned().parse::<i32>().unwrap())
            .collect();

        let move_count = digits[0] as usize;
        let move_from = digits[1] as usize;
        let move_to = digits[2] as usize;

        for _ in 0..move_count {
            let moved_val = stacks[move_from - 1].pop_back();
            stacks[move_to - 1].push_back(moved_val.unwrap());
        }
    });

    let mut result: String = "".to_owned();

    for i in 0..num_of_stacks {
        result.push_str(&stacks[i as usize].pop_back().unwrap());
    }

    println!("Part 1: {}", result);
}
