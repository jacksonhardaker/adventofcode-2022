use std::fs;

fn part1(input: &str, alphabet: Vec<String>) {
    let items = input
        .split("\n")
        .map(|line| {
            // Split into equal compartments
            let compartments = line.split_at(line.len() / 2);

            // Find intersection of strings
            let mut matching_char = String::new();
            compartments.0.chars().any(|c| {
                if compartments.1.contains(c) {
                    matching_char = c.to_string();
                    return true;
                }
                return false;
            });

            return matching_char;
        })
        .collect::<Vec<String>>();

    let total = items
        .iter()
        .map(|c: &String| {
            let index = alphabet.iter().position(|r| r.eq(c)).unwrap();
            return (index + 1) as i32;
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();

    println!("Part 1: {}", total);
}

fn part2(input: &str, alphabet: Vec<String>) {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut total = 0;
    let mut i = 0;
    while i < lines.len() {
        let first = lines[i];
        let second = lines[i + 1];
        let third = lines[i + 2];

        // Find intersection of strings
        first.chars().any(|c| {
            if second.contains(c) && third.contains(c) {
                let index = alphabet.iter().position(|r| r.eq(&c.to_string())).unwrap();
                total += index + 1;
                // return (index + 1) as i32;
                return true;
            }
            return false;
        });

        i += 3;
    }
    println!("Part 2: {}", total);
}

fn main() {
    let input = fs::read_to_string("./days/03/input.txt").expect("Error!");

    // Create Vec of priority values where index + 1 is priority.
    let mut alphabet: Vec<String> = ('a'..='z').map(|c| c.to_string()).collect();
    let alphabet_upper: Vec<String> = ('A'..='Z').map(|c| c.to_string()).collect();
    alphabet.extend(alphabet_upper.iter().cloned());

    part1(input.trim(), alphabet.clone());
    part2(input.trim(), alphabet);
}
