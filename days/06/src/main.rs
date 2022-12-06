use std::collections::{HashSet, VecDeque};
use std::fs;

fn parse_stream(datastream: String, buffer_size: usize) -> usize {
    let mut buffer: VecDeque<char> = VecDeque::new();
    let mut marker_index: usize = 0;
    for (index, char) in datastream.chars().enumerate() {
        buffer.push_back(char);

        // First four cannot be the marker
        if index >= buffer_size {
            buffer.pop_front();

            let is_unique =
                buffer.len() == buffer.clone().into_iter().collect::<HashSet<_>>().len();
            if is_unique {
                marker_index = index;
                break;
            }
        }
    }

    // println!("Last four: {:#?}", last_four);

    marker_index + 1
}

fn part1(datastream: String) -> usize {
    parse_stream(datastream, 4)
}

fn part2(datastream: String) -> usize {
    parse_stream(datastream, 14)
}

fn main() {
    let _input = fs::read_to_string("./days/06/input.txt").expect("Error!");

    println!("Part 1: {}", part1(_input.trim().to_string()));
    println!("Part 2: {}", part2(_input.trim().to_string()));
}

#[test]
fn test_part1() {
    assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned()), 7);
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned()), 5);
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg".to_owned()), 6);
    assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned()), 10);
    assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned()), 11);
}

#[test]
fn test_part2() {
    assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned()), 19);
    assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned()), 23);
    assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg".to_owned()), 23);
    assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned()), 29);
    assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned()), 26);
}
