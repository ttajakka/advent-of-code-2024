use regex::Regex;
use std::{collections::HashMap, io::Read};

use crate::util::read_input;

#[derive(Debug, Clone)]
enum Operation {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct Gate {
    input1: String,
    input2: String,
    operation: Operation,
    output: String,
}

pub fn puzzle1() {
    let (mut wires, mut gates) = parse_input();

    while gates.len() > 0 {
        let mut new_gates: Vec<Gate> = vec![];

        for i in 0..gates.len() {
            let gate = gates[i].clone();
            if !wires.contains_key(&gate.input1) || !wires.contains_key(&gate.input2) {
                new_gates.push(gate);
            } else {
                let a = *wires.get(&gate.input1).unwrap();
                let b = *wires.get(&gate.input2).unwrap();
                let value;
                match gate.operation {
                    Operation::AND => {value = a & b},
                    Operation::OR => {value = a | b},
                    Operation::XOR => {value = a ^ b},
                }
                wires.insert(gate.output, value);
            }
        }

        gates = new_gates;
    }

    let keys = wires.keys();
    let mut z_keys = vec![];
    for k in keys {
        if k.starts_with("z") {
            z_keys.push(k.to_string());
        }
    }
    z_keys.sort();

    let mut res = 0;
    for i in 0..z_keys.len() {
        let digit = *wires.get(&z_keys[i]).unwrap() as u64;
        res += digit * 2_u64.pow(i as u32);
    }

    println!("day 24, puzzle 1: {res}");
    
}

fn parse_input() -> (HashMap<String, u8>, Vec<Gate>) {
    let mut reader = read_input("input/day_24.txt");
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();

    let mut parts = buf.split("\n\n");

    let mut wires = HashMap::new();
    let wire_input = parts.next().unwrap();
    for l in wire_input.lines() {
        let mut p = l.split(": ");
        let name = p.next().unwrap().to_string();
        let value = p.next().unwrap().parse::<u8>().unwrap();
        wires.insert(name, value);
    }

    let mut gates = vec![];

    let re = Regex::new(r"([a-z,0-9]+) (AND|OR|XOR) ([a-z,0-9]+) -> ([a-z,0-9]+)").unwrap();
    let gate_input = parts.next().unwrap();
    for (_, [a, op, b, out]) in re.captures_iter(&gate_input).map(|c| c.extract()) {
        let operation;
        match op {
            "AND" => {operation = Operation::AND;},
            "OR" => {operation = Operation::OR;},
            "XOR" => {operation = Operation::XOR;},
            _ => panic!()
        }
        gates.push(Gate {
            input1: a.to_string(),
            input2: b.to_string(),
            operation: operation,
            output: out.to_string()
        })
    }

    (wires, gates)
}
