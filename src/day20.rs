use crate::util::read_input;
use core::panic;
use std::io::Read;

struct Track {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
}

pub fn puzzle1() {
    println!("day 20, puzzle 1: {}", puzzle_runner(2));
}

pub fn puzzle2() {
    println!("day 20, puzzle 2: {}", puzzle_runner(20));
}

fn puzzle_runner(max_cheat: i32) -> i32 {
    let track = parse_input();

    let (labeled, path) = label_track(&track);

    let mut res = 0;

    for pos in path {
        let current = labeled[pos.0][pos.1];
        for i in -max_cheat..max_cheat + 1 {
            for j in -(max_cheat - i.abs())..(max_cheat - i.abs()) + 1 {
                let test_pos = (pos.0 as i32 + i, pos.1 as i32 + j);

                if 0 <= test_pos.0
                    && test_pos.0 < track.height as i32
                    && 0 <= test_pos.1
                    && test_pos.1 < track.width as i32
                {
                    let test_pos = (test_pos.0 as usize, test_pos.1 as usize);
                    let diff =
                        labeled[test_pos.0][test_pos.1] as i32 - current as i32 - i.abs() - j.abs();
                    if diff >= 100 {
                        res += 1;
                    }
                }
            }
        }
    }
    res
}

fn label_track(track: &Track) -> (Vec<Vec<usize>>, Vec<(usize, usize)>) {
    let mut labeled_track: Vec<Vec<usize>> = vec![vec![0; track.width]; track.height];
    let map = &track.map;

    let mut pos = track.start;

    let mut count = 1;
    labeled_track[pos.0][pos.1] = count;
    let mut path = vec![pos];

    while pos != track.end {
        count += 1;
        if map[pos.0 + 1][pos.1] != '#' && labeled_track[pos.0 + 1][pos.1] == 0 {
            pos = (pos.0 + 1, pos.1);
        } else if map[pos.0 - 1][pos.1] != '#' && labeled_track[pos.0 - 1][pos.1] == 0 {
            pos = (pos.0 - 1, pos.1);
        } else if map[pos.0][pos.1 + 1] != '#' && labeled_track[pos.0][pos.1 + 1] == 0 {
            pos = (pos.0, pos.1 + 1);
        } else if map[pos.0][pos.1 - 1] != '#' && labeled_track[pos.0][pos.1 - 1] == 0 {
            pos = (pos.0, pos.1 - 1);
        } else {
            panic!()
        }

        labeled_track[pos.0][pos.1] = count;
        path.push(pos);
    }

    (labeled_track, path)
}

fn parse_input() -> Track {
    let mut reader = read_input("input/day_20.txt");
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();

    let map: Vec<Vec<char>> = buf.lines().map(|l| l.chars().collect()).collect();

    let height = map.len();
    let width = map[0].len();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..height {
        for j in 0..width {
            if map[i][j] == 'S' {
                start = (i, j);
            } else if map[i][j] == 'E' {
                end = (i, j)
            }
        }
    }

    Track {
        map,
        height,
        width,
        start,
        end,
    }
}
