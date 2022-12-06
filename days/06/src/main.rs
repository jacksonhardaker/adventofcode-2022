use std::collections::{HashSet, VecDeque};
use std::fs;

fn part1(datastream: String) {
    let mut last_four: VecDeque<char> = VecDeque::new();
    let mut marker_index: usize = 0;
    for (index, char) in datastream.chars().enumerate() {
        last_four.push_back(char);

        // First four cannot be the marker
        if index >= 4 {
            last_four.pop_front();

            let is_unique =
                last_four.len() == last_four.clone().into_iter().collect::<HashSet<_>>().len();
            if is_unique {
                marker_index = index;
                break;
            }
        }
    }

    println!("Last four: {:#?}", last_four);
    println!("Part 1: {}", marker_index + 1);
}

fn _part2() {}

fn main() {
    let _input = fs::read_to_string("./days/06/input.txt").expect("Error!");

    // part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned());
    // part1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned());
    // part1("nppdvjthqldpwncqszvftbrmjlhg".to_owned());
    // part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned());
    // part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned());
    part1(_input.trim().to_string());
}
