use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

const INFINITY: usize = 4294967295;

type Graph = HashMap<Coords, Node>;
type Row = usize;
type Col = usize;
type Coords = (Row, Col);

#[derive(Debug)]
struct HeightMapData {
    graph: Graph,
    src: Coords,
    dest: Coords,
}

#[derive(Debug)]
struct Node {
    coords: Coords,
    char: char,
    up: Option<Coords>,
    down: Option<Coords>,
    left: Option<Coords>,
    right: Option<Coords>,
}

fn get_char_value(char: char) -> char {
    if char == 'S' {
        'a'
    } else if char == 'E' {
        'z'
    } else {
        char
    }
}

/**
 * Dijkstra
 */
fn find_shortest_path(data: HeightMapData, start: Coords) -> usize {
    println!("finding shortest path from {:#?}", start);
    //   Declare a queue and insert the starting vertex.
    let mut queue: VecDeque<Coords> = VecDeque::new();

    // Initialize a visited array and mark the starting vertex as visited.
    let mut visited: HashSet<Coords> = HashSet::new();
    let mut distance: HashMap<Coords, usize> = HashMap::new();

    visited.insert(start);
    queue.push_back(start);
    distance.insert(start, 0);

    // Follow the below process till the queue becomes empty:
    while queue.len() > 0 {
        // Remove the first vertex of the queue.
        let node_coords = queue.pop_front().unwrap();
        let node = data.graph.get(&node_coords).unwrap();
        let char_val = get_char_value(node.char);

        // Mark that vertex as visited.
        visited.insert(node_coords);

        if node_coords == data.dest {
            break;
        }

        // Insert all the unvisited neighbours of the vertex into the queue.
        vec![node.up, node.down, node.left, node.right]
            .iter()
            .for_each(|sibling| {
                if !sibling.is_none() {
                    let sib_coords = sibling.unwrap();

                    if !visited.contains(&sib_coords) {
                        let sib_char_val =
                            get_char_value(data.graph.get(&sib_coords).unwrap().char);

                        if sib_char_val <= char_val || sib_char_val as u8 == char_val as u8 + 1 {
                            distance.insert(sib_coords, *distance.get(&node_coords).unwrap() + 1);
                            visited.insert(sib_coords);
                            queue.push_back(sib_coords);
                        }
                    }
                }
            })
    }

    let result = distance.get(&data.dest);

    if result.is_none() {
        INFINITY
    } else {
        *result.unwrap()
    }
}

fn graph_from_input(input: String) -> HeightMapData {
    let mut src = (INFINITY, INFINITY);
    let mut dest = (INFINITY, INFINITY);
    let mut graph: Graph = HashMap::new();

    let rows = input.trim().split("\n").collect::<Vec<&str>>();
    for (row, line) in rows.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                src = (row, col);
            }
            if c == 'E' {
                dest = (row, col);
            }

            let coords = (row, col);

            let up = if coords.0 > 0 {
                Some((coords.0 - 1, coords.1))
            } else {
                None
            };
            let down = if coords.0 < rows.len() - 1 {
                Some((coords.0 + 1, coords.1))
            } else {
                None
            };
            let left = if coords.1 > 0 {
                Some((coords.0, coords.1 - 1))
            } else {
                None
            };
            let right = if coords.1 < line.len() - 1 {
                Some((coords.0, coords.1 + 1))
            } else {
                None
            };

            graph.insert(
                coords,
                Node {
                    coords,
                    char: c,
                    up,
                    down,
                    left,
                    right,
                },
            );
        }
    }

    HeightMapData {
        graph: graph,
        src: src,
        dest: dest,
    }
}

fn part1(input: String) -> usize {
    let data = graph_from_input(input);
    let src = data.src.clone();
    find_shortest_path(data, src)
}

fn part2(input: String) -> usize {
    let data = graph_from_input(input.clone());

    let mut min_distance = INFINITY;

    let starting_points: Vec<&Node> = data
        .graph
        .values()
        .filter(|node| get_char_value(node.char) == 'a')
        .collect();

    for (index, node) in starting_points.iter().enumerate() {
        println!("{}/{}", index, starting_points.len());
        let data = graph_from_input(input.clone());
        let distance = find_shortest_path(data, node.coords);
        min_distance = if distance < min_distance {
            distance
        } else {
            min_distance
        };
    }

    min_distance
}

fn main() {
    let input = fs::read_to_string("./days/12/input.txt").expect("Error!");

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[test]
fn test_part1() {
    let input = fs::read_to_string("./test-input.txt").expect("Error!");
    assert_eq!(part1(input), 31);
}
#[test]
fn test_part2() {
    let input = fs::read_to_string("./test-input.txt").expect("Error!");
    assert_eq!(part2(input), 29);
}
