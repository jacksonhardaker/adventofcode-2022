use regex::Regex;
use std::{collections::HashMap, fs, str::Split};

const MAXIMUM_DISK_SPACE: usize = 70000000;
const REQUIRED_DISK_SPACE: usize = 30000000;

fn get_dirs(input: Split<&str>) -> HashMap<String, usize> {
    let mut dirs: HashMap<String, usize> = HashMap::new();
    let mut current_path: Vec<String> = Vec::new();

    input.for_each(|line| {
        if line.starts_with("$") {
            // User cmd
            let cmd_re = Regex::new(r"(?:\$)\s(cd|ls)\s?(.+)?").unwrap();
            let parts = cmd_re.captures(line);

            let cmd = &parts.as_ref().unwrap()[1];

            // Handle cd
            if cmd.eq("cd") {
                let target = &parts.as_ref().unwrap()[2];
                if target.eq("..") {
                    current_path.pop();
                } else {
                    current_path.push(target.to_owned());
                }
            }
        } else {
            let mut parts = line.split(" ");
            let left = parts.next().unwrap();
            let _right = parts.next().unwrap();

            if !left.eq("dir") {
                // Size of current file
                let size = left.parse::<usize>().unwrap();

                let mut cp = current_path.clone();
                let mut path_str = cp.join("/");
                while path_str.len() > 0 {
                    let current_size = dirs.get(&path_str);

                    if current_size != None {
                        dirs.insert(path_str, current_size.unwrap() + size);
                    } else {
                        dirs.insert(path_str, size);
                    }

                    cp.pop();
                    path_str = cp.join("/");
                }
            }
        }
    });

    dirs
}

fn part1(input: Split<&str>) -> usize {
    let dirs = get_dirs(input);
    dirs.into_values().filter(|size| size <= &100000).sum()
}

fn part2(input: Split<&str>) -> usize {
    let dirs = get_dirs(input);
    let current_unused = MAXIMUM_DISK_SPACE - dirs.get("/").unwrap();

    dirs.into_values()
        .filter(|size| (current_unused + size) >= REQUIRED_DISK_SPACE)
        .min()
        .unwrap()
}

fn main() {
    let raw_input = fs::read_to_string("./days/07/input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}

#[test]
fn test_part1() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part1(input), 95437);
}

#[test]
fn test_part2() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part2(input), 24933642);
}
