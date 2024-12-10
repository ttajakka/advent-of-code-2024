use std::io::BufRead;

use crate::util::read_input;

pub fn puzzle1() {
    let input = parse_input();

    let mut expanded = expand(&input);

    fragment(&mut expanded);

    let checksum = calculate_checksum(&expanded);

    println!("day 9, puzzle 1: {checksum}");
}

fn calculate_checksum(files: &Vec<i32>) -> i64 {
    let mut result = 0;
    for i in 0..files.len() {
        if files[i] == -1 {
            break;
        }
        result += i as i64 * files[i] as i64;
    }
    result
}

fn fragment(files: &mut Vec<i32>) {
    let mut tail = 0;
    let mut head = files.len() - 1;
    while tail < head {
        if files[tail] != -1 {
            tail += 1;
            continue;
        } else if files[head] == -1 {
            head -= 1;
            continue;
        } else {
            files[tail] = files[head];
            files[head] = -1;
        }
    }
}

fn expand(input: &Vec<u8>) -> Vec<i32> {
    let mut out = vec![];
    for _ in 0..input[0] {
        out.push(0);
    }
    let mut id = 1;
    for i in 1..(input.len() + 1) / 2 {
        for _ in 0..input[2 * i - 1] {
            out.push(-1);
        }
        for _ in 0..input[2 * i] {
            out.push(id);
        }
        id += 1;
    }

    out
}

fn parse_input() -> Vec<u8> {
    let path = "input/day_9.txt".to_string();
    let reader = read_input(path);

    return reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|a| a.to_string().parse::<u8>().unwrap())
        .collect();
}
