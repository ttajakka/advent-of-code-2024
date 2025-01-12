use core::panic;
use std::{collections::HashMap, io::BufRead};

use crate::util::read_input;

pub fn puzzle1() {
    let input = parse_input();

    let level_1_distances = generate_arrow_distances_level_1();
    let level_2_distances = generate_next_level_arrow_distances(level_1_distances);

    let mut res = 0;

    for code in input {
        let mut chars = vec!['A'];
        let mut to_append = code.1.chars().collect::<Vec<char>>();
        chars.append(&mut to_append);

        let mut length = 0;
        for i in 0..chars.len() - 1 {
            length += calculate_numpad_distance(&chars[i], &chars[i + 1], &level_2_distances)
        }

        res += (code.0 as u64) * length;
    }

    println!("day 21, puzzle 1: {res}");
}

pub fn puzzle2() {
    let input = parse_input();

    // let level_1_distances = generate_arrow_distances_level_1();
    // let level_2_distances = generate_next_level_arrow_distances(level_1_distances);
    let mut arrow_distances = generate_arrow_distances_level_1();

    for _ in 0..24 {
        arrow_distances = generate_next_level_arrow_distances(arrow_distances);
    }

    let mut res = 0;

    for code in input {
        let mut chars = vec!['A'];
        let mut to_append = code.1.chars().collect::<Vec<char>>();
        chars.append(&mut to_append);

        let mut length = 0;
        for i in 0..chars.len() - 1 {
            length += calculate_numpad_distance(&chars[i], &chars[i + 1], &arrow_distances)
        }

        res += (code.0 as u64) * length;
    }

    println!("day 21, puzzle 1: {res}");
}

fn generate_arrow_distances_level_1() -> HashMap<(char, char), u64> {
    let mut distances = HashMap::new();

    let keys = vec!['<', 'v', '>', '^', 'A'];
    for from in &keys {
        for to in &keys {
            let from_pos = arrow_to_pos(from);
            let to_pos = arrow_to_pos(to);
            distances.insert(
                (*from, *to),
                (from_pos.0 - to_pos.0).abs() as u64 + (from_pos.1 - to_pos.1).abs() as u64 + 1,
            );
        }
    }

    distances
}

fn generate_next_level_arrow_distances(
    previous_level_distances: HashMap<(char, char), u64>,
) -> HashMap<(char, char), u64> {
    let mut distances = HashMap::new();

    let keys = vec!['<', 'v', '>', '^', 'A'];
    for from in &keys {
        for to in &keys {
            let paths = generate_arrow_paths(from, to);
            let mut min = std::u64::MAX;
            for path in paths {
                let candidate = path_length(path, &previous_level_distances);
                if candidate < min {
                    min = candidate;
                }
            }
            distances.insert((*from, *to), min);
        }
    }

    distances
}

fn generate_arrow_paths(from: &char, to: &char) -> Vec<Vec<char>> {
    let mut paths = vec![];

    match (*from, *to) {
        ('<', 'A') => {
            paths.push(vec!['>', '>', '^']);
            paths.push(vec!['>', '^', '>']);
        }
        ('v', 'A') => {
            paths.push(vec!['>', '^']);
            paths.push(vec!['^', '>']);
        }
        ('A', '<') => {
            paths.push(vec!['v', '<', '<']);
            paths.push(vec!['<', 'v', '<']);
        }
        ('A', 'v') => {
            paths.push(vec!['v', '<']);
            paths.push(vec!['<', 'v']);
        }
        ('^', '>') => {
            paths.push(vec!['v', '>']);
            paths.push(vec!['>', 'v']);
        }
        ('>', '^') => {
            paths.push(vec!['^', '<']);
            paths.push(vec!['<', '^']);
        }
        ('^', '<') => {
            paths.push(vec!['v', '<']);
        }
        ('<', '^') => {
            paths.push(vec!['>', '^']);
        }
        _ => {
            let mut path = vec![];
            let from = arrow_to_pos(from);
            let to = arrow_to_pos(to);

            let diff_x = to.1 - from.1;
            let sym_x;
            if diff_x < 0 {
                sym_x = '<';
            } else {
                sym_x = '>';
            }
            path.append(&mut vec![sym_x; diff_x.abs() as usize]);

            let diff_y = to.0 - from.0;
            let sym_y;
            if diff_y < 0 {
                sym_y = 'v';
            } else {
                sym_y = '^';
            }
            path.append(&mut vec![sym_y; diff_y.abs() as usize]);

            paths.push(path);
        }
    }

    for i in 0..paths.len() {
        paths[i].push('A')
    }

    paths
}

fn path_length(mut path: Vec<char>, distances: &HashMap<(char, char), u64>) -> u64 {
    let mut tmp_path = vec!['A'];
    tmp_path.append(&mut path);
    let path = tmp_path;

    let mut length = 0;
    for i in 0..path.len() - 1 {
        length += distances.get(&(path[i], path[i + 1])).unwrap();
    }

    length
}

fn generate_num_paths(from: &char, to: &char) -> Vec<Vec<char>> {
    let mut paths = vec![];

    match (*from, *to) {
        ('A', '1') => {
            paths.push(vec!['<', '^', '<']);
            paths.push(vec!['^', '<', '<']);
        }
        ('A', '2') => {
            paths.push(vec!['<', '^']);
            paths.push(vec!['^', '<']);
        }
        ('A', '4') => {
            paths.push(vec!['^', '^', '<', '<']);
            paths.push(vec!['^', '<', '^', '<']);
            paths.push(vec!['^', '<', '<', '^']);
            paths.push(vec!['<', '^', '^', '<']);
            paths.push(vec!['<', '^', '<', '^']);
        }
        ('A', '5') => {
            paths.push(vec!['^', '^', '<']);
            paths.push(vec!['^', '<', '^']);
            paths.push(vec!['<', '^', '^']);
        }
        ('1', 'A') => {
            paths.push(vec!['>', '>', 'v']);
            paths.push(vec!['>', 'v', '>']);
        }
        ('2', '9') => {
            paths.push(vec!['^', '^', '>']);
            paths.push(vec!['^', '>', '^']);
            paths.push(vec!['^', '>', '^']);
        }
        ('3', '4') => {
            paths.push(vec!['^', '<', '<']);
            paths.push(vec!['<', '^', '<']);
            paths.push(vec!['<', '<', '^']);
        }
        ('3', '7') => {
            paths.push(vec!['^', '^', '<', '<']);
            paths.push(vec!['^', '<', '^', '<']);
            paths.push(vec!['^', '<', '<', '^']);
            paths.push(vec!['<', '^', '^', '<']);
            paths.push(vec!['<', '^', '<', '^']);
            paths.push(vec!['<', '<', '^', '^']);
        }
        ('5', '9') => {
            paths.push(vec!['>', '^']);
            paths.push(vec!['^', '>']);
        }
        ('8', 'A') => {
            paths.push(vec!['v', 'v', 'v', '>']);
            paths.push(vec!['v', 'v', '>', 'v']);
            paths.push(vec!['v', '>', 'v', 'v']);
            paths.push(vec!['>', 'v', 'v', 'v']);
        }
        ('8', '6') => {
            paths.push(vec!['v', '>']);
            paths.push(vec!['>', 'v']);
        }

        _ => {
            let mut path = vec![];
            let from = num_to_pos(from);
            let to = num_to_pos(to);

            let diff_x = to.1 - from.1;
            let sym_x;
            if diff_x < 0 {
                sym_x = '<';
            } else {
                sym_x = '>';
            }
            path.append(&mut vec![sym_x; diff_x.abs() as usize]);

            let diff_y = to.0 - from.0;
            let sym_y;
            if diff_y < 0 {
                sym_y = 'v';
            } else {
                sym_y = '^';
            }
            path.append(&mut vec![sym_y; diff_y.abs() as usize]);

            paths.push(path);
        }
    }

    for i in 0..paths.len() {
        paths[i].push('A')
    }

    paths
}

fn calculate_numpad_distance(
    from: &char,
    to: &char,
    level_2_distances: &HashMap<(char, char), u64>,
) -> u64 {
    let paths = generate_num_paths(from, to);
    let mut min = std::u64::MAX;
    for path in paths {
        let candidate = path_length(path, &level_2_distances);
        if candidate < min {
            min = candidate;
        }
    }
    min
}

fn num_to_pos(c: &char) -> (i32, i32) {
    match c {
        'A' => (0, 3),
        '0' => (0, 2),
        '1' => (1, 1),
        '2' => (1, 2),
        '3' => (1, 3),
        '4' => (2, 1),
        '5' => (2, 2),
        '6' => (2, 3),
        '7' => (3, 1),
        '8' => (3, 2),
        '9' => (3, 3),
        _ => panic!(),
    }
}

fn arrow_to_pos(c: &char) -> (i32, i32) {
    match c {
        '<' => (0, 1),
        'v' => (0, 2),
        '>' => (0, 3),
        '^' => (1, 2),
        'A' => (1, 3),
        _ => panic!("arrow_to_pos, got {c}"),
    }
}

pub fn parse_input() -> Vec<(usize, String)> {
    let reader = read_input("input/day_21.txt");
    reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let a = l.split("A").next().unwrap().parse::<usize>().unwrap();
            (a, l)
        })
        .collect()
}
