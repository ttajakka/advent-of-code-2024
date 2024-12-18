use std::io::BufRead;

use crate::util::read_input;

const ENV: &str = "REAL";

pub fn puzzle1() {
    let bytes = parse_input();

    let size;
    if ENV == "REAL" {
        size = 70;
    } else {
        size = 6;
    }

    let mut map = vec![vec!['.'; size]; size];
    
    let byte_count: usize;
    if ENV == "REAL" {
        byte_count = 1024;
    } else {
        byte_count = 12;
    }

    for i in 0..byte_count {
        let (x, y) = bytes[i];
        map[x][y] = '#'; // NOTE: indices should be flipped!
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
    reader.lines().map(|l| {
        let parts = l.unwrap();
        let mut parts = parts.split(",");
        let a = parts.next().unwrap().parse().unwrap();
        let b = parts.next().unwrap().parse().unwrap();
        (a, b)
    }).collect()
}
