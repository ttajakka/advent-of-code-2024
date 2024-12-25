use std::io::Read;

use crate::util::read_input;

pub fn puzzle1 () {
    let (locks, keys) = parse_input();

    let mut res = 0;
    
    for i in 0..locks.len() {
        let lock = &locks[i];
        for j in 0..keys.len() {
            let key = &keys[j];
            let mut fits = true;
            for k in 0..5 {
                if lock[k] + key[k] > 5 {
                    fits = false;
                    break;
                }
            }
            if fits {
                res += 1;
            }
        }
    }

    println!("day 25, puzzle 1: {res}");
    
    
}

fn parse_input() -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut reader = read_input("input/day_25.txt");
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();

    let mut locks = vec![];
    let mut keys = vec![];

    let parts = buf.split("\n\n");
    for p in parts {
        let mut lock = false;
        if p.starts_with("#") {
            lock = true;
        }

        let mut out = vec![0;5];
        for l in p.lines() {
            let l = l.to_string().chars().collect::<Vec<char>>();
            for i in 0..5 {
                if l[i] == '#' {
                    out[i] += 1;
                }
            }
        }
    for i in 0..5 {
        out[i] -= 1;
    }

        if lock {
            locks.push(out);
        } else {
            keys.push(out);
        }
    }

    (locks, keys)
}