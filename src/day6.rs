use std::fs::File;
use std::io::{BufRead, BufReader};

type Map = Vec<Vec<u8>>;
type Position = (usize, usize);
type Direction = (i32, i32);

pub fn puzzle1() {
    let mut map = read_input();

    let mut pos = find_initial_position(&map).unwrap();
    map[pos.0][pos.1] = 'X' as u8;

    let mut dir: Direction = (-1, 0);

    loop {
        let new = update_state(&mut map, pos, dir);
        if !new.0 {
            break;
        }
        pos = new.1;
        dir = new.2;
    }

    let mut result = 0;

    for row in map {
        for c in row {
            if c as char == 'X' {
                result += 1;
            }
        }
    }

    println!("day 6, puzzle 1: {result}");
}

pub fn puzzle2() {
    let init_map = read_input();
    let init_pos = find_initial_position(&init_map).unwrap();

    let mut initial_route: Vec<Position> = vec![];
    let mut map = init_map.clone();
    let mut pos = init_pos;
    let mut dir = (-1, 0);

    loop {
        let new = update_state(&mut map, pos, dir);
        if !new.0 {
            break;
        }
        if !initial_route.contains(&new.1) {
            initial_route.push(new.1)
        };
        pos = new.1;
        dir = new.2;
    }

    let mut result = 0;

    for pos in initial_route {
        let i = pos.0;
        let j = pos.1;
        if init_map[i][j] == '.' as u8 {
            let mut map = init_map.clone();
            map[i][j] = '#' as u8;
            if results_in_loop(map, init_pos) {
                println!("loop! {i} {j}");

                result += 1;
            }
        }
    }

    println!("day 6, puzzle 2: {result}");
}

fn results_in_loop(mut map: Map, init_pos: Position) -> bool {
    let mut pos = init_pos;
    let mut dir: Direction = (-1, 0);
    let mut history: Vec<(Position, Direction)> = vec![(pos, dir)];
    loop {
        let new = update_state(&mut map, pos, dir);
        if !new.0 {
            return false;
        }
        if history.contains(&(new.1, new.2)) {
            return true;
        }
        history.push((new.1, new.2));
        pos = new.1;
        dir = new.2;
    }
}

pub fn print_map(map: &Map) {
    for row in map {
        for c in row {
            print!("{}", *c as char);
        }
        println!()
    }
}

fn update_state(map: &mut Map, pos: Position, dir: Direction) -> (bool, Position, Direction) {
    map[pos.0][pos.1] = 'X' as u8;
    if moving_out(map, pos, dir) {
        return (false, pos, dir);
    } else if can_move(map, pos, dir) {
        return (true, next_pos(pos, dir), dir);
    } else {
        return (true, pos, rotate_clockwise(dir));
    }
}

fn moving_out(map: &Map, pos: Position, dir: Direction) -> bool {
    if pos.0 == 0 && dir.0 == -1 {
        return true;
    } else if pos.0 == map.len() - 1 && dir.0 == 1 {
        return true;
    } else if pos.1 == 0 && dir.1 == -1 {
        return true;
    } else if pos.1 == map[0].len() - 1 && dir.1 == 1 {
        return true;
    }

    return false;
}

fn can_move(map: &Map, pos: Position, dir: Direction) -> bool {
    let pos = next_pos(pos, dir);
    if map[pos.0][pos.1] == '#' as u8 {
        return false;
    }
    true
}

fn next_pos(pos: Position, dir: Direction) -> Position {
    (
        (pos.0 as i32 + dir.0) as usize,
        (pos.1 as i32 + dir.1) as usize,
    )
}

fn rotate_clockwise(dir: Direction) -> Direction {
    (dir.1, -dir.0)
}

fn find_initial_position(map: &Map) -> Option<(usize, usize)> {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '^' as u8 {
                return Some((i, j));
            }
        }
    }
    None
}

fn read_input() -> Map {
    let input = File::open("input/day_6.txt").unwrap();
    let reader = BufReader::new(input);

    let mut map: Map = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut bytes: Vec<u8> = vec![];
        for c in line.as_bytes() {
            bytes.push(*c);
        }
        map.push(bytes);
    }
    map
}
