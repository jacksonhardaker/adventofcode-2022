use std::{
    collections::{HashMap, HashSet},
    fs,
};

const INFINITY: usize = 4294967295;

type Graph = Vec<Vec<char>>;
type Row = usize;
type Col = usize;
type Coords = (Row, Col);

#[derive(Debug)]
struct HeightMapData {
    graph: Graph,
    src: Coords,
    dest: Coords,
    initial_unvisited: HashSet<Coords>,
    initial_tentative_distances: HashMap<Coords, usize>,
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
fn find_paths(data: HeightMapData) -> HashMap<Coords, usize> {
    let mut unvisited = data.initial_unvisited.clone();
    let mut tentative_dist = data.initial_tentative_distances.clone();

    let mut node = data.src;
    while unvisited.len() > 0 {
        println!("{}", unvisited.len());
        let val = get_char_value(data.graph[node.0][node.1]);

        if node == data.dest {
            break;
        }

        // Calculate tentative dist for up,down,left,right
        let up = if node.0 > 0 {
            Some((node.0 - 1, node.1))
        } else {
            None
        };
        let down = if node.0 < data.graph.len() - 1 {
            Some((node.0 + 1, node.1))
        } else {
            None
        };
        let left = if node.1 > 0 {
            Some((node.0, node.1 - 1))
        } else {
            None
        };
        let right = if node.1 < data.graph[0].len() - 1 {
            Some((node.0, node.1 + 1))
        } else {
            None
        };

        vec![up, down, left, right].iter().for_each(|adj_node| {
            if !adj_node.is_none() {
                let adj_coords = adj_node.unwrap();
                if unvisited.contains(&adj_coords) {
                    let adj_val = get_char_value(data.graph[adj_coords.0][adj_coords.1]);

                    if adj_val <= val || adj_val as u8 == val as u8 + 1 {
                        // if adj_val == val || adj_val as u8 == val as u8 - 1 || adj_val as u8 == val as u8 + 1 {
                        // We can visit from here.
                        let adj_tent = tentative_dist.get(&adj_coords).unwrap();
                        let cur_tent = tentative_dist.get(&node).unwrap();
                        let new_tent = if adj_tent < &(cur_tent + 1) {
                            adj_tent.to_owned()
                        } else {
                            (cur_tent + 1).clone()
                        };

                        tentative_dist.insert(adj_coords, new_tent);
                    }
                }
            }
        });

        // Of the unvisited nodes, visit the one with the lowest tentative distance, and remove the current node from unvisited
        unvisited.remove(&node);

        let mut lowest_tentative = INFINITY;
        unvisited.iter().for_each(|unvisited_node| {
            let tent = tentative_dist.get(unvisited_node).unwrap();

            if tent < &lowest_tentative {
                lowest_tentative = tent.to_owned();
                node = *unvisited_node;
            }
        });
    }

    tentative_dist
}

fn graph_from_input(input: String) -> HeightMapData {
    let mut unvisited: HashSet<Coords> = HashSet::new();
    let mut tenative_distances: HashMap<Coords, usize> = HashMap::new();
    let mut src = (INFINITY, INFINITY);
    let mut dest = (INFINITY, INFINITY);
    let graph = input
        .trim()
        .split("\n")
        .enumerate()
        .map(|(row_index, row)| {
            row.chars()
                .enumerate()
                .map(|(col_index, c)| {
                    if c == 'S' {
                        src = (row_index, col_index);
                    }
                    if c == 'E' {
                        dest = (row_index, col_index);
                    }

                    unvisited.insert((row_index, col_index));
                    tenative_distances
                        .insert((row_index, col_index), if c == 'S' { 0 } else { INFINITY });

                    c
                })
                .collect()
        })
        .collect::<Vec<Vec<char>>>();

    HeightMapData {
        graph: graph,
        src: src,
        dest: dest,
        initial_unvisited: unvisited,
        initial_tentative_distances: tenative_distances,
    }
}

fn part1(input: String) -> usize {
    let data = graph_from_input(input);
    let dest = data.dest.clone();
    let tentative = find_paths(data);

    *tentative.get(&dest).unwrap()
}

fn part2(input: String) -> usize {
    0
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
    assert_eq!(part2(input), 0);
}
