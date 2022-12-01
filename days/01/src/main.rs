use std::fs;

fn main() {
    let data = fs::read_to_string("./days/01/input.txt").expect("Error!");
    let split_data: Vec<i32> = data
        .trim()
        .split("\n\n")
        .map(|elf| {
            let nums: Vec<i32> = elf.split("\n").map(|num| num.parse().unwrap()).collect();
            return nums.iter().sum();
        })
        .collect();

    let max = split_data.iter().max().unwrap();
    println!("Part 1: {}", max);

    let mut part_2_data = split_data.clone();
    part_2_data.sort();

    let last_3 = &part_2_data[part_2_data.len() - 3..];
    let final_sum: i32 = last_3.iter().sum();

    println!("Part 2: {}", final_sum);
}
