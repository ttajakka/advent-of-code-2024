use crate::util::read_input;
use std::{collections::HashMap, io::BufRead};
// use std::thread::sleep;
// use std::time::Duration;

const ENV: &str = "REAL";
// const ENV: &str = "MOCK";
const MAX: i32 = std::i32::MAX;

type Graph = HashMap<(usize, usize), Vec<(usize, usize)>>;
type Distances = HashMap<(usize, usize), i32>;
type Predecessors = HashMap<(usize, usize), Option<(usize, usize)>>;

pub fn puzzle1() {
    let bytes = parse_input();

    let size;
    let byte_count: usize;
    if ENV == "REAL" {
        size = 71;
        byte_count = 1024;
    } else {
        byte_count = 12;
        size = 7;
    }
    let mut map = vec![vec!['.'; size]; size];
    
    for i in 0..byte_count {
        let (x, y) = bytes[i];
        map[x][y] = '#'; // NOTE: indices should be flipped!
    }

    let graph = create_graph(&map);
    let (mut distance_estimates, mut predecessors) = initialize_single_source(&graph, (0, 0));
    dijkstra(&graph, &mut distance_estimates, &mut predecessors);
    
    let destination= (size-1, size-1);
    println!("day 18, puzzle 1: {}", distance_estimates.get(&destination).unwrap());
    
}

fn dijkstra(graph: &Graph, distance_estimates: &mut Distances, predecessors: &mut Predecessors) {
    let mut queue = graph.keys().clone().collect::<Vec<_>>();
    while queue.len() > 0 {
        let min = extract_min(&mut queue, &distance_estimates);
        let min_d = distance_estimates.get(&min).unwrap().clone();
        let to_relax = graph.get(&min).unwrap();
        for v in to_relax {
            let d = distance_estimates.get_mut(v).unwrap();
            if *d > min_d + 1 {
                *d = min_d + 1;
                *predecessors.get_mut(&v).unwrap() = Some(min);
            }
        }
    }
}

fn extract_min(
    queue: &mut Vec<&(usize, usize)>,
    distance_estimates: &HashMap<(usize, usize), i32>,
) -> (usize, usize) {
    let mut min_index = 0;
    let mut min = MAX;

    for i in 0..queue.len() {
        let candidate = *distance_estimates.get(queue[i]).unwrap();
        if candidate < min {
            min = candidate;
            min_index = i;
        }
    }

    *queue.remove(min_index)
}

fn initialize_single_source(
    graph: &Graph,
    source: (usize, usize),
) -> (
    HashMap<(usize, usize), i32>,
    HashMap<(usize, usize), Option<(usize, usize)>>,
) {
    let keys = graph.keys().clone();
    let mut distance_estimates = HashMap::new();
    let mut predecessors = HashMap::new();
    for k in keys {
        distance_estimates.insert(*k, MAX);
        predecessors.insert(*k, None);
    }

    let s = distance_estimates.get_mut(&source).unwrap();
    *s = 0;

    (distance_estimates, predecessors)
}

fn create_graph(map: &Vec<Vec<char>>) -> Graph {
    let mut edge_lists = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' {
                edge_lists.insert((i, j), vec![]);
                if i > 0 && map[i - 1][j] == '.' {
                    edge_lists.get_mut(&(i - 1, j)).unwrap().push((i, j));
                    edge_lists.get_mut(&(i, j)).unwrap().push((i - 1, j));
                }
                if j > 0 && map[i][j - 1] == '.' {
                    edge_lists.get_mut(&(i, j - 1)).unwrap().push((i, j));
                    edge_lists.get_mut(&(i, j)).unwrap().push((i, j - 1));
                }
            }
        }
    }

    edge_lists
}

pub fn print_map(map: &Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{}", map[i][j]);
        }
        println!();
    }
}

pub fn parse_input() -> Vec<(usize, usize)> {
    let path;
    if ENV == "REAL" {
        path = "input/day_18.txt";
    } else {
        path = "input/day_18_mock.txt"
    }
    let reader = read_input(path);
    reader
        .lines()
        .map(|l| {
            let parts = l.unwrap();
            let mut parts = parts.split(",");
            let a = parts.next().unwrap().parse().unwrap();
            let b = parts.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect()
}
