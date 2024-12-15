use crate::util::read_input;
use std::io::BufRead;
use std::time;

const LUTSIZE: u8 = 38;
const LUTENTRIES: u64 = 10;

pub fn puzzle1() {
    let mut stones = parse_input();
    // println!("{stones:?}");
    for _ in 0..25 {
        let mut i: usize = 0;
        while i < stones.len() {
            if stones[i] == 0 {
                stones[i] = 1;
            } else {
                let a = stones[i].to_string();
                if a.len() % 2 == 0 {
                    let first = a[..a.len() / 2].parse::<u64>().unwrap();
                    let second = a[a.len() / 2..].parse::<u64>().unwrap();
                    stones[i] = first;
                    stones.insert(i + 1, second);
                    i += 1;
                } else {
                    stones[i] *= 2024;
                }
            }
            i += 1;
        }
        // println!("{stones:?}");
    }

    println!("day 11, puzzle 1: {}", stones.len());
}

fn f(label: u64, n: u8) -> u64 {
    if n == 0 {
        return 1;
    } else if n >= 4 && label == 0 {
        return f(4, n - 4) + 2 * f(2, n - 4) + f(0, n - 4);
    } else if n >= 3 && label == 1 {
        return f(4, n - 3) + 2 * f(2, n - 3) + f(0, n - 3);
    } else if n >= 3 && label == 2 {
        return f(8, n - 3) + 2 * f(4, n - 3) + f(0, n - 3);
    } else if n >= 3 && label == 3 {
        return f(7, n - 3) + f(6, n - 3) + f(2, n - 3) + f(0, n - 3);
    } else if n >= 3 && label == 4 {
        return f(9, n - 3) + f(8, n - 3) + f(6, n - 3) + f(0, n - 3);
    } else {
        if label == 0 {
            return f(1, n - 1);
        }
        let a = label.to_string();
        if a.len() % 2 == 0 {
            let first = a[..a.len() / 2].parse::<u64>().unwrap();
            let second = a[a.len() / 2..].parse::<u64>().unwrap();
            return f(first, n - 1) + f(second, n - 1);
        } else {
            return f(2024 * label, n - 1);
        }
    }
}

fn g(label: u64, n: u8, lut: &Vec<Vec<u64>>) -> u64 {
    if label < LUTENTRIES && n < LUTSIZE {
        return lut[label as usize][n as usize];
    } if n == 0 {
        return 1;
    } else if n >= 4 && label == 0 {
        return g(4, n - 4, lut) + 2 * g(2, n - 4, lut) + g(0, n - 4, lut);
    } else if n >= 3 && label == 1 {
        return g(4, n - 3, lut) + 2 * g(2, n - 3, lut) + g(0, n - 3, lut);
    } else if n >= 3 && label == 2 {
        return g(8, n - 3, lut) + 2 * g(4, n - 3, lut) + g(0, n - 3, lut);
    } else if n >= 3 && label == 3 {
        return g(7, n - 3, lut) + g(6, n - 3, lut) + g(2, n - 3, lut) + g(0, n - 3, lut);
    } else if n >= 3 && label == 4 {
        return g(9, n - 3, lut) + g(8, n - 3, lut) + g(6, n - 3, lut) + g(0, n - 3, lut);
    } else {
        if label == 0 {
            return g(1, n - 1, lut);
        }
        let a = label.to_string();
        if a.len() % 2 == 0 {
            let first = a[..a.len() / 2].parse::<u64>().unwrap();
            let second = a[a.len() / 2..].parse::<u64>().unwrap();
            return g(first, n - 1, lut) + g(second, n - 1, lut);
        } else {
            return g(2024 * label, n - 1, lut);
        }
    }
}

fn prepare_lut() -> Vec<Vec<u64>> {
    let mut lut = vec![vec![]; LUTENTRIES as usize];
    for i in 0..LUTENTRIES {
        for j in 0..LUTSIZE {
            lut[i as usize].push(f(i as u64, j))
        }
    }
    lut
}

pub fn puzzle2() {
    let before = time::SystemTime::now();
    let lut = prepare_lut();

    let stones = parse_input();

    let result: u64 = stones.iter().map(|label| g(*label, 75, &lut)).sum();
    let after = time::SystemTime::now();
    let diff = after.duration_since(before).unwrap();
    println!("day 11, puzzle 2: {} (took {})", result, diff.as_secs());
}

fn parse_input() -> Vec<u64> {
    let reader = read_input("input/day_11.txt");
    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect()
}
