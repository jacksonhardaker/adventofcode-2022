use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
    str::Split,
};

type Coords = (i32, i32);
type Scan = HashMap<Coords, Coords>;

fn get_manhattan_distance(a: &Coords, b: &Coords) -> i32 {
    i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1)
}

fn generate_scan(input: Split<&str>) -> Scan {
    let mut scan: HashMap<Coords, Coords> = HashMap::new();
    input.for_each(|line| {
        let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
        let mut matches = re.captures_iter(line);

        let sensor = matches.next().unwrap();
        let beacon = matches.next().unwrap();
        let sensor_coord: Coords = (
            sensor[1].parse::<i32>().unwrap(),
            sensor[2].parse::<i32>().unwrap(),
        );
        let beacon_coord: Coords = (
            beacon[1].parse::<i32>().unwrap(),
            beacon[2].parse::<i32>().unwrap(),
        );

        scan.insert(sensor_coord, beacon_coord);
    });

    scan
}

fn find_impossibles(scan: Scan, row: i32) -> usize {
    let mut distances: HashMap<Coords, i32> = HashMap::new();
    let mut beacons: HashSet<Coords> = HashSet::new();
    let mut min_x: i32 = 0;
    let mut max_x: i32 = 0;
    scan.keys().for_each(|sensor| {
        let beacon = scan.get(sensor).unwrap();
        let distance = get_manhattan_distance(sensor, beacon);
        distances.insert(*sensor, distance);
        beacons.insert(*beacon);

        if sensor.0 - distance < min_x {
            min_x = sensor.0 - distance;
        }
        if sensor.0 + distance > max_x {
            max_x = sensor.0 + distance;
        }
    });

    let mut impossibles: HashSet<Coords> = HashSet::new();

    for x in min_x..=max_x {
        let current = (x, row);
        scan.keys().for_each(|sensor| {
            let distance = get_manhattan_distance(sensor, &current);

            if distance <= *distances.get(sensor).unwrap() && !beacons.contains(&current) {
                // Is
                impossibles.insert(current);
            }
        });
    }

    // println!("{:#?}", impossibles);
    impossibles.len()
}

fn part1(input: Split<&str>, row: i32) -> usize {
    let scan = generate_scan(input);
    find_impossibles(scan, row)
}

fn part2(input: Split<&str>) -> usize {
    0
}

fn main() {
    let input = fs::read_to_string("./days/15/input.txt").expect("Error!");

    println!("Part 1: {}", part1(input.trim().split("\n"), 2000000));
    println!("Part 2: {}", part2(input.trim().split("\n")));
}

#[test]
fn test_part1() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part1(input, 10), 26);
}
#[test]
fn test_part2() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part2(input), 0);
}
