use crate::util::read_input;
use rand::Rng;
use std::io::BufRead;

// use std::thread::sleep;
// use std::time::Duration;

const MAX: u64 = std::u64::MAX;

enum Direction {
    Right,
    Straight,
    Left,
}

type Map = Vec<Vec<char>>;

struct Race {
    map: Vec<Vec<char>>,
    start: (i64, i64),
    end: (i64, i64),
    count: usize
}

pub fn puzzle1() {
    let mut race = parse_input();
    println!("{}", race.count);
    

    let mut res = MAX;
    // let mut dist_from_start = 0;

    for i in 0..10000000 {
        let (simulated, pos) = monte_carlo(&mut race);
        println!("{}", count_dots(&race.map));
        // if race.start.0 - pos.0 + race.start.1 - pos.1 > dist_from_start {
        //     dist_from_start = race.start.0 - pos.0 + race.start.1 - pos.1;
        //     println!("{pos:?}: {dist_from_start}");
            
        // } 
        println!("{pos:?}");
        if simulated < MAX {

            println!("{i}: {simulated}");
        }
        
        if simulated < res {
            res = simulated;
        }
    }

    println!("day 16, puzzle 1: {res}");
    
}

fn monte_carlo(race: &mut Race) -> (u64, (i64, i64)) {
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
            return (dist, pos)
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
    

    Race { map, start, end, count }
}
