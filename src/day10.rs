use std::io::BufRead;

use crate::util::read_input;

type Map = Vec<Vec<u8>>;
type Position = (usize, usize);

pub fn puzzle1() {
    let map = parse_input();

    let trailheads = find_trailheads(&map);

    let mut total_score = 0;
    for head in trailheads {
        total_score += find_score(&map, head);
    }

    println!("day 10, puzzle 1: {total_score}");
}

fn find_score(map: &Map, head: Position) -> usize {
    if map[head.0][head.1] != 0 {
        panic!();
    }
    let peaks = find_peaks_recursive(&map, head);
    return peaks.len();
}

fn find_peaks_recursive(map: &Map, pos: Position) -> Vec<Position> {
    println!(
        "finding peaks, pos: {} {}, height: {}",
        pos.0, pos.1, map[pos.0][pos.1]
    );
    if map[pos.0][pos.1] == 9 {
        return vec![pos];
    } else {
        let mut peaks = vec![];
        if pos.0 > 0 && map[pos.0 - 1][pos.1] == map[pos.0][pos.1] + 1 {
            peaks = find_peaks_recursive(map, (pos.0 - 1, pos.1));
        }
        if pos.0 < map.len() - 1 && map[pos.0 + 1][pos.1] == map[pos.0][pos.1] + 1 {
            for p in find_peaks_recursive(map, (pos.0 + 1, pos.1)) {
                if !peaks.contains(&p) {
                    peaks.push(p);
                }
            }
        }
        if pos.1 > 0 && map[pos.0][pos.1 - 1] == map[pos.0][pos.1] + 1 {
            for p in find_peaks_recursive(map, (pos.0, pos.1 - 1)) {
                if !peaks.contains(&p) {
                    peaks.push(p);
                }
            }
        }
        if pos.1 < map[0].len() - 1 && map[pos.0][pos.1 + 1] == map[pos.0][pos.1] + 1 {
            for p in find_peaks_recursive(map, (pos.0, pos.1 + 1)) {
                if !peaks.contains(&p) {
                    peaks.push(p);
                }
            }
        }
        peaks
    }
}

fn find_trailheads(map: &Map) -> Vec<Position> {
    let mut positions = vec![];

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                positions.push((i, j));
            }
        }
    }
    positions
}

fn parse_input() -> Map {
    let reader = read_input("input/day_10.txt".to_string());
    let mut map = vec![];
    for line in reader.lines() {
        map.push(
            line.unwrap()
                .chars()
                .map(|a| a.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>(),
        );
    }
    map
}
