use crate::util::read_input;
use rand::Rng;
use std::hash::Hash;
use std::io::BufRead;

use std::thread::sleep;
use std::time::Duration;

use std::collections::HashMap;

const MAX: u64 = std::u64::MAX;
const TIMEOUT: u64 = 1000;

enum Direction {
    Right,
    Straight,
    Left,
}

type Map = Vec<Vec<char>>;

pub struct Race {
    map: Vec<Vec<char>>,
    start: (i64, i64),
    end: (i64, i64),
    count: usize,
}

#[derive(Debug)]
struct Reindeer {
    pos: (i64, i64),
    dir: (i64, i64),
    dist: u64,
    timeout: u64,
    history: Vec<(i64, i64)>,
}

pub fn puzzle1() {
    let race = parse_input();
    let res = puzzle1_runner();
    println!("day 16, puzzle 1: {}", res[0].dist);

    let mut visited = vec![race.start, race.end];
    for r in res {
        for v in r.history {
            if !visited.contains(&v) {
                visited.push(v)
            }
        }
    }

    println!("day 16, puzzle 2: {}", visited.len())
}

fn puzzle1_runner() -> Vec<Reindeer> {
    let race = parse_input();
    println!("{}", race.count);

    let mut frontline = vec![Reindeer {
        pos: race.start,
        dir: (0, 1),
        dist: 0,
        timeout: 0,
        history: vec![race.start],
    }];

    let mut frontline_len = frontline.len();
    let mut round_count = 0;
    let mut visited = vec![(race.start, (0, 1))];

    let mut finishers = vec![];

    while frontline_len > 0 {
        if round_count % 100 == 0 {
            // println!("round {round_count}: {} reindeers", frontline_len);
        }
        round_count += 1;

        let mut next_visited = vec![];

        let mut next_frontline = vec![];
        for reindeer in &frontline {
            if reindeer.pos == race.end {
                let re = Reindeer {
                    history: reindeer.history.clone(),
                    ..*reindeer
                };
                finishers.push(re);
            } else if reindeer.timeout > 0 {
                let history = reindeer.history.clone();
                next_frontline.push(Reindeer {
                    timeout: reindeer.timeout - 1,
                    history,
                    ..*reindeer
                })
            } else {
                if can_move_left(&race.map, &reindeer.pos, &reindeer.dir) {
                    let dir = (-reindeer.dir.1, reindeer.dir.0);
                    let pos = (reindeer.pos.0 + dir.0, reindeer.pos.1 + dir.1);
                    if !visited.contains(&(pos, dir)) {
                        next_visited.push((pos, dir));
                        let mut history = reindeer.history.clone();
                        history.push(reindeer.pos);
                        next_frontline.push(Reindeer {
                            pos,
                            dir,
                            dist: reindeer.dist + 1001,
                            timeout: TIMEOUT,
                            history,
                        })
                    }
                }

                if can_move_straight(&race.map, &reindeer.pos, &reindeer.dir) {
                    let dir = reindeer.dir;
                    let pos = (reindeer.pos.0 + dir.0, reindeer.pos.1 + dir.1);
                    if !visited.contains(&(pos, dir)) {
                        next_visited.push((pos, dir));
                        let mut history = reindeer.history.clone();
                        history.push(reindeer.pos);
                        next_frontline.push({
                            Reindeer {
                                pos,
                                dir,
                                dist: reindeer.dist + 1,
                                timeout: 0,
                                history,
                            }
                        })
                    }
                }

                if can_move_right(&race.map, &reindeer.pos, &reindeer.dir) {
                    let dir = (reindeer.dir.1, -reindeer.dir.0);
                    let pos = (reindeer.pos.0 + dir.0, reindeer.pos.1 + dir.1);
                    if !visited.contains(&(pos, dir)) {
                        next_visited.push((pos, dir));
                        let mut history = reindeer.history.clone();
                        history.push(reindeer.pos);
                        next_frontline.push({
                            Reindeer {
                                pos,
                                dir,
                                dist: reindeer.dist + 1001,
                                timeout: TIMEOUT,
                                history,
                            }
                        })
                    }
                }
            }
        }

        for v in next_visited {
            if !visited.contains(&v) {
                visited.push(v)
            }
        }

        frontline = next_frontline;
        frontline_len = frontline.len();

        if false {
            sleep(Duration::from_millis(100));
        }
        if finishers.len() > 0 {
            break;
        }
    }
    return finishers;
}

pub fn monte_carlo(race: &mut Race) -> (u64, (i64, i64)) {
    let mut pos = race.start;
    let mut dir: (i64, i64) = (0, 1);
    let mut dist = 0;

    for _ in 0..2000 {
        let mut possible_dirs = vec![];
        if can_move_left(&race.map, &pos, &dir) {
            possible_dirs.push(Direction::Left);
        }
        if can_move_straight(&race.map, &pos, &dir) {
            possible_dirs.push(Direction::Straight);
        }
        if can_move_right(&race.map, &pos, &dir) {
            possible_dirs.push(Direction::Right);
        }

        if possible_dirs.len() == 0 {
            if (pos.0 - race.start.0).abs() > 2 && (pos.0 - race.start.0).abs() > 2 {
                race.map[pos.0 as usize][pos.1 as usize] = '#';
                println!("tilkitty, {}", count_dots(&race.map));
            }
            return (MAX, pos);
        }

        let index = rand::thread_rng().gen_range(0..possible_dirs.len());
        let choice = &possible_dirs[index];

        match choice {
            Direction::Left => {
                dist += 1001;
                dir = (-dir.1, dir.0);
            }
            Direction::Straight => {
                dist += 1;
            }
            Direction::Right => {
                dist += 1001;
                dir = (dir.1, -dir.0);
            }
        }

        pos = (pos.0 + dir.0, pos.1 + dir.1);

        if pos == race.end {
            return (dist, pos);
        }
    }

    (MAX, pos)
}

fn count_dots(map: &Map) -> u64 {
    let mut count = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' {
                count += 1;
            }
        }
    }
    count
}

fn can_move_left(map: &Map, pos: &(i64, i64), dir: &(i64, i64)) -> bool {
    return map[(pos.0 - dir.1) as usize][(pos.1 + dir.0) as usize] == '.';
}

fn can_move_straight(map: &Map, pos: &(i64, i64), dir: &(i64, i64)) -> bool {
    return map[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] == '.';
}

fn can_move_right(map: &Map, pos: &(i64, i64), dir: &(i64, i64)) -> bool {
    return map[(pos.0 + dir.1) as usize][(pos.1 - dir.0) as usize] == '.';
}

pub fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{c}");
        }
        println!("");
    }
}

fn parse_input() -> Race {
    let reader = read_input("input/day_16.txt");

    let mut map: Vec<Vec<char>> = reader
        .lines()
        .map(|a| a.unwrap().chars().collect())
        .collect();

    let mut count = 0;

    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                start = (i as i64, j as i64);
                map[i][j] = '.';
            }

            if map[i][j] == 'E' {
                end = (i as i64, j as i64);
                map[i][j] = '.';
            }

            if map[i][j] == '.' {
                count += 1;
            }
        }
    }

    println!("start: {start:?}");
    println!("end: {end:?}");

    println!("count: {count}");

    Race {
        map,
        start,
        end,
        count,
    }
}

pub fn puzzle2() {
    let race = parse_input();

    let graph = create_graph(&race.map);

    let start= (race.start.0 as usize, race.start.1 as usize);
    let (mut distances_start, mut predecessors_start) =
        initialize_single_source(&graph, (start, Heading::East));

    dijkstra(&graph, &mut distances_start, &mut predecessors_start);

    let end = (race.end.0 as usize, race.end.1 as usize);

    let final_heading;
    let final_distance;
    let east = *distances_start.get(&(end, Heading::East)).unwrap();
    let north = *distances_start.get(&(end, Heading::North)).unwrap();
    
    println!("east: {east}");
    println!("north: {north}");

    if east < north {
        final_heading = Heading::East;
        final_distance = east;
    } else {
        final_heading = Heading::North;
        final_distance = north;
    }

    let (mut distances_end, mut predecessors_end) =
        initialize_single_source(&graph, (end, final_heading));

    dijkstra(&graph, &mut distances_end, &mut predecessors_end);
    
    let mut squares: Vec<(usize, usize)> = vec![];

    for node in graph.keys() {
        let dist_start = distances_start.get(node).unwrap();
        let dist_end = distances_end.get(node).unwrap();
        if dist_start + dist_end == final_distance && !squares.contains(&node.0) {
                squares.push(node.0);
        }
    }
    
    println!("day 16, puzzle 2: {}", squares.len());
    
    
}

const IMAX: i32 = std::i32::MAX;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Heading {
    East,
    North,
    West,
    South,
}

impl Heading {
    fn left(&self) -> Self {
        match self {
            Self::East => Self::North,
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::East => Self::South,
            Self::North => Self::East,
            Self::West => Self::North,
            Self::South => Self::West,
        }
    }
}

type Node = ((usize, usize), Heading);
type Edge = (Node, i32);
type Graph = HashMap<Node, Vec<Edge>>;
type Distances = HashMap<Node, i32>;
type Predecessors = HashMap<Node, Option<Node>>;

fn create_graph(map: &Vec<Vec<char>>) -> Graph {
    let headings = vec![Heading::East, Heading::North, Heading::West, Heading::South];
    let mut edge_lists: HashMap<((usize, usize), Heading), Vec<Edge>> = HashMap::new();

    for i in 1..map.len()-1 {
        for j in 1..map[0].len()-1 {
            if map[i][j] == '.' {
                for heading in &headings {
                    edge_lists.insert(((i, j), *heading), vec![]);
                    let edges = edge_lists.get_mut(&((i, j), *heading)).unwrap();
                    edges.push((((i, j), heading.left()), 1000));
                    edges.push((((i, j), heading.right()), 1000));
                }

                if map[i - 1][j] == '.' {
                    edge_lists
                        .get_mut(&((i - 1, j), Heading::North))
                        .unwrap()
                        .push((((i, j), Heading::North), 1));
                    edge_lists
                        .get_mut(&((i - 1, j), Heading::South))
                        .unwrap()
                        .push((((i, j), Heading::South), 1));
                    edge_lists
                        .get_mut(&((i, j), Heading::North))
                        .unwrap()
                        .push((((i - 1, j), Heading::North), 1));
                    edge_lists
                        .get_mut(&((i, j), Heading::South))
                        .unwrap()
                        .push((((i - 1, j), Heading::South), 1));
                }
                if map[i][j - 1] == '.' {
                    edge_lists
                        .get_mut(&((i, j - 1), Heading::East))
                        .unwrap()
                        .push((((i, j), Heading::East), 1));
                    edge_lists
                        .get_mut(&((i, j - 1), Heading::West))
                        .unwrap()
                        .push((((i, j), Heading::West), 1));
                    edge_lists
                        .get_mut(&((i, j), Heading::East))
                        .unwrap()
                        .push((((i, j - 1), Heading::East), 1));
                    edge_lists
                        .get_mut(&((i, j), Heading::West))
                        .unwrap()
                        .push((((i, j - 1), Heading::West), 1));
                }



                // let mut edges_to_add: HashMap<Heading, Vec<Edge>> = HashMap::new();
                
                // if map[i][j+1] == '.' {
                //     edges_to_add.insert(Heading::East, vec![(((i, j+1), Heading::East), 1)]);
                // }
                // if map[i-1][j] == '.' {
                //     edges_to_add.insert(Heading::North, vec![(((i-1, j), Heading::North), 1)]);
                // }
                // if map[i][j-1] == '.' {
                //     edges_to_add.insert(Heading::West, vec![(((i, j-1), Heading::West), 1)]);

                // }
                // if map[i+1][j] == '.' {
                //     edges_to_add.insert(Heading::South, vec![(((i+1, j), Heading::South), 1)]);
                // }

                // if map[i][j-1] == '.' || map[i][j+1] == '.' {

                // }

                // let edges2 = edges_to_add.clone();
                // let headings = edges2.keys().collect::<Vec<_>>();
                // let headings2 = headings.clone();
                // for heading in headings {
                //     if headings2.contains(&&heading.left()) {
                //         edges_to_add.get_mut(heading).unwrap().push((((i, j), heading.left()), 1000));
                //     }
                //     if headings2.contains(&&heading.right()) {
                //         edges_to_add.get_mut(heading).unwrap().push((((i, j), heading.right()), 1000));
                //     }
                // }

                // let headings = edges2.keys().collect::<Vec<_>>();
                // for k in 0..headings.len() {
                //     edge_lists.insert(((i, j), headings[k].clone()), edges_to_add.get(&headings[k]).unwrap().clone());
                // }
            }
        }
    }

    edge_lists
}

fn initialize_single_source(
    graph: &Graph,
    source: Node,
) -> (HashMap<Node, i32>, HashMap<Node, Option<Node>>) {
    let keys = graph.keys().clone();
    let mut distance_estimates = HashMap::new();
    let mut predecessors = HashMap::new();
    for k in keys {
        distance_estimates.insert(*k, IMAX);
        predecessors.insert(*k, None);
    }

    let s = distance_estimates.get_mut(&source).unwrap();
    *s = 0;

    (distance_estimates, predecessors)
}

fn dijkstra(graph: &Graph, distance_estimates: &mut Distances, predecessors: &mut Predecessors) {
    let mut queue = graph.keys().clone().collect::<Vec<_>>();
    while queue.len() > 0 {
        let min = extract_min(&mut queue, &distance_estimates);
        println!("relaxing: {min:?}, left: {}", queue.len());
        
        let min_d = distance_estimates.get(&min).unwrap().clone();
        if min_d == IMAX {
            break;
        }
        let to_relax = graph.get(&min).unwrap();
        for v in to_relax {
            let d = distance_estimates.get_mut(&v.0).unwrap();
            if *d > min_d + v.1 {
                *d = min_d + v.1;
                *predecessors.get_mut(&v.0).unwrap() = Some(min);
            }
        }
    }
}

fn extract_min(
    queue: &mut Vec<&Node>,
    distance_estimates: &HashMap<Node, i32>,
) -> Node {
    let mut min_index = 0;
    let mut min = IMAX;

    for i in 0..queue.len() {
        let candidate = *distance_estimates.get(queue[i]).unwrap();
        if candidate < min {
            min = candidate;
            min_index = i;
        }
    }

    *queue.remove(min_index)
}