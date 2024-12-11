use crate::util::read_input;
use std::io::BufRead;

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

// pub fn puzzle2() {
//     // let mut stones = parse_input();
//     let mut stones = vec![0]

//     for j in 0..75 {
//         let mut i: usize = 0;
//         while i < stones.len() {
//             if stones[i] == 0 {
//                 stones[i] = 1;
//             } else {
//                 let a = stones[i].to_string();
//                 if a.len() % 2 == 0 {
//                     let first = a[..a.len() / 2].parse::<u64>().unwrap();
//                     let second = a[a.len() / 2..].parse::<u64>().unwrap();
//                     stones[i] = first;
//                     stones.insert(i + 1, second);
//                     i += 1;
//                 } else {
//                     stones[i] *= 2024;
//                 }
//             }
//             i += 1;
//         }
//         println!("{j}: {}", stones.len());
//     }
//     println!("day 11, puzzle 2: {}", stones.len());
// }

fn parse_input() -> Vec<u64> {
    let reader = read_input("input/day_11.txt".to_string());
    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect()
}
