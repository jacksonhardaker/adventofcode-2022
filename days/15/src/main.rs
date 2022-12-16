use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
    str::Split,
};

const TUNING_MOD: i32 = 4000000;

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

fn find_impossibles(scan: Scan, row: i32) -> HashSet<Coords> {
    let mut occupied: HashSet<Coords> = HashSet::new();

    let mut beacons: HashSet<Coords> = HashSet::new();
    scan.keys().for_each(|key| {
        beacons.insert(*scan.get(key).unwrap());
    });

    scan.keys().for_each(|sensor| {
        let beacon = scan.get(sensor).unwrap();
        let distance: i32 = get_manhattan_distance(sensor, beacon);
        let diff: i32 = i32::abs_diff(sensor.1, row) as i32;

        let mut remaining_distance = distance - diff;

        if remaining_distance >= 0 {
            if !beacons.contains(&(sensor.0, row)) {
                occupied.insert((sensor.0, row));
            }

            let mut x_delta = 1;
            while remaining_distance > 0 {
                if !beacons.contains(&(sensor.0 + x_delta, row)) {
                    occupied.insert((sensor.0 + x_delta, row));
                }
                if !beacons.contains(&(sensor.0 - x_delta, row)) {
                    occupied.insert((sensor.0 - x_delta, row));
                }
                x_delta += 1;
                remaining_distance -= 1;
            }
        }
    });
    occupied
}

fn part1(input: Split<&str>, row: i32) -> usize {
    let scan = generate_scan(input);
    find_impossibles(scan, row).len()
}

fn part2(input: Split<&str>, min_max: (i32, i32)) -> usize {
    let scan = generate_scan(input);
    let mut beacons: HashSet<Coords> = HashSet::new();
    scan.keys().for_each(|key| {
        beacons.insert(*scan.get(key).unwrap());
    });

    for y in min_max.0..=min_max.1 {
        let occupied = find_impossibles(scan.to_owned(), y);
        for x in min_max.0..=min_max.1 {
            if !(occupied.contains(&(x, y)) || beacons.contains(&(x, y))) {
                return (x * TUNING_MOD + y).try_into().unwrap();
            }
        }
    }
    0
}

fn main() {
    let input = fs::read_to_string("./days/15/input.txt").expect("Error!");

    println!("Part 1: {}", part1(input.trim().split("\n"), 2000000));
    println!("Part 2: {}", part2(input.trim().split("\n"), (0, 4000000)));
}

#[test]
fn test_part1() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part1(input, 10), 26);
}
#[test]
#[ignore = "reason"]
fn test_part2() {
    let raw_input = fs::read_to_string("./test-input.txt").expect("Error!");
    let input = raw_input.trim().split("\n");
    assert_eq!(part2(input, (0, 20)), 56000011);
}
