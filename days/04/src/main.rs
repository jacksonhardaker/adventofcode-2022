use std::fs;
use std::ops::RangeInclusive;

type Input = Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>;

fn part1(input: Input) {
    let mut contained_count = 0;
    input.iter().for_each(|pair| {
        if (pair.0.contains(pair.1.start()) && pair.0.contains(pair.1.end()))
            || (pair.1.contains(pair.0.start()) && pair.1.contains(pair.0.end()))
        {
            // One of the ranges _fully_ includes the other
            contained_count += 1;
        }
    });

    println!("Part 1: {}", contained_count);
}

fn part2(input: Input) {
    let mut contained_count = 0;
    input.iter().for_each(|pair| {
        if (pair.0.contains(pair.1.start()))
            || (pair.0.contains(pair.1.end()))
            || (pair.1.contains(pair.0.start()))
            || (pair.1.contains(pair.0.end()))
        {
            // One of the ranges _partially_ includes the other
            contained_count += 1;
        }
    });

    println!("Part 2: {}", contained_count);
}

fn main() {
    let input = fs::read_to_string("./days/04/input.txt").expect("Error!");

    // Parse input
    let parsed_input: Input = input
        .trim()
        .split("\n")
        .map(|line| {
            let mut ranges = line.split(",").map(|range| {
                let mut split_range = range.split("-");
                let start = split_range.next().unwrap().parse::<i32>().unwrap();
                let end = split_range.next().unwrap().parse::<i32>().unwrap();

                start..=end
            });

            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .collect();

    // println!("{:#?}", parsed_input);

    part1(parsed_input.clone());
    part2(parsed_input.clone());
}
