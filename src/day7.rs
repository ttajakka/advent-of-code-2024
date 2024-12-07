use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Equation {
    result: i64,
    operands: Vec<i64>,
}

impl Equation {
    fn is_amenable(&self) -> bool {
        for i in 0..(2_u32.pow(self.operands.len() as u32 - 1)) {
            let operators = int_to_digis(i, self.operands.len() - 1 as usize);
            if self.check_operators(operators) {
                return true;
            }
        }
        false
    }

    fn check_operators(&self, operators: Vec<u8>) -> bool {
        let mut res = self.operands[0];
        for i in 0..operators.len() {
            let operator = operators[i];
            if operator == 0 {
                res += self.operands[i + 1];
            } else if operator == 1 {
                res *= self.operands[i + 1];
            }
        }
        if res == self.result {
            return true;
        }
        false
    }

    fn is_amenable_2(&self) -> bool {
        return check_operators_recursive(self.result, 0, self.operands.clone(), '+')
            || check_operators_recursive(self.result, 0, self.operands.clone(), '*')
            || check_operators_recursive(self.result, 0, self.operands.clone(), '|');
    }
}

fn check_operators_recursive(
    result: i64,
    mut current_value: i64,
    operands: Vec<i64>,
    operator: char,
) -> bool {
    if operands.len() == 0 {
        return result == current_value;
    }
    match operator {
        '+' => {
            current_value += operands[0];
        }
        '*' => {
            current_value *= operands[0];
        }
        '|' => {
            current_value = (current_value.to_string() + &operands[0].to_string()).parse().unwrap()
        }
        _ => panic!(),
    }
    return check_operators_recursive(result, current_value, operands[1..].to_vec(), '+')
        || check_operators_recursive(result, current_value, operands[1..].to_vec(), '*')
        || check_operators_recursive(result, current_value, operands[1..].to_vec(), '|');
}

fn int_to_digis(mut i: u32, len: usize) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    loop {
        out.push((i % 2) as u8);
        i = i / 2;
        if i == 0 {
            break;
        }
    }
    while out.len() < len {
        out.push(0)
    }
    out
}

pub fn puzzle1() {
    let equations = parse_input();

    let result: i64 = equations
        .iter()
        .filter(|a| a.is_amenable())
        .map(|a| a.result)
        .sum();

    println!("day 7, puzzle 1: {result}")
}

pub fn puzzle2() {
    let equations = parse_input();

    let result: i64 = equations
        .iter()
        .filter(|a| a.is_amenable_2())
        .map(|a| a.result)
        .sum();

    println!("day 7, puzzle 2: {result}")
}

fn parse_input() -> Vec<Equation> {
    let mut equations = vec![];
    for line in read_input().lines() {
        let line = line.unwrap();
        let mut parts = line.split(": ");
        let result = parts.next().unwrap().parse().unwrap();
        let operands = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|a| a.parse().unwrap())
            .collect();

        let eq = Equation { result, operands };

        equations.push(eq)
    }

    equations
}

fn read_input() -> BufReader<File> {
    let input = File::open("input/day_7.txt").unwrap();
    let reader = BufReader::new(input);

    reader
}
