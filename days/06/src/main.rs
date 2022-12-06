use std::collections::{HashSet, VecDeque};
use std::fs;

fn part1(datastream: String) -> usize {
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

    // println!("Last four: {:#?}", last_four);

    marker_index + 1
}

fn _part2() {}

fn main() {
    let _input = fs::read_to_string("./days/06/input.txt").expect("Error!");

    println!("Part 1: {}", part1(_input.trim().to_string()));
}

#[test]
fn test_part1() {
    assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned()), 7);
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned()), 5);
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg".to_owned()), 6);
    assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned()), 10);
    assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned()), 11);
}
