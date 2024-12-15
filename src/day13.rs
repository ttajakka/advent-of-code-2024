use crate::util::read_input;
use regex::Regex;
use std::io::Read;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Claw {
    a: Point,
    b: Point,
    target: Point,
}

impl Claw {
    fn det(&self) -> i64 {
        self.a.x * self.b.y - self.a.y * self.b.x
    }

    fn sol_cand(&self) -> (i64, i64) {
        let cand_x = self.b.y * self.target.x - self.b.x * self.target.y;
        let cand_y = -self.a.y * self.target.x + self.a.x * self.target.y;
        (cand_x, cand_y)
    }

    fn has_solution(&self) -> bool {
        let det = self.det();
        let (cand_x, cand_y) = self.sol_cand();
        return (cand_x % det == 0) && (cand_y % det == 0);
    }

    fn solution(&self) -> Option<(i64, i64)> {
        if !self.has_solution() {
            return None;
        } else {
            let (cand_x, cand_y) = self.sol_cand();
            let solution = (cand_x / self.det(), cand_y / self.det());
            return Some(solution)
        }
    }

    fn cost(&self) -> Option<i64> {
        if !self.has_solution() {
            return None;
        } else {
            let solution = self.solution().unwrap();
            return Some(3 * solution.0 + solution.1);
        }
    }
}

pub fn puzzle1() {
    let claws = parse_input();

    let mut total_cost = 0;

    for claw in claws {
        if let Some(cost) = claw.cost() {
            total_cost += cost;
        }
    }

    println!("day 13, puzzle 1: {total_cost}");
    
}

pub fn puzzle2() {
    let mut claws = parse_input();

    for claw in &mut claws {
        claw.target.x += 10000000000000;
        claw.target.y += 10000000000000;
    }

    let mut total_cost = 0;

    for claw in claws {
        if let Some(cost) = claw.cost() {
            total_cost += cost;
        }
    }

    println!("day 13, puzzle 2: {total_cost}");
}

fn parse_claw(chunk: &str) -> Claw {
    let mut lines = chunk.lines();
    let line = lines.next().unwrap();

    let re = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let (_, [ax, ay]) = re.captures(line).unwrap().extract();

    let re = Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let (_, [bx, by]) = re.captures(lines.next().unwrap()).unwrap().extract();

    let re = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let (_, [x, y]) = re.captures(lines.next().unwrap()).unwrap().extract();

    Claw {
        a: Point {
            x: ax.parse().unwrap(),
            y: ay.parse().unwrap(),
        },
        b: Point {
            x: bx.parse().unwrap(),
            y: by.parse().unwrap(),
        },
        target: Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        },
    }
}

fn parse_input() -> Vec<Claw> {
    let mut reader = read_input("input/day_13.txt");
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    buf.split("\n\n").map(|chunk| parse_claw(chunk)).collect()
}
