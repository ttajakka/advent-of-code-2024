use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

use crate::util::read_input;

type Map = Vec<Vec<char>>;
type Instructions = Vec<char>;

#[derive(Debug)]
struct Warehouse {
    map: Map,
    robot: (i32, i32),
    instructions: Instructions,
}

impl Warehouse {
    fn step(&mut self) {
        let inst = self.instructions.pop().unwrap();
        let move_dir;
        match inst {
            '^' => move_dir = (-1, 0),
            'v' => move_dir = (1, 0),
            '<' => move_dir = (0, -1),
            '>' => move_dir = (0, 1),
            _ => panic!(),
        }
        let (mut i, mut j) = (self.robot.0, self.robot.1);
        let mut boxes = 0;
        loop {
            i += move_dir.0;
            j += move_dir.1;
            if self.map[i as usize][j as usize] != 'O' {
                break;
            }
            boxes += 1;
        }
        if self.map[i as usize][j as usize] == '#' {
            return;
        }
        for _ in 0..boxes {
            self.map[i as usize][j as usize] = 'O';
            i -= move_dir.0;
            j -= move_dir.1;
        }
        self.map[i as usize][j as usize] = '@';
        self.map[self.robot.0 as usize][self.robot.1 as usize] = '.';
        self.robot = (self.robot.0 + move_dir.0, self.robot.1 + move_dir.1);
    }

    fn gps_sum(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j] == 'O' {
                    sum += 100 * i + j;
                }
            }
        }
        sum
    }
}

pub fn puzzle1() {
    let mut warehouse = parse_input();

    while warehouse.instructions.len() > 0 {
        warehouse.step();
        // print_map(&warehouse.map);
        // println!("");
        // println!("");

        // sleep(Duration::from_millis(80));
    }
    let res = warehouse.gps_sum();
    println!("day 15, puzzle 1: {res}");
}

pub fn print_map(map: &Map) {
    for row in map {
        for c in row {
            print!("{c}");
        }
        println!("");
    }
}

fn parse_map(map_input: &str) -> Map {
    let mut map = Vec::new();
    for line in map_input.lines() {
        let a: Vec<char> = line.chars().collect();
        map.push(a);
    }

    map
}

fn find_robot(map: &Map) -> (i32, i32) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '@' {
                return (i as i32, j as i32);
            }
        }
    }
    return (0, 0);
}

fn parse_input() -> Warehouse {
    let mut reader = read_input("input/day_15_mock.txt");
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();

    let mut parts = buf.split("\n\n");
    let map_input = parts.next().unwrap();

    let map = parse_map(map_input);
    let robot = find_robot(&map);

    let mut instructions = parts
        .next()
        .unwrap()
        .split("\n")
        .map(|l: &str| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
        .concat();
    instructions.reverse();

    Warehouse {
        map,
        robot,
        instructions,
    }
}

#[derive(Debug)]
struct BigWarehouse {
    height: usize,
    width: usize,
    map: Map,
    robot: (i32, i32),
    instructions: Instructions,
}

fn parse_dir(c: char) -> (i32, i32) {
    let dir;
    match c {
        '^' => dir = (-1, 0),
        'v' => dir = (1, 0),
        '<' => dir = (0, -1),
        '>' => dir = (0, 1),
        _ => panic!(),
    }
    dir
}

impl BigWarehouse {
    fn from_warehouse(wh: Warehouse) -> Self {
        let height = wh.map.len();
        let width = 2 * wh.map[0].len();

        let mut map = vec![vec!['.'; width]; height];
        let robot = (wh.robot.0, 2 * wh.robot.1);
        map[robot.0 as usize][robot.1 as usize];

        for i in 0..height {
            for j in 0..width / 2 {
                let sym = wh.map[i][j];
                if sym == '#' {
                    map[i][2 * j] = '#';
                    map[i][2 * j + 1] = '#';
                } else if sym == 'O' {
                    map[i][2 * j] = '[';
                    map[i][2 * j + 1] = ']';
                } else if sym == '@' {
                    map[i][2 * j] = '@';
                }
            }
        }

        BigWarehouse {
            height,
            width,
            map,
            robot: (wh.robot.0, 2 * wh.robot.1),
            instructions: wh.instructions,
        }
    }

    fn step(&mut self) {
        let instr = self.instructions.pop().unwrap();
        let dir = parse_dir(instr);
        let new_pos = (self.robot.0 + dir.0, self.robot.1 + dir.1);
        let sym = self.map[new_pos.0 as usize][new_pos.1 as usize];

        if sym == '#' {
            return;
        } else if sym == '.' {
            self.map[self.robot.0 as usize][self.robot.1 as usize] = '.';
            self.robot = (self.robot.0 + dir.0, self.robot.1 + dir.1);
            self.map[self.robot.0 as usize][self.robot.1 as usize] = '@';
            return;
        }

        let box_pos;
        match instr {
            '^' | 'v' => {
                if sym == '[' {
                    box_pos = new_pos
                } else {
                    box_pos = (new_pos.0, new_pos.1 - 1)
                }
            }
            '<' => box_pos = (new_pos.0, new_pos.1 - 1),
            '>' => box_pos = new_pos,
            _ => panic!(),
        }

        if self.can_move(box_pos, instr) {
            self.move_box(box_pos, instr);
            self.map[self.robot.0 as usize][self.robot.1 as usize] = '.';
            self.robot = (self.robot.0 + dir.0, self.robot.1 + dir.1);
            self.map[self.robot.0 as usize][self.robot.1 as usize] = '@';
        }
    }

    fn can_move(&self, pos: (i32, i32), instr: char) -> bool {
        match instr {
            '<' => {
                let sym = self.map[pos.0 as usize][pos.1 as usize - 1];
                if sym == '#' {
                    return false;
                } else if sym == '.' {
                    return true;
                } else {
                    return self.can_move((pos.0, pos.1 - 1), instr);
                }
            }
            '>' => {
                let sym = self.map[pos.0 as usize][pos.1 as usize + 2];
                if sym == '#' {
                    return false;
                } else if sym == '.' {
                    return true;
                } else {
                    return self.can_move((pos.0, pos.1 + 2), instr);
                }
            }
            '^' => {
                let sym1 = self.map[pos.0 as usize - 1][pos.1 as usize];
                let sym2 = self.map[pos.0 as usize - 1][pos.1 as usize + 1];
                if sym1 == '#' || sym2 == '#' {
                    return false;
                }

                if sym1 == '[' {
                    return self.can_move((pos.0 - 1, pos.1), instr);
                }

                let left;
                let right;

                if sym1 == ']' {
                    left = self.can_move((pos.0 - 1, pos.1 - 1), instr);
                } else {
                    left = true;
                }

                if sym2 == '[' {
                    right = self.can_move((pos.0 - 1, pos.1 + 1), instr);
                } else {
                    right = true;
                }

                return left && right;
            }
            'v' => {
                let sym1 = self.map[pos.0 as usize + 1][pos.1 as usize];
                let sym2 = self.map[pos.0 as usize + 1][pos.1 as usize + 1];
                if sym1 == '#' || sym2 == '#' {
                    return false;
                }

                if sym1 == '[' {
                    return self.can_move((pos.0 + 1, pos.1), instr);
                }

                let left;
                let right;

                if sym1 == ']' {
                    left = self.can_move((pos.0 + 1, pos.1 - 1), instr);
                } else {
                    left = true;
                }

                if sym2 == '[' {
                    right = self.can_move((pos.0 + 1, pos.1 + 1), instr);
                } else {
                    right = true;
                }

                return left && right;
            }
            _ => panic!(),
        }
    }

    fn move_box(&mut self, pos: (i32, i32), instr: char) {

    }

    fn gps_sum(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j] == '[' {
                    sum += 100 * i + j;
                }
            }
        }
        sum
    }
}

pub fn puzzle2() {
    let mut warehouse = BigWarehouse::from_warehouse(parse_input());
    warehouse.instructions = vec!['^', '>', '>', '>', '>'];
    print_map(&warehouse.map);

    while warehouse.instructions.len() > 0 {
        warehouse.step();
        print_map(&warehouse.map);
        println!("");
        println!("");
        sleep(Duration::from_millis(1000));
    }
    let res = warehouse.gps_sum();
    println!("day 15, puzzle 1: {res}");
}
