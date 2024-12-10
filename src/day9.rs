use std::io::BufRead;

use crate::util::read_input;

pub fn puzzle1() {
    let input = parse_input();

    let mut expanded = expand(&input);

    fragment(&mut expanded);

    let checksum = calculate_checksum(&expanded);

    println!("day 9, puzzle 1: {checksum}");
}

#[derive(Debug, Clone)]
pub struct Filespace {
    id: u32,
    size: u32,
    free: u32,
}

pub fn puzzle2() {
    let input = parse_input();

    let mut files = input_to_filespaces(input);

    let mut j = files.len() - 1;
    let mut next_id = files[j].id;
    let mut was_moved = false;
    while j > 0 {
        if files[j].id > next_id {
            j -= 1;
            continue;
        }

        for i in 0..j {
            if files[i].free >= files[j].size {
                let moved = files[j].clone();
                files.remove(j);
                files[j - 1].free += moved.size + moved.free;
                files.insert(
                    i + 1,
                    Filespace {
                        id: moved.id,
                        size: moved.size,
                        free: files[i].free - moved.size,
                    },
                );
                files[i].free = 0;
                was_moved = true;
                break;
            }
        }

        if was_moved {
            was_moved = false;
        } else {
            j -= 1;
        }
        next_id -= 1;
    }

    println!("day 9, puzzle 2: {}", calculate_checksum_2(&files));
}

fn input_to_filespaces(input: Vec<u8>) -> Vec<Filespace> {
    let mut out = vec![];
    for i in 0..(input.len() - 1) / 2 {
        out.push(Filespace {
            id: i as u32,
            size: input[2 * i] as u32,
            free: input[2 * i + 1] as u32,
        })
    }
    out.push(Filespace {
        id: (input.len() as u32 - 1) / 2,
        size: input[input.len() - 1] as u32,
        free: 0,
    });
    out
}

fn calculate_checksum_2(files: &Vec<Filespace>) -> u64 {
    let mut checksum = 0_u64;
    let mut ind = 0_u64;
    for f in files {
        for _ in 0..f.size {
            checksum += ind * f.id as u64;
            ind += 1;
        }
        ind += f.free as u64;
    }
    checksum
}

pub fn display_filespaces(files: &Vec<Filespace>) {
    for f in files {
        for _ in 0..f.size {
            print!("{}", f.id);
        }
        for _ in 0..f.free {
            print!(".")
        }
    }
    println!("");
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
